use reqwest::RequestBuilder;
use serde::Serialize;
use url::Url;

const BASE_URL: &'static str = "https://api.vimeo.com/";

pub trait Service<T>
where
    T: Serialize,
{
    fn request_builder(
        &self,
        method: reqwest::Method,
        rel_url: String,
        maybe_body_obj: Option<T>,
    ) -> RequestBuilder {
        let url = Url::parse(&format!("{}{}", BASE_URL, rel_url)).unwrap();

        let req_builder: RequestBuilder = self.client().request(method, url);

        if let Some(body_obj) = &maybe_body_obj {
            req_builder.json(body_obj)
        } else {
            req_builder
        }
    }

    fn client(&self) -> &reqwest::Client;
}
