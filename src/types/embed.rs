use serde::Serialize;

#[derive(Default, Serialize, Clone, Debug)]
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

pub struct EmbedBuilder<'a> {
    inner: Embed<'a>
}

impl<'a> EmbedBuilder<'a> {
    pub fn new() -> Self {
        let mut inner = Embed::default();
        inner.r#type = "rich";

        Self { inner }
    }

    pub fn set_title(mut self, title: &'a str) -> Self {
        self.inner.title = Some(title);
        self
    }

    pub fn set_description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }

    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    pub fn set_timestamp(mut self, timestamp: &'a str) -> Self {
        self.inner.timestamp = Some(timestamp);
        self
    }

    pub fn set_color_decimal(mut self, color: i64) -> Self {
        self.inner.color = Some(color);
        self
    }

    pub fn set_color_hex(mut self, color: &str) -> Self {
        self.inner.color = Some(i64::from_str_radix(&color.replace("#", ""), 16).unwrap());
        self
    }

    pub fn set_footer(mut self, footer: &'a EmbedFooter) -> Self {
        self.inner.footer = Some(footer);
        self
    }

    pub fn set_image(mut self, image: &'a EmbedImage) -> Self {
        self.inner.image = Some(image);
        self
    }

    pub fn set_thumbnail(mut self, thumbnail: &'a EmbedThumbnail) -> Self {
        self.inner.thumbnail = Some(thumbnail);
        self
    }

    pub fn set_video(mut self, video: &'a EmbedVideo) -> Self {
        self.inner.video = Some(video);
        self
    }

    pub fn set_provider(mut self, provider: &'a EmbedProvider) -> Self {
        self.inner.provider = Some(provider);
        self
    }

    pub fn set_author(mut self, author: &'a EmbedAuthor) -> Self {
        self.inner.author = Some(author);
        self
    }

    pub fn set_fields(mut self, fields: Vec<EmbedField<'a>>) -> Self {
        self.inner.fields = Some(fields);
        self
    }

    pub fn build(self) -> Embed<'a> {
        self.inner
    }
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedFooter<'a> {
    /// footer text
    pub text:           &'a str,
    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url:       Option<&'a str>,
    /// a proxied url of footer icon
    pub proxy_icon_url: Option<&'a str>
}

pub struct EmbedFooterBuilder<'a> {
    inner: EmbedFooter<'a>
}

impl<'a> EmbedFooterBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_text(mut self, text: &'a str) -> Self {
        self.inner.text = text;
        self
    }

    pub fn set_icon_url(mut self, icon_url: &'a str) -> Self {
        self.inner.icon_url = Some(icon_url);
        self
    }

    pub fn set_proxy_icon_url(mut self, proxy_icon_url: &'a str) -> Self {
        self.inner.proxy_icon_url = Some(proxy_icon_url);
        self
    }

    pub fn build(self) -> EmbedFooter<'a> {
        self.inner
    }
}

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

pub struct EmbedImageBuilder<'a> {
    inner: EmbedImage<'a>
}

impl<'a> EmbedImageBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    pub fn set_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.inner.proxy_url = Some(proxy_url);
        self
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self.inner.height = Some(height);
        self
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.inner.width = Some(width);
        self
    }

    pub fn build(self) -> EmbedImage<'a> {
        self.inner
    }
}

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

pub struct EmbedThumbnailBuilder<'a> {
    inner: EmbedThumbnail<'a>
}

impl<'a> EmbedThumbnailBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    pub fn set_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.inner.proxy_url = Some(proxy_url);
        self
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self.inner.height = Some(height);
        self
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.inner.width = Some(width);
        self
    }

    pub fn build(self) -> EmbedThumbnail<'a> {
        self.inner
    }
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedProvider<'a> {
    /// name of provider
    pub name:           Option<&'a str>,
    /// url of provider
    pub url:            Option<&'a str>
}

pub struct EmbedProviderBuilder<'a> {
    inner: EmbedProvider<'a>
}

impl<'a> EmbedProviderBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }

    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    pub fn build(self) -> EmbedProvider<'a> {
        self.inner
    }
}

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

pub struct EmbedAuthorBuilder<'a> {
    inner: EmbedAuthor<'a>
}

impl<'a> EmbedAuthorBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }

    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    pub fn set_icon_url(mut self, icon_url: &'a str) -> Self {
        self.inner.icon_url = Some(icon_url);
        self
    }

    pub fn set_proxy_icon_url(mut self, proxy_icon_url: &'a str) -> Self {
        self.inner.proxy_icon_url = Some(proxy_icon_url);
        self
    }

    pub fn build(self) -> EmbedAuthor<'a> {
        self.inner
    }
}

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

pub struct EmbedVideoBuilder<'a> {
    inner: EmbedVideo<'a>
}

impl<'a> EmbedVideoBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }

    pub fn set_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.inner.proxy_url = Some(proxy_url);
        self
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self.inner.height = Some(height);
        self
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.inner.width = Some(width);
        self
    }

    pub fn build(self) -> EmbedVideo<'a> {
        self.inner
    }
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct EmbedField<'a> {
    /// name of the field
    pub name:           &'a str,
    /// value of the field
    pub value:          &'a str,
    /// whether or not this field should display inline
    pub inline:         Option<bool>
}

pub struct EmbedFieldBuilder<'a> {
    inner: EmbedField<'a>
}

impl<'a> EmbedFieldBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_name(mut self, name: &'a str) -> Self {
        self.inner.name = name;
        self
    }

    pub fn set_value(mut self, value: &'a str) -> Self {
        self.inner.value = value;
        self
    }

    pub fn set_inline(mut self, inline: bool) -> Self {
        self.inner.inline = Some(inline);
        self
    }

    pub fn build(self) -> EmbedField<'a> {
        self.inner
    }
}
