#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::Path;

/// Defines the Error type we use in this library.
pub mod error;
//pub use error::*;

mod config;
pub use self::config::{RedirectPolicy, ClientConfig};

mod request;
mod response;

pub mod client;

/// Create a replay client instance using the specified file path as storage for
/// request and response data.
pub fn replay<P: AsRef<Path>>(replay_file: P) {}
