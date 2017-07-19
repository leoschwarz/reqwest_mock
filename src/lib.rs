//! Provides a mockable reqwest-like HTTP client.
//!
//! Write your code generic over the [Client](trait.Client.html) trait,
//! and in production use [DirectClient](struct.DirectClient.html) while in testing
//! you can use [ReplayClient](struct.ReplayClient.html), which will record a request
//! the first time and replay it every time the exact same request is made in the
//! future.
//!
//! # Examples
//!
//! ```
//! use reqwest_mock::{Client, DirectClient, ReplayClient, Error};
//! use reqwest_mock::header::UserAgent;
//!
//! struct MyClient<C: Client> {
//!     client: C,
//! }
//!
//! fn new_client() -> MyClient<DirectClient> {
//!     MyClient {
//!         client: DirectClient::new()
//!     }
//! }
//!
//! #[cfg(test)]
//! fn test_client(path: &str) -> MyClient<ReplayClient> {
//!     MyClient {
//!         client: ReplayClient::new(path)
//!     }
//! }
//!
//! impl<C: Client> MyClient<C> {
//!     /// For simplicity's sake we are not parsing the response but just extracting the
//!     /// response body.
//!     /// Also in your own code it might be a good idea to define your own `Error` type.
//!     pub fn get_time(&self) -> Result<String, Error> {
//!         let response = self.client
//!             .get("https://now.httpbin.org/")
//!             .header(UserAgent::new("MyClient"))
//!             .send()?;
//!
//!         response.body_to_utf8()
//!     }
//! }
//! ```

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate twox_hash;

mod helper;

pub mod error;
pub mod config;

mod body;
pub use body::IntoBody;

mod request;
mod response;

pub mod client;
mod request_builder;

pub use self::client::*;
pub use self::error::Error;

pub use reqwest::{header, Url, UrlError, IntoUrl, Method, StatusCode};
