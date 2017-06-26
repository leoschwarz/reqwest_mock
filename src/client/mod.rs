//! Defines the main types to be used to mock the HTTP client.

use config::ClientConfig;
use error::Error;
use request::Request;
use request_builder::RequestBuilder;
use reqwest::{Method, IntoUrl};
use response::Response;

/// Provides a unified interface over the different Clients.
///
/// Write your code generic over this trait and for example in testing you can use a different
/// client than in your normal library's code.
pub trait Client: Sized {
    /// Execute a request.
    ///
    /// If config is `None` the client is to use the internal config otherwise it is to use the
    /// provided config here.
    fn execute(&self, config: Option<&ClientConfig>, request: Request) -> Result<Response, Error>;

    /// Returns a immutable reference to the internal config.
    fn config(&self) -> &ClientConfig;

    /// Returns a mutable reference to the internal config.
    fn config_mut(&mut self) -> &mut ClientConfig;

    ////////////////////////////////////////////////////////

    /// Convenience method to make a `GET` request to a URL.
    fn get<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Get, url)
    }

    /// Convenience method to make a `POST` request to a URL.
    fn post<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Post, url)
    }

    /// Convenience method to make a `PUT` request to a URL.
    fn put<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Put, url)
    }

    /// Convenience method to make a `PATCH` request to a URL.
    fn patch<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Patch, url)
    }

    /// Convenience method to make a `DELETE` request to a URL.
    fn delete<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Delete, url)
    }

    /// Convenience method to make a `HEAD` request to a URL.
    fn head<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Head, url)
    }

    /// Returns a `RequestBuilder` for the given method and URL, which allows for further
    /// configuration of the request, like including additional headers, and sending it.
    fn request<'cl, U: IntoUrl>(&'cl self, method: Method, url: U) -> RequestBuilder<'cl, Self> {
        RequestBuilder::new(self, url, method)
    }
}

mod direct;
pub use self::direct::DirectClient;

mod replay;
pub use self::replay::ReplayClient;

//mod stub;
//pub use self::stub::StubClient;

mod generic;
pub use self::generic::GenericClient;
