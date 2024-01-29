use std::fs::File;
use reqwest::Client;
use std::io::Write;
use regex::Regex;
use serde_json::from_str;

use crate::dumper_builder::Dump;
use crate::message::MessageStruct;
use crate::profile_struct::Profile;
use crate::web_client::Token;

pub struct RecursiveDump {
    pub(crate) client: Client,
    pub(crate) profile: Profile,
    pub(crate) profile_dump: Dump
}

impl RecursiveDump {
    pub async fn start(&self) {
        let mut msg_dump: Vec<String> = vec![];
        let mut last_message_id = String::new();
        let mut tmp_last_message_id = String::new();

        if let Some(txt) = dm_dump_first_request(&self.client, &self.profile_dump.channel_id).await {
            let msg_array = parse_msg(txt, &mut last_message_id).unwrap();
            msg_dump.extend(msg_array);

            while tmp_last_message_id != last_message_id {
                tmp_last_message_id = last_message_id.clone();
                if let Some(txt) = dm_dump_recursion(&self.client, &self.profile_dump.channel_id, tmp_last_message_id.clone()).await {
                    println!("Nb msg dump: {}", msg_dump.len());
                    let msg_array = parse_msg(txt, &mut last_message_id).unwrap();
                    msg_dump.extend(msg_array);
                }
            }

            msg_dump.reverse();

            let mut dump_file = File::create(format!("dump/{}.txt", self.profile.user.username)).unwrap();
            for x in msg_dump {
                write!(dump_file, "{}", x).unwrap();
            }
        }
    }
}

fn parse_msg(txt: String, x1: &mut String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut msg_vec: Vec<String> = vec![];
    match from_str::<MessageStruct>(&*txt) {
        Ok(parse) => {
            for msg in &parse {
                *x1 = msg.id.clone();
                let x = format!("({}) {}: {}\n", msg.timestamp, msg.author.username, msg.content);
                msg_vec.push(x);
            }
        }
        Err(e) => {
            let re = Regex::new(r"line (\d+) column (\d+)").unwrap();

            if let Some(captures) = re.captures(e.to_string().as_str()) {
                if let (Some(line), Some(column)) = (captures.get(1), captures.get(2)) {
                    let line_num = line.as_str().parse::<usize>().unwrap();
                    let column_num = column.as_str().parse::<usize>().unwrap();
                    eprintln!("Error parsing json: Line {}, Column {}", line_num, column_num);
                    if column_num == 0 {
                        eprintln!("Text: {}", txt);
                    } else {
                        let start_index = column_num.saturating_sub(25);
                        let end_index = column_num.saturating_add(25).min(txt.len());
                        eprintln!("{}", &txt[start_index..end_index]);
                    }
                }
            } else {
                eprintln!("Error: {}", e);
                eprintln!("Text: {}", txt);
            }
        }
    };
    Ok(msg_vec)
}

async fn dm_dump_first_request(client: &Client, channel_id: &String) -> Option<String> {
    Token::http(&client, format!("https://discord.com/api/v9/channels/{}/messages?limit=100", channel_id)).await
}

async fn dm_dump_recursion(client: &Client, channel_id: &String, last_message_id: String) -> Option<String> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages?before={}", channel_id, last_message_id);
    Token::http(&client, url).await
}