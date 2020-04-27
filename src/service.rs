use url::Url;
use reqwest::RequestBuilder;
use serde::Serialize;

use crate::Client;

pub trait Service<T>
    where T: Serialize,
{
    fn request_builder(&self, method: reqwest::Method, rel_url: String, maybe_body_obj: Option<T>) -> RequestBuilder {
        let url = Url::parse(&format!("{}{}", self.client().base_url(), rel_url)).unwrap();

        let req_builder: RequestBuilder = self.client().http_client().request(method, url);

        if let Some(body_obj) = &maybe_body_obj {
            req_builder
                .json(body_obj)
        } else {
            req_builder
        }
    }

    fn client(&self) -> &Client;
}