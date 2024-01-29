use reqwest::Client;
use serde_json::from_str;

use crate::profile_struct::Profile;
use crate::web_client::Token;

pub async fn get_user_profile_info(client: &Client, id: &String) -> Profile {
    from_str::<Profile>(Token::http(client, format!("https://discord.com/api/v9/users/{id}/profile")).await.unwrap().as_str()).unwrap()
}