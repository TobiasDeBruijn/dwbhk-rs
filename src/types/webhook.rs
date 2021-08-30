use serde::Serialize;
use crate::Embed;

#[derive(Default, Serialize, Clone, Debug)]
pub struct Webhook<'a> {
    /// the message contents (up to 2000 characters)
    pub content:            Option<&'a str>,
    /// override the default username of the webhook
    pub username:           Option<&'a str>,
    /// override the default avatar of the webhook
    pub avatar_url:         Option<&'a str>,
    /// true if this is a TTS message
    pub tts:                Option<bool>,
    /// the contents of the file being sent
    pub file:               Option<&'a [u8]>,
    /// embedded rich content
    pub embeds:             Option<Vec<Embed<'a>>>,
    /// allowed mentions for the message
    pub allowed_mentions:   Option<&'a AllowedMention<'a>>,
}

#[derive(Default)]
pub struct WebhookBuilder<'a> {
    inner: Webhook<'a>
}

impl<'a> WebhookBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_content(mut self, content: &'a str) -> Self {
        self.inner.content = Some(content);
        self
    }

    pub fn set_username(mut self, username: &'a str) -> Self {
        self.inner.username = Some(username);
        self
    }

    pub fn set_avatar_url(mut self, avatar_url: &'a str) -> Self {
        self.inner.avatar_url = Some(avatar_url);
        self
    }

    pub fn set_tts(mut self, tts: bool) -> Self {
        self.inner.tts = Some(tts);
        self
    }

    pub fn set_file(mut self, file: &'a [u8]) -> Self {
        self.inner.file = Some(file);
        self
    }

    pub fn set_embeds(mut self, embeds: Vec<Embed<'a>>) -> Self {
        self.inner.embeds = Some(embeds);
        self
    }

    pub fn set_allowed_mentions(mut self, allowed_mentions: &'a AllowedMention) -> Self {
        self.inner.allowed_mentions = Some(allowed_mentions);
        self
    }

    pub fn build(self) -> Webhook<'a> {
        let content = self.inner.content.is_some();
        let file = self.inner.file.is_some();
        let embeds = self.inner.embeds.is_some();

        if !(content ^ file ^ embeds) {
            panic!("You can only supply one of 'file', 'content' or 'embeds', not more than one.");
        }

        if let Some(embeds) = &self.inner.embeds {
            if embeds.len() > 10 {
                panic!("You may supply up to 10 embeds, not more");
            }
        }

        self.inner
    }
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct AllowedMention<'a> {
    /// An array of allowed mention types to parse from the content.
    pub parse:              Vec<AllowedMentionType>,
    /// Array of role_ids to mention (Max size of 100)
    pub roles:              Vec<&'a str>,
    /// Array of user_ids to mention (Max size of 100)
    pub users:              Vec<&'a str>,
    /// For replies, whether to mention the author of the message being replied to (default false)
    pub replied_user:       bool
}

#[derive(Default)]
pub struct AllowedMentionBuilder<'a> {
    inner: AllowedMention<'a>
}

impl<'a> AllowedMentionBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    pub fn set_parse(mut self, parse: Vec<AllowedMentionType>) -> Self {
        self.inner.parse = parse;
        self
    }

    pub fn set_roles(mut self, roles: Vec<&'a str>) -> Self {
        if roles.len() > 100 {
            panic!("The max size for the roles vec is 100");
        }

        self.inner.roles = roles;
        self
    }

    pub fn set_users(mut self, users: Vec<&'a str>) -> Self {
        if users.len() > 100 {
            panic!("The max size for the users vec is 100");
        }

        self.inner.users = users;
        self
    }

    pub fn set_replied_user(mut self, replied_user: bool) -> Self {
        self.inner.replied_user = replied_user;
        self
    }

    pub fn build(self) -> AllowedMention<'a> {
        self.inner
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum AllowedMentionType {
    /// Controls role mentions
    #[serde(rename(serialize = "roles"))]
    RoleMention,
    /// Controls user mentions
    #[serde(rename(serialize = "users"))]
    UserMention,
    /// Controls @everyone and @here mentions
    #[serde(rename(serialize = "everyone"))]
    EveryoneMention
}