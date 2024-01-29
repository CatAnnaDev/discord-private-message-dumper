use std::error::Error;

use crate::dm_message::RecursiveDump;
use crate::dumper_builder::Dump;
use crate::profile::get_user_profile_info;
use crate::web_client::Token;

mod message;
mod profile;
mod profile_struct;
mod dm_message;
mod dumper_builder;
mod web_client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let profile_dump = Dump::new("user_id".to_string(), "channel_id".to_string());
    let client = Token::new_web_client(Token { token: "xhr_token".to_string() });

    let profile = get_user_profile_info(&client, &profile_dump.user_id).await;
    println!("Username: {}", profile.user.username);

    RecursiveDump::start(&RecursiveDump{ client, profile, profile_dump }).await;

    Ok(())
}


