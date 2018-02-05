use client::{Client, Response};
use config::ClientConfig;
use error::Error;
use request::Request;
use std::io::Read;

/// Just a regular client performing no mocking at all.
///
/// The idea is that this one can be used in production code,
/// while another client is to be used in testing code.
pub struct DirectClient {
    config: ClientConfig,
}

impl DirectClient {
    pub fn new() -> Self {
        DirectClient { config: ClientConfig::default() }
    }
}

impl Client for DirectClient {
    fn execute(&self, config: Option<&ClientConfig>, request: Request) -> Result<Response, Error> {
        // Some information potentially useful for debugging.
        debug!(
            "DirectClient performing {} request of URL: {}",
            request.method,
            request.url
        );
        trace!("request headers: {}", request.headers);
        trace!("request body: {:?}", request.body);

        // Use internal config if none was provided together with the request.
        let config = config.unwrap_or_else(|| &self.config);

        // Setup the client instance.
        let mut client_builder = ::reqwest::Client::builder();
        client_builder.gzip(config.gzip);
        client_builder.redirect(config.redirect.clone().into());
        client_builder.referer(config.referer);
        if let Some(timeout) = config.timeout.clone() {
            client_builder.timeout(timeout);
        }
        let client = client_builder.build()?;

        // Build the request.
        let mut builder = client.request(request.method, request.url);
        if let Some(body) = request.body {
            builder.body(body);
        }

        // Send the request.
        let mut response = builder.send()?;

        // Extract the response.
        Ok(Response {
            url: response.url().clone(),
            status: response.status().clone(),
            headers: response.headers().clone(),
            body: {
                let mut buf = Vec::<u8>::new();
                response.read_to_end(&mut buf)?;
                buf
            },
        })
    }

    fn config(&self) -> &ClientConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut ClientConfig {
        &mut self.config
    }
}
