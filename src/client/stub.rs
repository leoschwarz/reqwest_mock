// This is some unfinished WIP.
//
// Missing things:
// - How will the client handle request bodies, we will probably want to provide options to be
//   strict about it or relaxed.
// - Actually this is more of a sketch and still sort of broken.

use client::Client;
use config::ClientConfig;
use error::Error;
use request::Request;
use reqwest::{Method, Url};
use std::collections::HashMap;
use response::Response;

/// A client which allows you to stub out the response to a request explicitely.
pub struct StubClient {
    config: ClientConfig,
    stubs: HashMap<StubKey, Response>,
    settings: StubSettings
}

/// Some settings for the `StubClient`.
#[derive(Clone, Debug)]
pub struct StubSettings {
    /// If true instead of just making requests to resources which weren't stubbed,
    /// when trying to execute any such requests the client will just return a failure.
    fail_on_unstubbed: bool,
}

#[derive(Hash, PartialEq, Eq)]
struct StubKey {
    method: Method,
    url: Url,
}

impl Client for StubClient {
    fn execute(&self, config: Option<&ClientConfig>, request: Request) -> Result<Response, Error> {
        // Check if there is a recorded stub for the request.
    }

    fn config(&self) -> &ClientConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut ClientConfig {
        &mut self.config
    }
}

impl StubClient {
    pub fn new(stub_settings: StubSettings) -> Self {
        StubClient {
            config: ClientConfig::default(),
            stubs: HashMap::new(),
            settings: stub_settings,
        }
    }

}

/// Allows to stub out a response using a builder API.
pub struct ResponseStubber<'cl> {
    method: Method,
    url: Url,
    client: &'cl StubClient,
}

impl<'cl> ResponseStubber<'cl> {
    fn to_stub(self) -> Stub {
        Stub {
            method: self.method,
            url: self.url,
        }
    }
}
