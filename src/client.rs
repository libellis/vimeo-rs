use std::env;
use reqwest::{header, ClientBuilder};

pub struct Client {
    http_client: reqwest::Client

    // TODO: Add Services per resource here.
}

impl Client {
    pub fn new(access_token: &String) -> Client {
        let mut headers = header::HeaderMap::new();
        let header_value = format!("Bearer {}", access_token);
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&header_value).unwrap());

        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Client {
            http_client,
        }
    }
}