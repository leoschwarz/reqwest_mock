extern crate reqwest_mock;

use reqwest_mock::client::*;

const URL: &'static str = "https://httpbin.org/uuid";
const URL2: &'static str = "https://httpbin.org/uuid#";

fn perform_request<C: Client>(client: &C, url: &str) -> String {
    // This method is just a placeholder for some fancy computations.
    let response = client.get(url).send().unwrap();
    response.body_to_utf8().unwrap()
}

fn main() {
    let client1 = DirectClient::new();
    let resp1 = perform_request(&client1, URL);
    let resp2 = perform_request(&client1, URL);
    // httpbin should return a different UUID for every actual request
    assert_ne!(resp1, resp2);

    let client2 = ReplayClient::builder().target_file("simple.replay").build().unwrap();
    let resp1 = perform_request(&client2, URL);
    let resp2 = perform_request(&client2, URL);
    assert_eq!(resp1, resp2);

    let resp3 = perform_request(&client2, URL2);
    assert_ne!(resp1, resp3);

    let resp4 = perform_request(&client2, URL2);
    assert_eq!(resp3, resp4);

    println!("Tests finished.");
}
