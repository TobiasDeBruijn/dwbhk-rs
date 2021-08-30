use crate::Webhook;
use reqwest::{Response, Error};
use lazy_static::lazy_static;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Default, Clone, Debug)]
pub struct WebhookRequest<'a> {
    /// The Webhook's payload
    pub data:       Webhook<'a>,
    /// The name of the File to be send, if there is any
    pub file_name:  Option<&'a str>
}

pub struct WebhookRequestBuilder<'a> {
    inner: WebhookRequest<'a>
}

impl<'a> WebhookRequestBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_data(mut self, data: Webhook<'a>) -> Self {
        self.inner.data = data;
        self
    }

    pub fn set_file_name(mut self, file_name: &'a str) -> Self {
        self.inner.file_name = Some(file_name);
        self
    }

    pub fn build(self) -> WebhookRequest<'a> {
        if self.inner.data.file.is_some() && self.inner.file_name.is_none() {
            panic!("You must set a file name if you are uploading a file");
        }

        self.inner
    }
}

impl<'a> WebhookRequest<'a> {
    /// Execute the current webhook request to the target URL
    pub async fn execute_url(&self, url: &str) -> Result<Response, Error> {
        let mut req_builder = HTTP_CLIENT.post(url)
            .json(&self.data);

        if let Some(f) = self.file_name {
            req_builder = req_builder.header("Content-Disposition", f);
        }

        req_builder.send().await
    }

    /// Execute the current webhook request to a target URL build from the given `id` and `token`
    pub async fn execute(&self, id: &str, token: &str) -> Result<Response, Error> {
        let mut req_builder = HTTP_CLIENT.post(format!("https://discord.com/api/webhooks/{}/{}", id, token))
            .json(&self.data);

        if let Some(f) = self.file_name {
            req_builder = req_builder.header("Content-Disposition", f);
        }

        req_builder.send().await
    }
}