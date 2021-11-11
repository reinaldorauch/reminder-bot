
use std::fmt;

use Deserialize;
use chrono::{TimeZone, Utc};

#[derive(Deserialize)]
pub struct ChatPhoto {

}

impl fmt::Display for ChatPhoto {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "")
  }
}

#[derive(Deserialize)]
pub struct Chat {
  id: u64,
  r#type: String, // "private" | "group" | "supergroup" | "channel"
  title: Option<String>,
  username: Option<String>,
  first_name: Option<String>,
  last_name: Option<String>,
  photo: Option<ChatPhoto>,
  bio: Option<String>,
  description: Option<String>,
  invite_link: Option<String>,
  //pinned_message: Message,

}

impl fmt::Display for Chat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "")
  }
}

#[derive(Deserialize)]
pub struct User {
  id: u64,
  is_bot: bool,
  first_name: String,
  last_name: Option<String>,
  username: Option<String>,
  language_code: Option<String>,
  can_join_groups: Option<bool>,
  can_read_all_group_messages: Option<bool>,
  supports_inline_queries: Option<bool>
}

impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let is_bot = match self.is_bot {
      true => "bot",
      false => "not bot"
    };
    let empty = String::from("");
    let last_name = match &self.last_name {
      Some(n) => n,
      None => &empty
    };
    let username = match &self.username {
      Some(n) => n,
      None => &empty
    };
    let language_code = match &self.language_code {
      Some(lc) => format!("LANG: {}", lc),
      None => format!("")
    };
    let can_join_groups = match self.can_join_groups {
      Some(b) => match b {
        true => String::from("can join groups"),
        false => String::from("cannot join groups")
      },
      None => format!("")
    };
    let can_read_all_group_messages = match self.can_read_all_group_messages {
      Some(b) => match b {
        true => String::from("can read all group message"),
        false => String::from("cannot read all group message")
      },
      None => format!("")
    };   
    let supports_inline_queries = match self.supports_inline_queries {
      Some(b) => match b {
        true => String::from("supports inline queries"),
        false => String::from("dont supports inline queries")
      },
      None => format!("")
    };
    write!(f, "#{}, {} {} {}; {}; {}; features: {}, {}, {}", self.id, is_bot, self.first_name, last_name, username, language_code, can_join_groups, can_read_all_group_messages, supports_inline_queries)
  }
}

#[derive(Deserialize)]
pub struct Message {
  message_id: u64,
  from: Option<User>,
  sender_chat: Option<Chat>,
  date: i64, 
  chat: Chat,
  forward_from: Option<User>,
  forward_from_chat: Option<Chat>,
  forward_from_message_id: Option<u64>,
  forward_signature: Option<String>,
  forward_sender_name: Option<String>,
  forward_date: Option<i64>

}

impl fmt::Display for Message {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let from = match &self.from {
      Some(u) => format!("\nUser: {}", u),
      None => String::from("")
    };
    let sender_chat = match &self.sender_chat {
      Some(sc) => format!("\nSender Chat: {}", sc),
      None => String::from("")
    };
    write!(f, "#{}{}{}{}", self.message_id, from, sender_chat, Utc.timestamp(self.date, 0).format("%c"))
  }
}

#[derive(Deserialize)]
pub struct Update {
  update_id: u64,
  message: Message,
  edited_message: Message,
}

impl fmt::Display for Update {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "#{}\nMessage: {}\nEdited message: {}", self.update_id, self.message, self.edited_message)
  }
}