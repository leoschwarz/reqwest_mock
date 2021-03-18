#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};

extern crate reqwest_mock;

use reqwest_mock::client::RocketClient;
use reqwest_mock::{Client, StatusCode};

#[get("/")]
fn index() -> &'static str {
    "Yes, this is index"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

#[test]
fn test_index() {
    let client = RocketClient::new(rocket(), "http://some.site".to_string());

    // Good request
    let response = client.get("http://some.site").send().unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body_to_utf8().unwrap(), "Yes, this is index");

    // Bad requests
    let response = client.get("http://some.site/path/whatever").send().unwrap();
    assert_eq!(response.status, StatusCode::NOT_FOUND);

    let response = client.get("http://bad.domain").send().unwrap();
    assert_eq!(response.status, StatusCode::NOT_FOUND);
}
