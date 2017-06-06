use reqwest::header::Headers;
use reqwest::{Url, StatusCode, HttpVersion};
use serde::de::Error as DeError;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, Unexpected};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    /// The final url of this response.
    pub url: Url,

    /// Status code.
    pub status: StatusCode,

    /// Headers
    pub headers: Headers,

    /// HTTP Version
    pub version: HttpVersion,

    /// The response body in binary format.
    pub body: Vec<u8>,
}

const N_RESPONSE: &'static str = "Response";
const F_URL: &'static str = "url";
const F_STATUS: &'static str = "status";
const F_HEADERS: &'static str = "headers";
const F_VERSION: &'static str = "version";
const F_BODY: &'static str = "body";

fn serialize_headers(headers: &Headers) -> HashMap<String, String> {
    let tuples_iter = headers
        .iter()
        .map(|hv| (hv.name().to_string(), hv.value_string()));

    HashMap::<String, String>::from_iter(tuples_iter)
}

fn deserialize_headers(map: &HashMap<String, String>) -> Headers {
    let mut headers = ::reqwest::header::Headers::new();
    for (name, value) in map.iter() {
        headers.append_raw(name.clone(), value.as_bytes().to_vec())
    }

    headers
}

// TODO remove once my commit lands on crates.io
fn serialize_http_version(v: &HttpVersion) -> String {
    match *v {
        HttpVersion::Http09 => "HTTP/0.9".to_string(),
        HttpVersion::Http10 => "HTTP/1.0".to_string(),
        HttpVersion::Http11 => "HTTP/1.1".to_string(),
        HttpVersion::Http20 => "HTTP/2.0".to_string(),
    }
}

// TODO remove once my commit lands on crates.io
fn deserialize_http_version(v: &str) -> Result<HttpVersion, ()> {
    match v {
        "HTTP/0.9" => Ok(HttpVersion::Http09),
        "HTTP/1.0" => Ok(HttpVersion::Http10),
        "HTTP/1.1" => Ok(HttpVersion::Http11),
        "HTTP/2.0" => Ok(HttpVersion::Http20),
        _ => Err(()),
    }
}

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut res = serializer.serialize_struct(N_RESPONSE, 5)?;

        res.serialize_field(F_URL, self.url.as_ref())?;
        // TODO: actually the docs for this are hidden
        res.serialize_field(F_STATUS, &self.status.to_u16())?;
        res.serialize_field(F_HEADERS, &serialize_headers(&self.headers))?;
        res.serialize_field(F_VERSION, &serialize_http_version(&self.version))?;
        res.serialize_field(F_BODY, &self.body)?;

        res.end()
    }
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    Url,
    Status,
    Headers,
    Version,
    Body,
}


struct ResponseVisitor {}

impl<'de> Visitor<'de> for ResponseVisitor {
    type Value = Response;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Response")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Response, V::Error>
        where V: MapAccess<'de>
    {
        let mut url = None;
        let mut status = None;
        let mut headers = None;
        let mut version = None;
        let mut body = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Url => {
                    if url.is_some() {
                        return Err(DeError::duplicate_field(F_URL));
                    }
                    let s: &str = map.next_value()?;
                    url = Some(Url::parse(s)
                                   .map_err(|_| {
                                                DeError::invalid_value(Unexpected::Str(s), &F_URL)
                                            })?);
                }
                Field::Status => {
                    if status.is_some() {
                        return Err(DeError::duplicate_field(F_STATUS));
                    }
                    let s: u16 = map.next_value()?;
                    status = Some(StatusCode::from_u16(s));
                }
                Field::Headers => {
                    if headers.is_some() {
                        return Err(DeError::duplicate_field(F_HEADERS));
                    }
                    headers = Some(deserialize_headers(&map.next_value()?));
                }
                Field::Version => {
                    if version.is_some() {
                        return Err(DeError::duplicate_field(F_VERSION));
                    }
                    let s: &str = map.next_value()?;
                    version = Some(deserialize_http_version(s)
                                       .map_err(|_| {
                                                    DeError::invalid_value(Unexpected::Str(s),
                                                                           &F_VERSION)
                                                })?);
                }
                Field::Body => {
                    if body.is_some() {
                        return Err(DeError::duplicate_field(F_BODY));
                    }
                    body = Some(map.next_value()?);
                }
            }
        }

        Ok(Response {
               url: url.ok_or_else(|| DeError::missing_field(F_URL))?,
               status: status.ok_or_else(|| DeError::missing_field(F_STATUS))?,
               headers: headers.ok_or_else(|| DeError::missing_field(F_HEADERS))?,
               version: version.ok_or_else(|| DeError::missing_field(F_VERSION))?,
               body: body.ok_or_else(|| DeError::missing_field(F_BODY))?,
           })
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        const FIELDS: &'static [&'static str] = &[F_URL, F_STATUS, F_HEADERS, F_VERSION, F_BODY];
        deserializer.deserialize_struct(N_RESPONSE, FIELDS, ResponseVisitor {})
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

        let resp1 = Response {
            url: Url::parse("http://example.com/index.html").unwrap(),
            status: StatusCode::Ok,
            headers: headers,
            version: HttpVersion::Http11,
            body: vec![2, 4, 8, 16, 32, 64, 42],
        };

        let json = ::serde_json::to_string(&resp1).unwrap();

        let resp2 = ::serde_json::from_str(json.as_ref()).unwrap();
        assert_eq!(resp1, resp2);
    }
}
