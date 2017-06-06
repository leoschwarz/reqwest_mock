use config::ClientConfig;
use error::Error;
use request::Request;
use request_builder::RequestBuilder;
use reqwest::{Method, IntoUrl};
use response::Response;

pub trait Client: Sized {
    /// Execute a request.
    /// If config is None the client is to use the internal config otherwise it is to use the
    /// provided config here.
    fn execute(&self, config: Option<&ClientConfig>, request: Request) -> Result<Response, Error>;
    fn config(&self) -> &ClientConfig;

    fn get<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Get, url)
    }

    fn post<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Post, url)
    }

    fn put<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Put, url)
    }

    fn patch<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Patch, url)
    }

    fn delete<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Delete, url)
    }

    fn head<'cl, U: IntoUrl>(&'cl self, url: U) -> RequestBuilder<'cl, Self> {
        self.request(Method::Head, url)
    }

    fn request<'cl, U: IntoUrl>(&'cl self, method: Method, url: U) -> RequestBuilder<'cl, Self> {
        RequestBuilder::new(self, url, method)
    }
}

mod direct;
pub use self::direct::DirectClient;

mod replay;
pub use self::replay::ReplayClient;
