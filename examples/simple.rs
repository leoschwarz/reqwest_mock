extern crate reqwest_mock;

use reqwest_mock::client::*;

const URL: &'static str = "https://now.httpbin.org/";
const URL2: &'static str = "https://now.httpbin.org/#";

fn perform_request<C: Client>(client: &C, url: &str) -> String {
    // This method is just a placeholder for some fancy computations.
    let response = client.get(url).send().unwrap();
    response.body_to_utf8().unwrap()
}

fn main() {
    let c1 = DirectClient::new();
    let r1 = perform_request(&c1, URL);
    let r2 = perform_request(&c1, URL);
    // There was a delay between the two requests, so httpbin should have returned two different
    // times.
    assert!(r1 != r2);

    let c2 = ReplayClient::new("simple.replay");
    let r1 = perform_request(&c2, URL);
    let r2 = perform_request(&c2, URL);
    assert_eq!(r1, r2);

    let r3 = perform_request(&c2, URL2);
    assert!(r1 != r3);

    let r4 = perform_request(&c2, URL2);
    assert_eq!(r3, r4);

    println!("Tests finished.");
}
