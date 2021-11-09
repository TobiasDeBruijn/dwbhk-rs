//! Structs related to the structure of a Discord Embed

use serde::Serialize;

/// An Embed
#[derive(Serialize, Clone, Debug)]
pub struct Embed<'a> {
    /// title of embed
    pub title:          Option<String>,
    /// type of embed (always "rich" for webhook embeds)
    pub r#type:         &'static str,
    /// description of embed
    pub description:    Option<String>,
    /// url of embed
    pub url:            Option<String>,
    /// timestamp of embed content
    pub timestamp:      Option<String>,
    /// color code of the embed
    /// To convert a hex color code to the required format, one can do
    /// ```rust
    /// let color = "ff0000";
    /// let formatted_color = i64::from_str_radix(color, 16).unwrap();
    /// ```
    pub color:          Option<i64>,
    /// footer information
    pub footer:         Option<&'a EmbedFooter>,
    /// image information
    pub image:          Option<&'a EmbedImage>,
    /// thumbnail information
    pub thumbnail:      Option<&'a EmbedThumbnail>,
    /// video information
    pub video:          Option<&'a EmbedVideo>,
    /// provider information
    pub provider:       Option<&'a EmbedProvider>,
    /// author information
    pub author:         Option<&'a EmbedAuthor>,
    /// fields information
    pub fields:         Option<Vec<EmbedField>>
}

impl<'a> Default for Embed<'a> {
    fn default() -> Self {
        Embed {
            title:          None,
            r#type:         "rich",
            description:    None,
            url:            None,
            timestamp:      None,
            color:          None,
            footer:         None,
            image:          None,
            thumbnail:      None,
            video:          None,
            provider:       None,
            author:         None,
            fields:         None
        }
    }
}

/// Builder for Embed
#[derive(Default)]
pub struct EmbedBuilder<'a> {
    /// inner data
    inner: Embed<'a>
}

impl<'a> EmbedBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// title of embed
    pub fn set_title<S: 'a + AsRef<str>>(mut self, title: S) -> Self {
        self.inner.title = Some(title.as_ref().to_string());
        self
    }

    /// description of embed
    pub fn set_description<S: 'a + AsRef<str>>(mut self, description: S) -> Self {
        self.inner.description = Some(description.as_ref().to_string());
        self
    }

    /// url of embed
    pub fn set_url<S: 'a + AsRef<str>>(mut self, url: S) -> Self {
        self.inner.url = Some(url.as_ref().to_string());
        self
    }

    /// timestamp of embed content
    pub fn set_timestamp<S: 'a + AsRef<str>>(mut self, timestamp: S) -> Self {
        self.inner.timestamp = Some(timestamp.as_ref().to_string());
        self
    }

    /// color code of the embed in decimals
    pub fn set_color_decimal(mut self, color: i64) -> Self {
        self.inner.color = Some(color);
        self
    }

    /// color code of the embed in hexadecimal
    pub fn set_color_hex(mut self, color: &str) -> Self {
        self.inner.color = Some(i64::from_str_radix(&color.replace("#", ""), 16).unwrap());
        self
    }

    /// footer information
    pub fn set_footer(mut self, footer: &'a EmbedFooter) -> Self {
        self.inner.footer = Some(footer);
        self
    }

    /// image information
    pub fn set_image(mut self, image: &'a EmbedImage) -> Self {
        self.inner.image = Some(image);
        self
    }

    /// thumbnail information
    pub fn set_thumbnail(mut self, thumbnail: &'a EmbedThumbnail) -> Self {
        self.inner.thumbnail = Some(thumbnail);
        self
    }

    /// video information
    pub fn set_video(mut self, video: &'a EmbedVideo) -> Self {
        self.inner.video = Some(video);
        self
    }

    /// provider information
    pub fn set_provider(mut self, provider: &'a EmbedProvider) -> Self {
        self.inner.provider = Some(provider);
        self
    }

    /// author information
    pub fn set_author(mut self, author: &'a EmbedAuthor) -> Self {
        self.inner.author = Some(author);
        self
    }

    /// fields information
    pub fn set_fields(mut self, fields: Vec<EmbedField>) -> Self {
        self.inner.fields = Some(fields);
        self
    }

    /// Build the Builder
    pub fn build(self) -> Embed<'a> {
        self.inner
    }
}

/// The footer of an Embed
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedFooter {
    /// footer text
    pub text:           String,
    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url:       Option<String>,
    /// a proxied url of footer icon
    pub proxy_icon_url: Option<String>
}

/// Builder for EmbedFooter
#[derive(Default)]
pub struct EmbedFooterBuilder {
    /// Inner data
    inner: EmbedFooter
}

impl<'a> EmbedFooterBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// footer text
    pub fn set_text<S: 'a + AsRef<str>>(mut self, text: S) -> Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    /// url of footer icon (only supports http(s) and attachments)
    pub fn set_icon_url<S: 'a + AsRef<str>>(mut self, icon_url: S) -> Self {
        self.inner.icon_url = Some(icon_url.as_ref().to_string());
        self
    }

    /// a proxied url of footer icon
    pub fn set_proxy_icon_url<S: 'a + AsRef<str>>(mut self, proxy_icon_url: S) -> Self {
        self.inner.proxy_icon_url = Some(proxy_icon_url.as_ref().to_string());
        self
    }

    /// Build the Builder
    pub fn build(self) -> EmbedFooter {
        self.inner
    }
}

/// Embed image
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedImage {
    /// source url of image (only supports http(s) and attachments)
    pub url:            Option<String>,
    /// a proxied url of the image
    pub proxy_url:      Option<String>,
    /// height of image
    pub height:         Option<i32>,
    /// width of image
    pub width:          Option<i32>,
}

/// Builder for EmbedImage
#[derive(Default)]
pub struct EmbedImageBuilder {
    /// Inner data
    inner: EmbedImage
}

impl<'a> EmbedImageBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// source url of image (only supports http(s) and attachments)
    pub fn set_url<S: 'a + AsRef<str>>(mut self, url: S) -> Self {
        self.inner.url = Some(url.as_ref().to_string());
        self
    }

    /// a proxied url of the image
    pub fn set_proxy_url<S: 'a + AsRef<str>>(mut self, proxy_url: S) -> Self {
        self.inner.proxy_url = Some(proxy_url.as_ref().to_string());
        self
    }

    /// height of image
    pub fn set_height(mut self, height: i32) -> Self {
        self.inner.height = Some(height);
        self
    }

    /// width of image
    pub fn set_width(mut self, width: i32) -> Self {
        self.inner.width = Some(width);
        self
    }

    /// Build the Builder
    pub fn build(self) -> EmbedImage {
        self.inner
    }
}

/// Embed Thumbnail
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedThumbnail {
    /// source url of thumbnail (only supports http(s) and attachments)
    pub url:            Option<String>,
    /// a proxied url of the thumbnail
    pub proxy_url:      Option<String>,
    /// height of thumbnail
    pub height:         Option<i32>,
    /// width of thumbnail
    pub width:          Option<i32>,
}

/// Builder for EmbedThumbnail
#[derive(Default)]
pub struct EmbedThumbnailBuilder {
    /// Internal data
    inner: EmbedThumbnail
}

impl<'a> EmbedThumbnailBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// source url of thumbnail (only supports http(s) and attachments)
    pub fn set_url<S: 'a + AsRef<str>>(mut self, url: S) -> Self {
        self.inner.url = Some(url.as_ref().to_string());
        self
    }

    /// a proxied url of the thumbnail
    pub fn set_proxy_url<S: 'a + AsRef<str>>(mut self, proxy_url: S) -> Self {
        self.inner.proxy_url = Some(proxy_url.as_ref().to_string());
        self
    }

    /// height of thumbnail
    pub fn set_height(mut self, height: i32) -> Self {
        self.inner.height = Some(height);
        self
    }

    /// width of thumbnail
    pub fn set_width(mut self, width: i32) -> Self {
        self.inner.width = Some(width);
        self
    }

    /// Build the Builder
    pub fn build(self) -> EmbedThumbnail {
        self.inner
    }
}

/// Embed provider
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedProvider {
    /// name of provider
    pub name:           Option<String>,
    /// url of provider
    pub url:            Option<String>
}

/// Builder for EmbedProvider
#[derive(Default)]
pub struct EmbedProviderBuilder {
    /// Internal data
    inner: EmbedProvider
}

impl<'a> EmbedProviderBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// name of provider
    pub fn set_name<S: 'a + AsRef<str>>(mut self, name: S) -> Self {
        self.inner.name = Some(name.as_ref().to_string());
        self
    }

    /// url of provider
    pub fn set_url<S: 'a + AsRef<str>>(mut self, url: S) -> Self {
        self.inner.url = Some(url.as_ref().to_string());
        self
    }

    /// Build the Builder
    pub fn build(self) -> EmbedProvider {
        self.inner
    }
}

/// Embed Author
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedAuthor {
    /// name of author
    pub name:           Option<String>,
    /// url of author
    pub url:            Option<String>,
    /// url of author icon (only supports http(s) and attachments)
    pub icon_url:       Option<String>,
    /// a proxied url of author icon
    pub proxy_icon_url: Option<String>
}

/// Builder for EmbedAuthor
#[derive(Default)]
pub struct EmbedAuthorBuilder {
    /// Inner data
    inner: EmbedAuthor
}

impl<'a> EmbedAuthorBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// name of author
    pub fn set_name<S: 'a + AsRef<str>>(mut self, name: S) -> Self {
        self.inner.name = Some(name.as_ref().to_string());
        self
    }

    /// url of author
    pub fn set_url<S: 'a + AsRef<str>>(mut self, url: S) -> Self {
        self.inner.url = Some(url.as_ref().to_string());
        self
    }

    /// url of author icon (only supports http(s) and attachments)
    pub fn set_icon_url<S: 'a + AsRef<str>>(mut self, icon_url: S) -> Self {
        self.inner.icon_url = Some(icon_url.as_ref().to_string());
        self
    }

    /// a proxied url of author icon
    pub fn set_proxy_icon_url<S: 'a + AsRef<str>>(mut self, proxy_icon_url: S) -> Self {
        self.inner.proxy_icon_url = Some(proxy_icon_url.as_ref().to_string());
        self
    }

    /// Build the builder
    pub fn build(self) -> EmbedAuthor {
        self.inner
    }
}

/// Embed Video
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedVideo {
    /// source url of video
    pub url:            Option<String>,
    /// a proxied url of the video
    pub proxy_url:      Option<String>,
    /// height of video
    pub height:         Option<i32>,
    /// width of video
    pub width:          Option<i32>,
}

/// Builder for EmbedVideo
#[derive(Default)]
pub struct EmbedVideoBuilder {
    /// Internal data
    inner: EmbedVideo
}

impl<'a> EmbedVideoBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// source url of video
    pub fn set_url<S: 'a + AsRef<str>>(mut self, url: S) -> Self {
        self.inner.url = Some(url.as_ref().to_string());
        self
    }

    /// a proxied url of the video
    pub fn set_proxy_url<S: 'a + AsRef<str>>(mut self, proxy_url: S) -> Self {
        self.inner.proxy_url = Some(proxy_url.as_ref().to_string());
        self
    }

    /// height of video
    pub fn set_height(mut self, height: i32) -> Self {
        self.inner.height = Some(height);
        self
    }

    /// width of video
    pub fn set_width(mut self, width: i32) -> Self {
        self.inner.width = Some(width);
        self
    }

    /// Build the builder
    pub fn build(self) -> EmbedVideo {
        self.inner
    }
}

/// Embed Field
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedField {
    /// name of the field
    pub name:           String,
    /// value of the field
    pub value:          String,
    /// whether or not this field should display inline
    pub inline:         Option<bool>
}

/// Builder for EmbedField
#[derive(Default)]
pub struct EmbedFieldBuilder {
    /// Internal data
    inner: EmbedField
}

impl<'a> EmbedFieldBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// name of the field
    pub fn set_name<S: 'a + AsRef<str>>(mut self, name: S) -> Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    /// value of the field
    pub fn set_value<S: 'a + AsRef<str>>(mut self, value: S) -> Self {
        self.inner.value = value.as_ref().to_string();
        self
    }

    /// whether or not this field should display inline
    pub fn set_inline(mut self, inline: bool) -> Self {
        self.inner.inline = Some(inline);
        self
    }

    /// Build the builder
    pub fn build(self) -> EmbedField {
        self.inner
    }
}
