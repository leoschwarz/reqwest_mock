#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::PathBuf;

/// Defines some things used from different modules but not to be exported.
mod helper;

/// Defines the Error type we use in this library.
pub mod error;
//pub use error::*;

mod config;
pub use self::config::{RedirectPolicy, ClientConfig};

mod request;
mod response;

pub mod client;
mod request_builder;

/// Create a replay client instance using the specified file path as storage for
/// request and response data.
pub fn replay<P: Into<PathBuf>>(replay_file: P) -> client::ReplayClient {
    client::ReplayClient::new(replay_file)
}
