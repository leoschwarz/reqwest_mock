use body::Body;
use http::Request as HttpRequest;
use reqwest::{Method, Url};
use reqwest::header::HeaderMap;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::de::{Deserialize, Deserializer, MapAccess, Unexpected, Visitor};
use serde::de::Error as DeError;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct RequestHeader {
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
}

#[derive(Debug)]
pub struct Request {
    pub header: RequestHeader,
    pub body: Option<Body>,
}

impl Request {
    pub(crate) fn to_mem(self) -> Result<RequestMem, ::std::io::Error> {
        Ok(RequestMem {
            header: self.header,
            body: match self.body {
                Some(b) => Some(b.try_to_vec()?),
                None => None,
            },
        })
    }
}

impl<T> From<HttpRequest<T>> for Request where T: Into<Body> {
    fn from(r: HttpRequest<T>) -> Self {
        let header = RequestHeader {
            url: Url::parse(&format!("{}", r.uri())).unwrap(),
            method: r.method().clone(),
            headers: r.headers().clone(),
        };

        Request {
            header,
            body: Some(r.into_body().into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequestMem {
    pub header: RequestHeader,
    pub body: Option<Vec<u8>>,
}

impl From<RequestMem> for Request {
    fn from(r: RequestMem) -> Self {
        Request {
            header: r.header,
            body: r.body.map(Body::from),
        }
    }
}

/// We need this so we can generate unique filenames for each request.
impl Hash for RequestMem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.header.url.hash(state);
        self.header.method.hash(state);
        ::helper::serialize_headers(&self.header.headers).hash(state);
        self.body.hash(state);
    }
}

impl Serialize for RequestMem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut req = serializer.serialize_struct("Request", 4)?;

        req.serialize_field("url", self.header.url.as_ref())?;
        req.serialize_field("method", self.header.method.as_ref())?;
        req.serialize_field("body", &self.body)?;
        req.serialize_field(
            "headers",
            &::helper::serialize_headers(&self.header.headers),
        )?;

        req.end()
    }
}

impl<'de> Deserialize<'de> for RequestMem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Url,
            Method,
            Body,
            Headers,
        }

        struct RequestVisitor {}

        impl<'de> Visitor<'de> for RequestVisitor {
            type Value = RequestMem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Request")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RequestMem, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut url = None;
                let mut method = None;
                let mut body = None;
                let mut headers = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Url => {
                            if url.is_some() {
                                return Err(DeError::duplicate_field("url"));
                            }
                            let s: String = map.next_value()?;
                            url = Some(Url::parse(s.as_ref()).map_err(|_| {
                                DeError::invalid_value(Unexpected::Str(s.as_ref()), &"url")
                            })?);
                        }
                        Field::Method => {
                            if method.is_some() {
                                return Err(DeError::duplicate_field("method"));
                            }
                            let s: String = map.next_value()?;
                            method = Some(Method::from_str(s.as_ref()).map_err(|_| {
                                DeError::invalid_value(Unexpected::Str(s.as_ref()), &"method")
                            })?);
                        }
                        Field::Body => {
                            if body.is_some() {
                                return Err(DeError::duplicate_field("body"));
                            }
                            body = map.next_value()?;
                        }
                        Field::Headers => {
                            if headers.is_some() {
                                return Err(DeError::duplicate_field("headers"));
                            }
                            headers = Some(::helper::deserialize_headers(&map.next_value()?));
                        }
                    }
                }

                Ok(RequestMem {
                    header: RequestHeader {
                        url: url.ok_or_else(|| DeError::missing_field("url"))?,
                        method: method.ok_or_else(|| DeError::missing_field("method"))?,
                        headers: headers.ok_or_else(|| DeError::missing_field("headers"))?,
                    },
                    body: body,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["url", "method", "body"];
        deserializer.deserialize_struct("Request", FIELDS, RequestVisitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        use reqwest::header::{CONTENT_LENGTH, USER_AGENT};

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_LENGTH, 2000.into());
        headers.insert(USER_AGENT, "Testing Code".parse().unwrap());

        let req1 = RequestMem {
            header: RequestHeader {
                url: Url::parse("https://example.com").unwrap(),
                method: Method::GET,
                headers: headers,
            },
            body: Some(vec![2, 4, 11, 32, 99, 1, 4, 5]),
        };

        let json = ::serde_json::to_string(&req1).unwrap();
        let req2 = ::serde_json::from_str(json.as_ref()).unwrap();
        assert_eq!(req1, req2);
    }
}
