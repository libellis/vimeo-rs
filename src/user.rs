use serde::{Serialize, Deserialize};
use crate::{Client, Service};
use reqwest::Method;

pub struct UsersService {
    client: Client
}

impl Service<User> for UsersService {
    fn client(&self) -> &Client {
        &self.client
    }
}

impl UsersService {
    pub async fn get(&self, maybe_uid: Option<String>) -> Result<User, reqwest::Error> {
        let u = if let Some(uid) = maybe_uid {
            format!("users/{}", uid)
        } else {
            "me".to_string()
        };

        let user = self.request_builder(Method::GET, u, None)
            .send()
            .await?
            .json::<User>()
            .await?;

        Ok(user)
    }
}

// TODO: check omitempty's - if they are needed.
#[derive(Serialize, Deserialize)]
pub struct User {
    uri: String,
    name: String,
    link: String,
    location: String,
    bio: String,

    // TODO: Convert to time type.
    creation_time: String,

    account: String,
    pictures: Pictures,
    websites: Vec<Website>,
    content_filter: Vec<String>,
    resource_key: String,
}

// Pictures internal object provides access to pictures.
#[derive(Serialize, Deserialize)]
pub struct Pictures {
    uri: String,
    active: bool,
    #[serde(rename = "type")]
    ty: String,
    sizes: Vec<PictureSize>,
    link: String,
    resource_key: String,
}

// PictureSize internal object provides access to picture size.
#[derive(Serialize, Deserialize)]
pub struct PictureSize {
    width: i32,
    height: i32,
    link: String,
    link_with_play_button: String,
}

// WebSite represents a web site.
#[derive(Serialize, Deserialize)]
pub struct Website {
    name: String,
    link: String,
    description: String,
}