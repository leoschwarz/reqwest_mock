use reqwest::{Url, Method};
use reqwest::header::Headers;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, Unexpected};
use serde::de::Error as DeError;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub url: Url,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut req = serializer.serialize_struct("Request", 4)?;

        req.serialize_field("url", self.url.as_ref())?;
        req.serialize_field("method", self.method.as_ref())?;
        req.serialize_field("body", &self.body)?;
        req.serialize_field(
            "headers",
            &::helper::serialize_headers(&self.headers),
        )?;

        req.end()
    }
}

impl<'de> Deserialize<'de> for Request {
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
            type Value = Request;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Request")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Request, V::Error>
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

                Ok(Request {
                    url: url.ok_or_else(|| DeError::missing_field("url"))?,
                    method: method.ok_or_else(|| DeError::missing_field("method"))?,
                    body: body,
                    headers: headers.ok_or_else(|| DeError::missing_field("headers"))?,
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
        use reqwest::header::{ContentLength, UserAgent};

        let mut headers = Headers::new();
        headers.set(ContentLength(2000));
        headers.set(UserAgent("Testing Code".to_string()));

        let req1 = Request {
            url: Url::parse("https://example.com").unwrap(),
            method: Method::Get,
            body: Some(vec![2, 4, 11, 32, 99, 1, 4, 5]),
            headers: headers,
        };

        let json = ::serde_json::to_string(&req1).unwrap();
        let req2 = ::serde_json::from_str(json.as_ref()).unwrap();
        assert_eq!(req1, req2);
    }
}
