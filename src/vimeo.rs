use crate::{UserService, VideoService};
use reqwest::{header, ClientBuilder};

// 0.1 represents the crate version.
const USER_AGENT: &'static str = "vimeo-rs/0.1";
const MEDIA_TYPE_VERSION: &'static str = "application/vnd.vimeo.*+json;version=3.4";

// Reference types used for video upload activities. Might use later.
// const HEADER_RATE_LIMIT: &'static str = "X-RateLimit-Limit";
// const HEADER_RATE_REMAINING: &'static str = "X-RateLimit-Remaining";
// const HEADER_RATE_RESET: &'static str = "X-RateLimit-Reset";


// VimeoServices holds all Vimeo API services.
// A service is related to a specific API resource you would like to interact with.
pub struct Client {
    _http_client: reqwest::Client,

    users:  UserService,
    videos: VideoService,
}

impl Client {
    pub fn new(access_token: String) -> Client {
        let mut headers = header::HeaderMap::new();
        let header_value = format!("Bearer {}", access_token);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&header_value).unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_str(MEDIA_TYPE_VERSION).unwrap(),
        );

        let _http_client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(USER_AGENT)
            .build()
            .unwrap();

        let users = UserService::new(_http_client.clone());
        let videos = VideoService::new(_http_client.clone());

        Client {
            _http_client,

            users,
            videos,
        }
    }

    pub fn users(&self) -> &UserService {
        &self.users
    }

    pub fn videos(&self) -> &VideoService {
        &self.videos
    }
}
