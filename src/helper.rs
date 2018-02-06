//! Defines some things used from different modules but not to be exported.

use reqwest::header::Headers;
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub fn serialize_headers(headers: &Headers) -> BTreeMap<String, String> {
    let tuples_iter = headers
        .iter()
        .map(|hv| (hv.name().to_string(), hv.value_string()));

    BTreeMap::<String, String>::from_iter(tuples_iter)
}

pub fn deserialize_headers(map: &BTreeMap<String, String>) -> Headers {
    let mut headers = ::reqwest::header::Headers::new();
    for (name, value) in map.iter() {
        headers.append_raw(name.clone(), value.as_bytes().to_vec())
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{ContentType, UserAgent};

    /// Just a basic example of one single header being serialized.
    #[test]
    fn serialize_headers() {
        let mut headers = Headers::new();
        headers.set(UserAgent::new("testing"));
        let serialized = super::serialize_headers(&headers);
        let mut expected = BTreeMap::new();
        expected.insert("User-Agent".to_string(), "testing".to_string());
        assert_eq!(serialized, expected);
    }

    /// Now a less trivial example checking whether the headers are being sorted,
    /// which is important for things like hashing of requests, which has to be
    /// deterministic regardless of the order headers were appended.
    #[test]
    fn serialize_headers_deterministic() {
        let mut headers1 = Headers::new();
        headers1.set(UserAgent::new("testing"));
        headers1.set(ContentType::png());
        let mut headers2 = Headers::new();
        headers2.set(ContentType::png());
        headers2.set(UserAgent::new("testing"));

        let ser1 = super::serialize_headers(&headers1);
        let ser2 = super::serialize_headers(&headers2);

        assert_eq!(ser1, ser2);
    }
}
