//! Defines some things used from different modules but not to be exported.

use reqwest::header::Headers;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn serialize_headers(headers: &Headers) -> HashMap<String, String> {
    let tuples_iter = headers.iter().map(|hv| {
        (hv.name().to_string(), hv.value_string())
    });

    HashMap::<String, String>::from_iter(tuples_iter)
}

pub fn deserialize_headers(map: &HashMap<String, String>) -> Headers {
    let mut headers = ::reqwest::header::Headers::new();
    for (name, value) in map.iter() {
        headers.append_raw(name.clone(), value.as_bytes().to_vec())
    }

    headers
}
