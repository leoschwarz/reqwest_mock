use client::{Client, Response};
use config::ClientConfig;
use error::Error;
use request::Request;
use reqwest::header::HeaderMap;
use reqwest::{StatusCode, Url};
use rocket;
use rocket::http::Method;
use rocket::http::{ContentType, Status};
use std::io::Read;

pub struct RocketClient {
    config: ClientConfig,
    base_url: String,
    test_client: rocket::local::Client,
}

impl RocketClient {
    pub fn new(app: rocket::Rocket, base_url: String) -> Self {
        RocketClient {
            config: ClientConfig::default(),
            base_url,
            test_client: rocket::local::Client::new(app).unwrap(),
        }
    }
}

fn reqwest_method_to_rocket_method(method: reqwest::Method) -> Method {
    match method {
        reqwest::Method::GET => Method::Get,
        reqwest::Method::PUT => Method::Put,
        reqwest::Method::POST => Method::Post,
        reqwest::Method::DELETE => Method::Delete,
        reqwest::Method::OPTIONS => Method::Options,
        reqwest::Method::HEAD => Method::Head,
        reqwest::Method::TRACE => Method::Trace,
        reqwest::Method::CONNECT => Method::Connect,
        reqwest::Method::PATCH => Method::Patch,
        _ => panic!(),
    }
}

impl Client for RocketClient {
    fn execute(&self, config: Option<&ClientConfig>, request: Request) -> Result<Response, Error> {
        // Use internal config if none was provided together with the request.
        // TODO: use the config
        let config = config.unwrap_or_else(|| &self.config);
        let path = match request.header.url.as_str().strip_prefix(&self.base_url) {
            Some(path) => path,
            None => {
                return Ok(Response {
                    url: request.header.url,
                    status: StatusCode::NOT_FOUND,
                    headers: HeaderMap::new(),
                    body: vec![],
                })
            }
        };
        let mut local_response = self
            .test_client
            .req(reqwest_method_to_rocket_method(request.header.method), path)
            .dispatch();
        let status = match local_response.status() {
            Status::Ok => StatusCode::OK,
            Status::NotFound => StatusCode::NOT_FOUND,
            _ => panic!(),
        };
        Ok(Response {
            url: request.header.url.clone(),
            status: status,
            headers: HeaderMap::new(),
            body: local_response.body_bytes().unwrap_or(vec![]),
        })
    }

    fn config(&self) -> &ClientConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut ClientConfig {
        &mut self.config
    }
}
