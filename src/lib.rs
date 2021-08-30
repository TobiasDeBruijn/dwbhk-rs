//! dwbhk is a Rust library for firing a Discord webhook
//!
//! ## Usage
//! Example usage:
//! ```no_run
//! use dwbhk::*;
//!
//! let req = WebhookRequestBuilder::new()
//!     .set_data(WebhookBuilder::new()
//!         .set_embeds(vec![
//!             EmbedBuilder::new()
//!                 .set_title("The Embed Title")
//!                 .set_color_hex("#ff0000") // Red
//!                 .set_description("Hello world!")
//!                 .set_fields(vec![
//!                     EmbedFieldBuilder::new()
//!                         .set_name("Field Name")
//!                         .set_value("Field Value")
//!                         .build()
//!                    ]
//!                 )
//!                 .build()
//!             ]
//!         )
//!         .build()
//!    )
//!    .build();
//!
//! let url = get_discord_webhook_url();
//! req.execute_url(&url);
//! ```

#![warn(rust_2018_idioms)]
#![warn(clippy::cargo)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::if_not_else)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::needless_continue)]

mod types;
pub use types::*;

mod webhook;
pub use webhook::*;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn request_embed() {
        let req = WebhookRequestBuilder::new()
            .set_data(WebhookBuilder::new()
                .set_embeds(vec![
                    EmbedBuilder::new()
                        .set_title("TITLEEEE")
                        .set_color_hex("#ff0000")
                        .set_description("Hello world!")
                        .set_fields(vec![
                            EmbedFieldBuilder::new()
                                .set_name("Test")
                                .set_value("Value")
                                .build()
                            ]
                        )
                        .build()
                    ]
                )
                .build()
            )
            .build();

        let url = std::env::var("DISCORD_WEBHOOK_URL").unwrap();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .unwrap();
        let _guard = rt.enter();

        rt.block_on(req.execute_url(&url)).unwrap();
    }
}