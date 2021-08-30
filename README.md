# dwbhk-rs
This is a Rust library for firing a Discord webhook

## Usage
Example usage:
```rs
let req = WebhookRequestBuilder::new()
    .set_data(WebhookBuilder::new()
        .set_embeds(vec![
            EmbedBuilder::new()
                .set_title("The Embed Title")
                .set_color_hex("#ff0000") // Red
                .set_description("Hello world!")
                .set_fields(vec![
                    EmbedFieldBuilder::new()
                        .set_name("Field Name")
                        .set_value("Field Value")
                        .build()
                    ]
                )
                .build()
            ]
        )
        .build()
    )
    .build();

let url = get_discord_webhook_url();
req.execute_url(&url);
```