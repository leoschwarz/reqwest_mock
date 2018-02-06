use base64;
use error::Error;
use reqwest::header::Headers;
use reqwest::{StatusCode, Url};
use serde::de::Error as DeError;
use serde::de::{Deserialize, Deserializer, MapAccess, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    /// The final url of this response.
    pub url: Url,

    /// Status code.
    pub status: StatusCode,

    /// Headers
    pub headers: Headers,

    /// The response body in binary format.
    pub body: Vec<u8>,
}

impl Response {
    pub fn body_to_utf8(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.body.clone())?)
    }
}

const N_RESPONSE: &'static str = "Response";
const F_URL: &'static str = "url";
const F_STATUS: &'static str = "status";
const F_HEADERS: &'static str = "headers";
const F_BODY: &'static str = "body";

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut res = serializer.serialize_struct(N_RESPONSE, 5)?;

        res.serialize_field(F_URL, self.url.as_ref())?;
        // TODO: actually the docs for this are hidden
        res.serialize_field(F_STATUS, &u16::from(self.status.clone()))?;
        res.serialize_field(F_HEADERS, &::helper::serialize_headers(&self.headers))?;
        res.serialize_field(F_BODY, &base64::encode(&self.body))?;

        res.end()
    }
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    Url,
    Status,
    Headers,
    Body,
}

struct ResponseVisitor {}

impl<'de> Visitor<'de> for ResponseVisitor {
    type Value = Response;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Response")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Response, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut url = None;
        let mut status = None;
        let mut headers = None;
        let mut body = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Url => {
                    if url.is_some() {
                        return Err(DeError::duplicate_field(F_URL));
                    }
                    let s: String = map.next_value()?;
                    url = Some(Url::parse(s.as_ref())
                        .map_err(|_| DeError::invalid_value(Unexpected::Str(s.as_ref()), &F_URL))?);
                }
                Field::Status => {
                    if status.is_some() {
                        return Err(DeError::duplicate_field(F_STATUS));
                    }
                    let s: u16 = map.next_value()?;
                    status = Some(StatusCode::try_from(s).map_err(|_| {
                        DeError::invalid_value(Unexpected::Unsigned(s as u64), &"StatusCode")
                    })?);
                }
                Field::Headers => {
                    if headers.is_some() {
                        return Err(DeError::duplicate_field(F_HEADERS));
                    }
                    headers = Some(::helper::deserialize_headers(&map.next_value()?));
                }
                Field::Body => {
                    if body.is_some() {
                        return Err(DeError::duplicate_field(F_BODY));
                    }
                    let s: String = map.next_value()?;
                    body = Some(base64::decode(&s).map_err(|_| {
                        DeError::invalid_value(Unexpected::Str(s.as_ref()), &F_BODY)
                    })?);
                }
            }
        }

        Ok(Response {
            url: url.ok_or_else(|| DeError::missing_field(F_URL))?,
            status: status.ok_or_else(|| DeError::missing_field(F_STATUS))?,
            headers: headers.ok_or_else(|| DeError::missing_field(F_HEADERS))?,
            body: body.ok_or_else(|| DeError::missing_field(F_BODY))?,
        })
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &[F_URL, F_STATUS, F_HEADERS, F_BODY];
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
        headers.set(UserAgent::new("Testing Code"));

        let resp1 = Response {
            url: Url::parse("http://example.com/index.html").unwrap(),
            status: StatusCode::Ok,
            headers: headers,
            body: vec![2, 4, 8, 16, 32, 64, 42],
        };

        let json = ::serde_json::to_string(&resp1).unwrap();

        let resp2 = ::serde_json::from_str(json.as_ref()).unwrap();
        assert_eq!(resp1, resp2);
    }
}
