//! Structs related to the structure of a Discord Embed

use serde::Serialize;

/// An Embed
#[derive(Serialize, Clone, Debug)]
pub struct Embed<'a> {
    /// title of embed
    pub title:          Option<&'a str>,
    /// type of embed (always "rich" for webhook embeds)
    pub r#type:         &'static str,
    /// description of embed
    pub description:    Option<&'a str>,
    /// url of embed
    pub url:            Option<&'a str>,
    /// timestamp of embed content
    pub timestamp:      Option<&'a str>,
    /// color code of the embed
    /// To convert a hex color code to the required format, one can do
    /// ```rust
    /// let color = "ff0000";
    /// let formatted_color = i64::from_str_radix(color, 16).unwrap();
    /// ```
    pub color:          Option<i64>,
    /// footer information
    pub footer:         Option<&'a EmbedFooter<'a>>,
    /// image information
    pub image:          Option<&'a EmbedImage<'a>>,
    /// thumbnail information
    pub thumbnail:      Option<&'a EmbedThumbnail<'a>>,
    /// video information
    pub video:          Option<&'a EmbedVideo<'a>>,
    /// provider information
    pub provider:       Option<&'a EmbedProvider<'a>>,
    /// author information
    pub author:         Option<&'a EmbedAuthor<'a>>,
    /// fields information
    pub fields:         Option<Vec<EmbedField<'a>>>
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
    pub fn set_title(mut self, title: &'a str) -> Self {
        self.inner.title = Some(title);
        self
    }

    /// description of embed
    pub fn set_description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }

    /// url of embed
    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    /// timestamp of embed content
    pub fn set_timestamp(mut self, timestamp: &'a str) -> Self {
        self.inner.timestamp = Some(timestamp);
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
    pub fn set_footer(mut self, footer: &'a EmbedFooter<'a>) -> Self {
        self.inner.footer = Some(footer);
        self
    }

    /// image information
    pub fn set_image(mut self, image: &'a EmbedImage<'a>) -> Self {
        self.inner.image = Some(image);
        self
    }

    /// thumbnail information
    pub fn set_thumbnail(mut self, thumbnail: &'a EmbedThumbnail<'a>) -> Self {
        self.inner.thumbnail = Some(thumbnail);
        self
    }

    /// video information
    pub fn set_video(mut self, video: &'a EmbedVideo<'a>) -> Self {
        self.inner.video = Some(video);
        self
    }

    /// provider information
    pub fn set_provider(mut self, provider: &'a EmbedProvider<'a>) -> Self {
        self.inner.provider = Some(provider);
        self
    }

    /// author information
    pub fn set_author(mut self, author: &'a EmbedAuthor<'a>) -> Self {
        self.inner.author = Some(author);
        self
    }

    /// fields information
    pub fn set_fields(mut self, fields: Vec<EmbedField<'a>>) -> Self {
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
pub struct EmbedFooter<'a> {
    /// footer text
    pub text:           &'a str,
    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url:       Option<&'a str>,
    /// a proxied url of footer icon
    pub proxy_icon_url: Option<&'a str>
}

/// Builder for EmbedFooter
#[derive(Default)]
pub struct EmbedFooterBuilder<'a> {
    /// Inner data
    inner: EmbedFooter<'a>
}

impl<'a> EmbedFooterBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// footer text
    pub fn set_text(mut self, text: &'a str) -> Self {
        self.inner.text = text;
        self
    }

    /// url of footer icon (only supports http(s) and attachments)
    pub fn set_icon_url(mut self, icon_url: &'a str) -> Self {
        self.inner.icon_url = Some(icon_url);
        self
    }

    /// a proxied url of footer icon
    pub fn set_proxy_icon_url(mut self, proxy_icon_url: &'a str) -> Self {
        self.inner.proxy_icon_url = Some(proxy_icon_url);
        self
    }

    /// Build the Builder
    pub fn build(self) -> EmbedFooter<'a> {
        self.inner
    }
}

/// Embed image
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedImage<'a> {
    /// source url of image (only supports http(s) and attachments)
    pub url:            Option<&'a str>,
    /// a proxied url of the image
    pub proxy_url:      Option<&'a str>,
    /// height of image
    pub height:         Option<i32>,
    /// width of image
    pub width:          Option<i32>,
}

/// Builder for EmbedImage
#[derive(Default)]
pub struct EmbedImageBuilder<'a> {
    /// Inner data
    inner: EmbedImage<'a>
}

impl<'a> EmbedImageBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// source url of image (only supports http(s) and attachments)
    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    /// a proxied url of the image
    pub fn set_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.inner.proxy_url = Some(proxy_url);
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
    pub fn build(self) -> EmbedImage<'a> {
        self.inner
    }
}

/// Embed Thumbnail
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedThumbnail<'a> {
    /// source url of thumbnail (only supports http(s) and attachments)
    pub url:            Option<&'a str>,
    /// a proxied url of the thumbnail
    pub proxy_url:      Option<&'a str>,
    /// height of thumbnail
    pub height:         Option<i32>,
    /// width of thumbnail
    pub width:          Option<i32>,
}

/// Builder for EmbedThumbnail
#[derive(Default)]
pub struct EmbedThumbnailBuilder<'a> {
    /// Internal data
    inner: EmbedThumbnail<'a>
}

impl<'a> EmbedThumbnailBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// source url of thumbnail (only supports http(s) and attachments)
    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    /// a proxied url of the thumbnail
    pub fn set_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.inner.proxy_url = Some(proxy_url);
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
    pub fn build(self) -> EmbedThumbnail<'a> {
        self.inner
    }
}

/// Embed provider
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedProvider<'a> {
    /// name of provider
    pub name:           Option<&'a str>,
    /// url of provider
    pub url:            Option<&'a str>
}

/// Builder for EmbedProvider
#[derive(Default)]
pub struct EmbedProviderBuilder<'a> {
    /// Internal data
    inner: EmbedProvider<'a>
}

impl<'a> EmbedProviderBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// name of provider
    pub fn set_name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }

    /// url of provider
    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    /// Build the Builder
    pub fn build(self) -> EmbedProvider<'a> {
        self.inner
    }
}

/// Embed Author
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedAuthor<'a> {
    /// name of author
    pub name:           Option<&'a str>,
    /// url of author
    pub url:            Option<&'a str>,
    /// url of author icon (only supports http(s) and attachments)
    pub icon_url:       Option<&'a str>,
    /// a proxied url of author icon
    pub proxy_icon_url: Option<&'a str>
}

/// Builder for EmbedAuthor
#[derive(Default)]
pub struct EmbedAuthorBuilder<'a> {
    /// Inner data
    inner: EmbedAuthor<'a>
}

impl<'a> EmbedAuthorBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// name of author
    pub fn set_name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }

    /// url of author
    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    /// url of author icon (only supports http(s) and attachments)
    pub fn set_icon_url(mut self, icon_url: &'a str) -> Self {
        self.inner.icon_url = Some(icon_url);
        self
    }

    /// a proxied url of author icon
    pub fn set_proxy_icon_url(mut self, proxy_icon_url: &'a str) -> Self {
        self.inner.proxy_icon_url = Some(proxy_icon_url);
        self
    }

    /// Build the builder
    pub fn build(self) -> EmbedAuthor<'a> {
        self.inner
    }
}

/// Embed Video
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedVideo<'a> {
    /// source url of video
    pub url:            Option<&'a str>,
    /// a proxied url of the video
    pub proxy_url:      Option<&'a str>,
    /// height of video
    pub height:         Option<i32>,
    /// width of video
    pub width:          Option<i32>,
}

/// Builder for EmbedVideo
#[derive(Default)]
pub struct EmbedVideoBuilder<'a> {
    /// Internal data
    inner: EmbedVideo<'a>
}

impl<'a> EmbedVideoBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// source url of video
    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    /// a proxied url of the video
    pub fn set_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.inner.proxy_url = Some(proxy_url);
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
    pub fn build(self) -> EmbedVideo<'a> {
        self.inner
    }
}

/// Embed Field
#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedField<'a> {
    /// name of the field
    pub name:           &'a str,
    /// value of the field
    pub value:          &'a str,
    /// whether or not this field should display inline
    pub inline:         Option<bool>
}

/// Builder for EmbedField
#[derive(Default)]
pub struct EmbedFieldBuilder<'a> {
    /// Internal data
    inner: EmbedField<'a>
}

impl<'a> EmbedFieldBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// name of the field
    pub fn set_name(mut self, name: &'a str) -> Self {
        self.inner.name = name;
        self
    }

    /// value of the field
    pub fn set_value(mut self, value: &'a str) -> Self {
        self.inner.value = value;
        self
    }

    /// whether or not this field should display inline
    pub fn set_inline(mut self, inline: bool) -> Self {
        self.inner.inline = Some(inline);
        self
    }

    /// Build the builder
    pub fn build(self) -> EmbedField<'a> {
        self.inner
    }
}
