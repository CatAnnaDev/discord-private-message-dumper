use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user: User,
    #[serde(rename = "connected_accounts")]
    pub connected_accounts: Vec<ConnectedAccount>,
    #[serde(rename = "premium_since")]
    pub premium_since: Value,
    #[serde(rename = "premium_type")]
    pub premium_type: Value,
    #[serde(rename = "premium_guild_since")]
    pub premium_guild_since: Value,
    #[serde(rename = "profile_themes_experiment_bucket")]
    pub profile_themes_experiment_bucket: i64,
    #[serde(rename = "user_profile")]
    pub user_profile: UserProfile,
    pub badges: Vec<Value>,
    #[serde(rename = "guild_badges")]
    pub guild_badges: Vec<Value>,
    #[serde(rename = "mutual_guilds")]
    pub mutual_guilds: Vec<MutualGuild>,
    #[serde(rename = "legacy_username")]
    pub legacy_username: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(rename = "global_name")]
    pub global_name: String,
    pub avatar: String,
    #[serde(rename = "avatar_decoration_data")]
    pub avatar_decoration_data: Option<Value>,
    pub discriminator: String,
    #[serde(rename = "public_flags")]
    pub public_flags: i64,
    pub flags: i64,
    pub banner: Option<Value>,
    #[serde(rename = "banner_color")]
    pub banner_color: Option<String>,
    #[serde(rename = "accent_color")]
    pub accent_color: Option<i64>,
    pub bio: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedAccount {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub verified: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub bio: Option<String>,
    #[serde(rename = "accent_color")]
    pub accent_color: Option<i64>,
    pub pronouns: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualGuild {
    pub id: Option<String>,
    pub nick: Option<String>,
}
