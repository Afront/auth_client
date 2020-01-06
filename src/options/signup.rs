use crate::Result;
pub use crate::io::{PasswordStep, password_prompt};
use crate::{hash,LoginResult};
use addr::{Email, Host};
use promptly::prompt;
use reqwest::Response;
use std::env;
use serde::{Deserialize, Serialize};
use validators::email::Email as Validator;

#[derive(Serialize, Deserialize)]
struct User {
	username: String,
	email: String,
	password: String,
}

async fn validate_email(email: &String) -> Result<bool> {
	return match mailchecker::is_valid(&email) {
		true => match &email.parse::<Email>()?.host() {
			Host::Domain(name) => match name.root().suffix().is_known() {
				true => Ok(Validator::into_string(Validator::from_str(&email)?) == *email),
				false => Ok(false),
			},
			Host::Ip(_) => Ok(true),
		},
		false => Ok(false),
	}
}


async fn send_json(user_json: String) -> Result<Response> {
	let client = reqwest::Client::new();
	let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");

	println!("{:?}", &user_json);

	let res = client.post(&server_url)
		.body("hi!")
		.send()
		.await?;

	println!("{:?}", &res);

	Ok(res)
}

pub async fn signup() -> Result<LoginResult> {
	loop {
		print!("\x1B[2J");
		let username: String = prompt("Please enter your username");
		let email: String = prompt("Please enter your email");

		if validate_email(&email).await.unwrap() {
			let password = password_prompt(PasswordStep::First);
			if password == password_prompt(PasswordStep::Second) {
				let user = User {
					username: username,
					email: email,
					password: hash(password)	
				};
				let user_json = serde_json::to_string(&user)?;
				println!("{:?}", user_json);
				send_json(user_json).await?;
				return Ok(LoginResult::SignedUp)
			}	
		}
	}
}