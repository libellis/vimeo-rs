use crate::{Client, Pictures, Service, User};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// VideosService handles communication with the videos related
/// methods of the Vimeo API.
///
/// Vimeo API docs: https://developer.vimeo.com/api/reference/videos
pub struct VideoService {
    client: Client,
}

impl Service<Video> for VideoService {
    fn client(&self) -> &Client {
        &self.client
    }
}

impl VideoService {
    pub fn new(client: Client) -> VideoService {
        VideoService { client }
    }

    // List will either list all the videos of the user id supplied, or if None is supplied will list all
    // the videos of the currently logged in user.
    pub async fn list(&self, maybe_uid: Option<String>) -> Result<Vec<Video>, reqwest::Error> {
        let u = if let Some(uid) = maybe_uid {
            format!("users/{}/videos", uid)
        } else {
            "me/videos".to_string()
        };

        let videos: VideoList = self
            .request_builder(Method::GET, u, None)
            .send()
            .await?
            .json::<VideoList>()
            .await?;

        Ok(videos.data)
    }
}

// TODO: It looks like the vimeo API pages the response here so we need to deal with paging.
#[derive(Serialize, Deserialize, Debug)]
struct VideoList {
    data: Vec<Video>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    uri:            String,
    name:           String,
    description:    Option<String>,
    link:           String,
    duration:       i32,
    width:          i32,
    height:         i32,
    language:       String,
    // TODO: Add this in once type is made.
    // embed: Embed,
    // TODO: Switch these to a Time type.
    created_time:   String,
    modified_time:  String,
    release_time:   String,
    content_rating: Vec<String>,
    license:        Option<String>,
    privacy:        Privacy,
    pictures:       Pictures,
    tags:           Vec<Tag>,
    stats:          Stats,
    categories:     Vec<Category>,
    user:           User,
    files:          Option<Vec<File>>,
    // app: App,
    status:         String,
    resource_key:   String,
    embed_presets:  Option<EmbedPresets>,
    upload:         Upload,
    transcode:      Transcode,
}

// File internal object provides access to video file information
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    quality:      String,
    #[serde(rename = "type")]
    ty:           String,
    width:        i32,
    height:       i32,
    link:         String,
    // TODO: Convert to Time type.
    created_time: String,
    fps:          f32,
    size:         i32,
    md5:          String,
}

// Stats internal object provides access to video statistic.
#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    plays: i32,
}

// Category represents a category.
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    uri:                      String,
    link:                     String,
    name:                     String,
    top_level:                bool,
    pictures:                 Pictures,
    last_video_featured_time: String,
    parent:                   SubCategory,
    subcategories:            Vec<SubCategory>,
    resource_key:             String,
}

// SubCategory internal object provides access to subcategory in category.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubCategory {
    uri:  String,
    name: String,
    link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    uri:          String,
    name:         String,
    tag:          String,
    canonical:    String,
    resource_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedPresets {
    uri:      String,
    name:     String,
    settings: EmbedSettings,
    user:     Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedSettings {
    buttons:                            Option<Buttons>,
    logos:                              Logos,
    outro:                              String,
    portrait:                           String,
    title:                              String,
    byline:                             String,
    badge:                              bool,
    byline_badge:                       bool,
    collections_button:                 bool,
    playbar:                            bool,
    volume:                             bool,
    fullscreen_button:                  bool,
    scaling_button:                     bool,
    autoplay:                           bool,
    autopause:                          bool,
    #[serde(rename = "loop")]
    looping:                            bool,
    color:                              String,
    link:                               bool,
    overlay_email_capture:              i32,
    overlay_email_capture_text:         String,
    overlay_email_capture_confirmation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buttons {
    like:       bool,
    watchlater: bool,
    share:      bool,
    embed:      bool,
    vote:       bool,
    #[serde(rename = "HD")]
    hd:         bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logos {
    vimeo: bool,
    // TODO: Looking at the response, this is an object not a bool.
    // custom:        bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Upload {
    status:       String,
    upload_link:  Option<String>,
    redirect_url: Option<String>,
    link:         Option<String>,
    form:         Option<String>,
    approach:     Option<String>,
    size:         Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transcode {
    status: String,
}

// TODO: Probably move this to some kind of common file once we expand this client lib.
#[derive(Serialize, Deserialize, Debug)]
pub struct Privacy {
    view:     String,
    comments: String,
    embed:    String,
    download: bool,
    add:      bool,

    // TODO: In my testing so far these don't exist. Maybe remove entirely?
    join:   Option<String>,
    videos: Option<String>,
    forums: Option<String>,
    invite: Option<String>,
}
