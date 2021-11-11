#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::{Json, Value, json}};
use chrono::{Utc,Locale};

pub mod telegram;

#[post("/webhook", format = "application/json", data = "<update>")]
fn update(update: Json<telegram::Update>) {
  println!("DATA: {}", update.into_inner())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate="rocket::serde")]
struct Message<'a> {
  id: u64,
  message: &'a str,
  timestamp: Option<String>
}

#[post("/test", format="application/json", data="<body>")]
fn test<'a>(body: Json<Message<'a>>) -> Value {
  let message = Message::copy(body.into_inner());
  let timestamp = Utc::now().format_localized("%c", Locale::pt_BR);
  message.timestamp = Some(format!("{}", timestamp));
  json!(message)
}

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, update, test])
}
