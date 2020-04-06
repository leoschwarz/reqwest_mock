//! Some helper functions for the integration tests. (Mainly the test server.)
//!
//! Specification of the test server:
//! =================================
//! (square brackets are not printed, only included for clarity)
//!
//! Given a HTTP [METHOD] request of [URI] with [HEADERS] and [BODY],
//! the server will check if BODY is a plaintext integer number (regex [0-9]+).
//! If it is not, error 404 (bad request) will be returned, echoing the body.
//! If it is, the number will be incremented by one and the following output
//! will be generated:
//!
//! body (plaintext):
//! ```
//! [INCREMENTED NUMBER]\n
//! [METHOD] [URI]\n
//! [HEADERS]\n
//! ```

use futures::sync::oneshot;
use futures::{self, Future, Stream};
use hyper::server::{Http, Service};
use hyper::{self, Request, Response, StatusCode};
use regex::Regex;
use std::net::SocketAddr;
use std::thread;

struct TestServer;

impl Service for TestServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        // Extract the request headers.
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = req.headers().clone();

        // Extract the request body, note that it might consisd of multiple
        // chunks, which here should not be the case but has to be handled
        // anyway.
        let body = req.body().collect().and_then(|chunks| {
            // Extract body into a String.
            let mut bytes = Vec::new();
            for chunk in chunks {
                bytes.extend(chunk);
            }
            futures::future::ok(String::from_utf8(bytes).expect("Invalid encoding."))
        });

        // Determine the response.
        let response = body.and_then(move |body| {
            // Try extracting the number.
            let re = Regex::new(r"^(\d+)$").unwrap();
            let mut number: Option<u32> = None;
            if let Some(caps) = re.captures(body.as_str()) {
                if let Some(cap) = caps.get(1) {
                    number = Some(cap.as_str().parse().unwrap());
                }
            }

            // Create the response.
            let resp = if let Some(req_num) = number {
                let resp_num = req_num + 1;
                Response::new()
                    .with_body(format!("{}\n{} {}\n{:?}", resp_num, method, uri, headers))
            } else {
                Response::new()
                    .with_body(body)
                    .with_status(StatusCode::BadRequest)
            };
            futures::future::ok(resp)
        });
        Box::new(response)
    }
}

pub fn run_server(addr: SocketAddr) -> TestServerRunner {
    let (stop_tx, stop_rx) = oneshot::channel();

    thread::spawn(move || {
        let server = Http::new().bind(&addr, || Ok(TestServer)).unwrap();
        let stop_rx = stop_rx.map_err(|_| unreachable!());
        server.run_until(stop_rx).unwrap();
    });

    TestServerRunner {
        stop_server: stop_tx,
    }
}

/// This struct is used to destroy the server automatically after running a test.
pub struct TestServerRunner {
    stop_server: oneshot::Sender<()>,
}

impl TestServerRunner {
    pub fn terminate(self) {
        self.stop_server.send(()).unwrap();
    }
}

/*
#[test]
fn check_test_server() {
    let server = run_server("127.0.0.1:19241".parse().unwrap());
    let client = reqwest::Client::new();

    // Good request.
    let mut resp = client.get("http://127.0.0.1:19241/abc")
        .body("42")
        .send()
        .unwrap();
    assert_eq!(resp.status(), StatusCode::Ok);
    let lines: Vec<String> = resp.text().unwrap().lines().map(String::from).collect();
    assert_eq!(lines[0], "43");
    assert_eq!(lines[1], "GET /abc");

    // Bad request.
    let mut resp = client.get("http://127.0.0.1:19241/xyz")
        .body("pi")
        .send()
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BadRequest);
    assert_eq!(resp.text().unwrap(), "pi");

    server.terminate();
}
*/
