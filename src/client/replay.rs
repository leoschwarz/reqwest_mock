use client::{Client, Response};
use config::ClientConfig;
use error::Error;
use request::Request;
use std::path::PathBuf;

pub struct ReplayClient {
    replay_file: PathBuf,
}

impl ReplayClient {
    pub fn new<P: Into<PathBuf>>(replay_file: P) -> Self {
        ReplayClient { replay_file: replay_file.into() }
    }
}

impl Client for ReplayClient {
    fn execute(&self, config: &ClientConfig, request: Request) -> Result<Response, Error> {
        // Check if the request was already performed with this exact arguments,
        // if it was just return the existing result otherwise perform the request and store
        // the output.


        unimplemented!()
    }
}

/// The data stored inside of a replay file.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReplayData {
    request: Request,
}
