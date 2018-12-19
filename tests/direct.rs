//! Test the `DirectClient`.

extern crate futures;
extern crate hyper;
extern crate regex;
extern crate reqwest;
extern crate reqwest_mock;
mod helper;

use reqwest_mock::{Client, StatusCode};
use reqwest_mock::client::DirectClient;

#[test]
fn direct_client() {
    let server = helper::run_server("127.0.0.1:19241".parse().unwrap());

    // Good request.
    let client = DirectClient::new();
    let resp = client
        .get("http://127.0.0.1:19241/abc")
        .body("42")
        .send()
        .unwrap();
    assert_eq!(resp.status, StatusCode::OK);
    let lines: Vec<String> = resp.body_to_utf8()
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(lines[0], "43");
    assert_eq!(lines[1], "GET /abc");

    // Bad request.
    let resp = client
        .get("http://127.0.0.1:19241/xyz")
        .body("pi")
        .send()
        .unwrap();
    assert_eq!(resp.status, StatusCode::BAD_REQUEST);
    assert_eq!(resp.body_to_utf8().unwrap(), "pi");

    server.terminate();
}
