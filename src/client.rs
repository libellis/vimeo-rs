use url::Url;
use reqwest::{header, ClientBuilder, RequestBuilder};
use serde::Serialize;

pub(crate) const BASE_URL: &'static str = "https://api.vimeo.com/";
// 0.1 represents the crate version.
const USER_AGENT: &'static str = "vimeo-rs/0.1";
const MEDIA_TYPE_VERSION: &'static str = "application/vnd.vimeo.*+json;version=3.4";

const HEADER_RATE_LIMIT: &'static str = "X-RateLimit-Limit";
const HEADER_RATE_REMAINING: &'static str = "X-RateLimit-Remaining";
const HEADER_RATE_RESET: &'static str = "X-RateLimit-Reset";

pub struct Client {
    pub(crate) http_client: reqwest::Client,
    // TODO: Add Services per resource here.
}

impl Client {
    // TODO: Update this to return a Result type once we have library Error types.
    /// Constructs a new `Client` with the provided `access_token`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vimeo_rs::Client;
    ///
    /// let client = Client::new(env!("VIMEO_ACCESS_TOKEN"));
    /// ```
    pub fn new(access_token: &str) -> Client {
        let mut headers = header::HeaderMap::new();
        let header_value = format!("Bearer {}", access_token);
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&header_value).unwrap());
        headers.insert(header::ACCEPT, header::HeaderValue::from_str(MEDIA_TYPE_VERSION).unwrap());

        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(USER_AGENT)
            .build()
            .unwrap();

        Client {
            http_client,
        }
    }
}