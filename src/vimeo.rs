use crate::{Client, UserService, VideoService};

// VimeoServices holds all Vimeo API services.
// A service is related to a specific API resource you would like to interact with.
pub struct VimeoServices {
    client: Client,

    users:  UserService,
    videos: VideoService,
}

impl VimeoServices {
    pub fn new(access_token: String) -> VimeoServices {
        let client = Client::new(&access_token);

        let users = UserService::new(client.clone());
        let videos = VideoService::new(client.clone());

        VimeoServices {
            client,
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
