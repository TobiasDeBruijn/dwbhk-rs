//! Module containing a webhook's execution logic

use crate::Webhook;
use reqwest::{Response, Error};
use lazy_static::lazy_static;

lazy_static! {
    /// HTTP client to be reused for every webhook call
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

/// A webhook request
#[derive(Default, Clone, Debug)]
pub struct WebhookRequest<'a> {
    /// The Webhook's payload
    pub data:       Webhook<'a>,
    /// The name of the File to be send, if there is any
    pub file_name:  Option<String>
}

/// Builder for WebhookRequest
#[derive(Default)]
pub struct WebhookRequestBuilder<'a> {
    /// Inner data
    inner: WebhookRequest<'a>
}

impl<'a> WebhookRequestBuilder<'a> {
    /// Create a Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// Set the webhook data to be send
    pub fn set_data(mut self, data: Webhook<'a>) -> Self {
        self.inner.data = data;
        self
    }

    /// Set the filename to be used for an uploaded file
    pub fn set_file_name<S: 'a + AsRef<str>>(mut self, file_name: S) -> Self {
        self.inner.file_name = Some(file_name.as_ref().to_string());
        self
    }

    /// Build the WebhookRequest
    ///
    /// # Panics
    /// Panics if a file name has not been set and a file is to be uploaded
    pub fn build(self) -> WebhookRequest<'a> {
        if self.inner.data.file.is_some() && self.inner.file_name.is_none() {
            #[cfg(not(feature = "no-panic"))]
            {
                panic!("You must set a file name if you are uploading a file");
            }
        }

        self.inner
    }
}

impl<'a> WebhookRequest<'a> {
    /// Execute the current webhook request to the target URL
    ///
    /// # Errors
    /// - When the request fails
    pub async fn execute_url<S: AsRef<str>>(&self, url: S) -> Result<Response, Error> {
        let mut req_builder = HTTP_CLIENT.post(url.as_ref())
            .json(&self.data);

        if let Some(f) = &self.file_name {
            req_builder = req_builder.header("Content-Disposition", f);
        }

        req_builder.send().await
    }

    /// Execute the current webhook request to a target URL build from the given `id` and `token`
    ///
    /// # Errors
    /// - When the request fails
    pub async fn execute<A: AsRef<str>, B: AsRef<str>>(&self, id: A, token: B) -> Result<Response, Error> {
        let mut req_builder = HTTP_CLIENT.post(format!("https://discord.com/api/webhooks/{}/{}", id.as_ref(), token.as_ref()))
            .json(&self.data);

        if let Some(f) = &self.file_name {
            req_builder = req_builder.header("Content-Disposition", f);
        }

        req_builder.send().await
    }
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use crate::WebhookRequest;
    use reqwest::blocking::Response;
    use reqwest::Error;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BLOCKING_HTTP_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
    }

    impl<'a> WebhookRequest<'a> {
        /// Execute the current webhook request to the target URL
        ///
        /// # Errors
        /// - When the request fails
        pub fn execute_url_sync<S: AsRef<str>>(&self, url: S) -> Result<Response, Error> {
            let mut req_builder = BLOCKING_HTTP_CLIENT.post(url.as_ref())
                .json(&self.data);

            if let Some(f) = &self.file_name {
                req_builder = req_builder.header("Content-Disposition", f);
            }

            req_builder.send()
        }

        /// Execute the current webhook request to a target URL build from the given `id` and `token`
        ///
        /// # Errors
        /// - When the request fails
        pub fn execute_sync<A: AsRef<str>, B: AsRef<str>>(&self, id: A, token: B) -> Result<Response, Error> {
            let mut req_builder = BLOCKING_HTTP_CLIENT.post(format!("https://discord.com/api/webhooks/{}/{}", id.as_ref(), token.as_ref()))
                .json(&self.data);

            if let Some(f) = &self.file_name {
                req_builder = req_builder.header("Content-Disposition", f);
            }

            req_builder.send()
        }
    }
}