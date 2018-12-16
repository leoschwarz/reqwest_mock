use body::Body;
use client::Client;
use reqwest::{IntoUrl, Method, Url};
use request::{Request, RequestHeader};
use response::Response;
use reqwest::header::{IntoHeaderName, HeaderValue, HeaderMap};
use error::{Error, ResultExt};

pub struct RequestBuilder<'cl, Cl: Client + 'cl> {
    client: &'cl Cl,

    url: Result<Url, Error>,
    method: Method,
    headers: HeaderMap,
    body: Option<Body>,
}

impl<'cl, Cl: Client + 'cl> RequestBuilder<'cl, Cl> {
    #[doc(hidden)]
    pub fn new<U: IntoUrl>(client: &'cl Cl, url: U, method: Method) -> Self {
        RequestBuilder {
            client: client,
            url: url.into_url().chain_err(|| "invalid url"),
            method: method,
            headers: HeaderMap::new(),
            body: None,
        }
    }

    /// Add a header to the request.
    pub fn header<H: IntoHeaderName>(mut self, name: H, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Add multiple headers to the request.
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Set the body of the request.
    pub fn body<B: Into<Body>>(mut self, body: B) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Send the request.
    pub fn send(self) -> Result<Response, Error> {
        let request = Request {
            header: RequestHeader {
                url: self.url?,
                method: self.method,
                headers: self.headers,
            },
            body: self.body,
        };

        self.client.execute(None, request)
    }
}
