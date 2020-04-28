use crate::{Service};
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub struct UserService {
    http_client: reqwest::Client,
}

impl Service<User> for UserService {
    fn client(&self) -> &reqwest::Client {
        &self.http_client
    }
}

impl UserService {
    pub fn new(client: reqwest::Client) -> UserService {
        UserService { http_client: client }
    }

    pub async fn get(&self, maybe_uid: Option<String>) -> Result<User, reqwest::Error> {
        let u = if let Some(uid) = maybe_uid {
            format!("users/{}", uid)
        } else {
            "me".to_string()
        };

        let user = self
            .request_builder(Method::GET, u, None)
            .send()
            .await?
            .json::<User>()
            .await?;

        Ok(user)
    }
}

// TODO: check omitempty's - if they are needed.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    uri:            String,
    name:           String,
    link:           String,
    location:       String,
    bio:            Option<String>,
    // TODO: Convert to time type.
    created_time:   String,
    account:        String,
    pictures:       Pictures,
    websites:       Vec<Website>,
    content_filter: Vec<String>,
    resource_key:   String,
}

// Pictures internal object provides access to pictures.
#[derive(Serialize, Deserialize, Debug)]
pub struct Pictures {
    uri:          Option<String>,
    active:       bool,
    #[serde(rename = "type")]
    ty:           String,
    sizes:        Vec<PictureSize>,
    link:         Option<String>,
    resource_key: String,
}

// PictureSize internal object provides access to picture size.
#[derive(Serialize, Deserialize, Debug)]
pub struct PictureSize {
    width:                 i32,
    height:                i32,
    link:                  String,
    link_with_play_button: Option<String>,
}

// WebSite represents a web site.
#[derive(Serialize, Deserialize, Debug)]
pub struct Website {
    name:        String,
    link:        String,
    description: String,
}
