use client::Client;
use reqwest::{IntoUrl, Url, Method};
use request::Request;
use response::Response;
use reqwest::header::{Headers, Header, HeaderFormat};
use error::{Error, ResultExt};

pub struct RequestBuilder<'cl, Cl: Client + 'cl> {
    client: &'cl Cl,

    url: Result<Url, Error>,
    method: Method,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl<'cl, Cl: Client + 'cl> RequestBuilder<'cl, Cl> {
    pub fn new<U: IntoUrl>(client: &'cl Cl, url: U, method: Method) -> Self {
        RequestBuilder {
            client: client,
            url: url.into_url().chain_err(||"invalid url"),
            method: method,
            headers: Headers::new(),
            body: None
        }
    }

    pub fn header<H: Header + HeaderFormat>(mut self, header: H) -> Self {
        self.headers.set(header);
        self
    }

    pub fn headers(mut self, headers: Headers) -> Self {
        self.headers.extend(headers.iter());
        self
    }

    // TODO create new trait and conversions like reqwest::Body
    //      (note: we cannot use reqwest::Body since it doesn't provide a way to get bytes out of
    //      it from outside the crate.)
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn send(self) -> Result<Response, Error> {
        let request = Request {
            url: self.url?,
            method: self.method,
            headers: self.headers,
            body: self.body
        };

        self.client.execute(None, request)
    }
}
