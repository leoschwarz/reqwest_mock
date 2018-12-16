//! Defines some things used from different modules but not to be exported.

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub fn serialize_headers(headers: &HeaderMap) -> BTreeMap<String, String> {
    let tuples_iter = headers
        .iter()
        .map(|(hn, hv)| (hn.to_string(), hv.to_str().unwrap().to_string()));

    BTreeMap::<String, String>::from_iter(tuples_iter)
}

pub fn deserialize_headers(map: &BTreeMap<String, String>) -> HeaderMap {
    let mut headers = ::reqwest::header::HeaderMap::new();
    for (name, value) in map.iter() {
        headers.insert(HeaderName::from_bytes(&name.clone().into_bytes()).unwrap(), HeaderValue::from_str(value).unwrap());
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{CONTENT_TYPE, USER_AGENT};

    /// Just a basic example of one single header being serialized.
    #[test]
    fn serialize_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "testing".parse().unwrap());
        let serialized = super::serialize_headers(&headers);
        let mut expected = BTreeMap::new();
        expected.insert("user-agent".to_string(), "testing".to_string());
        assert_eq!(serialized, expected);
    }

    /// Now a less trivial example checking whether the headers are being sorted,
    /// which is important for things like hashing of requests, which has to be
    /// deterministic regardless of the order headers were appended.
    #[test]
    fn serialize_headers_deterministic() {
        let mut headers1 = HeaderMap::new();
        headers1.insert(USER_AGENT, "testing".parse().unwrap());
        headers1.insert(CONTENT_TYPE, "image/png".parse().unwrap());
        let mut headers2 = HeaderMap::new();
        headers2.insert(CONTENT_TYPE, "image/png".parse().unwrap());
        headers2.insert(USER_AGENT, "testing".parse().unwrap());

        let ser1 = super::serialize_headers(&headers1);
        let ser2 = super::serialize_headers(&headers2);

        assert_eq!(ser1, ser2);
    }
}
