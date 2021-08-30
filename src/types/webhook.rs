//! Struct related to the structure of a Discord Webhook

use serde::Serialize;
use crate::Embed;

/// A webhook
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

/// Builder for Webhook
#[derive(Default)]
pub struct WebhookBuilder<'a> {
    /// Inner data
    inner: Webhook<'a>
}

impl<'a> WebhookBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// the message contents (up to 2000 characters)
    pub fn set_content(mut self, content: &'a str) -> Self {
        self.inner.content = Some(content);
        self
    }

    /// override the default username of the webhook
    pub fn set_username(mut self, username: &'a str) -> Self {
        self.inner.username = Some(username);
        self
    }

    /// override the default avatar of the webhook
    pub fn set_avatar_url(mut self, avatar_url: &'a str) -> Self {
        self.inner.avatar_url = Some(avatar_url);
        self
    }

    /// true if this is a TTS message
    pub fn set_tts(mut self, tts: bool) -> Self {
        self.inner.tts = Some(tts);
        self
    }

    /// the contents of the file being sent
    pub fn set_file(mut self, file: &'a [u8]) -> Self {
        self.inner.file = Some(file);
        self
    }

    /// embedded rich content
    pub fn set_embeds(mut self, embeds: Vec<Embed<'a>>) -> Self {
        self.inner.embeds = Some(embeds);
        self
    }

    /// allowed mentions for the message
    pub fn set_allowed_mentions(mut self, allowed_mentions: &'a AllowedMention<'a>) -> Self {
        self.inner.allowed_mentions = Some(allowed_mentions);
        self
    }

    /// Build the Webhook
    ///
    /// # Panics
    /// - If more than one of `content`, `file`, or `embeds` has been set
    /// - If there are more than 10 embeds
    pub fn build(self) -> Webhook<'a> {
        let content = self.inner.content.is_some();
        let file = self.inner.file.is_some();
        let embeds = self.inner.embeds.is_some();

        if !(content ^ file ^ embeds) {
            #[cfg(not(feature = "no-panic"))]
            {
                panic!("You can only supply one of 'file', 'content' or 'embeds', not more than one.");
            }
        }

        if let Some(embeds) = &self.inner.embeds {
            if embeds.len() > 10 {
                #[cfg(not(feature = "no-panic"))]
                {
                    panic!("You may supply up to 10 embeds, not more");
                }
            }
        }

        self.inner
    }
}

/// The allowed mention object allows for more granular control over mentions without various hacks to the message content
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

/// Builder for AllowedMentions
#[derive(Default)]
pub struct AllowedMentionBuilder<'a> {
    /// Inner data
    inner: AllowedMention<'a>
}

impl<'a> AllowedMentionBuilder<'a> {
    /// Create a new Builder
    pub fn new() -> Self {
        Self { inner: Default::default() }
    }

    /// An array of allowed mention types to parse from the content.
    pub fn set_parse(mut self, parse: Vec<AllowedMentionType>) -> Self {
        self.inner.parse = parse;
        self
    }

    /// Array of role_ids to mention (Max size of 100)
    pub fn set_roles(mut self, roles: Vec<&'a str>) -> Self {
        if roles.len() > 100 {
            #[cfg(not(feature = "no-panic"))]
            {
                panic!("The max size for the roles vec is 100");
            }
        }

        self.inner.roles = roles;
        self
    }

    /// Array of user_ids to mention (Max size of 100)
    pub fn set_users(mut self, users: Vec<&'a str>) -> Self {
        if users.len() > 100 {
            #[cfg(not(feature = "no-panic"))]
            {
                panic!("The max size for the users vec is 100");
            }
        }

        self.inner.users = users;
        self
    }

    /// For replies, whether to mention the author of the message being replied to (default false)
    pub fn set_replied_user(mut self, replied_user: bool) -> Self {
        self.inner.replied_user = replied_user;
        self
    }

    /// Build the Builder
    pub fn build(self) -> AllowedMention<'a> {
        self.inner
    }
}

/// The type of Allowed Mentions
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