use reqwest::{header, ClientBuilder};

const BASE_URL: &'static str = "https://api.vimeo.com/";
// 0.1 represents the crate version.
const USER_AGENT: &'static str = "vimeo-rs/0.1";
const MEDIA_TYPE_VERSION: &'static str = "application/vnd.vimeo.*+json;version=3.4";

const HEADER_RATE_LIMIT: &'static str = "X-RateLimit-Limit";
const HEADER_RATE_REMAINING: &'static str = "X-RateLimit-Remaining";
const HEADER_RATE_RESET: &'static str = "X-RateLimit-Reset";

// Client is a simple wrapper around an http client that has auth headers and all necessary
// default headers attached for all future requests related to interfacing with Vimeo's API.
pub struct Client {
    http_client: reqwest::Client,
    base_url: &'static str,
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
            base_url: BASE_URL,
        }
    }

    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    pub fn base_url(&self) -> &'static str {
        self.base_url
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Client {
            // This is cheap to clone because under the hood it's just an Arc<ClientRef>
            http_client: self.http_client.clone(),
            base_url: self.base_url,
        }
    }
}