use crate::Result;
pub use crate::io::{PasswordStep, password_prompt};
use crate::{hash,LoginResult};
use addr::{Email, Host};
use promptly::{prompt,prompt_default};
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

async fn send_json(user_json: String) -> Result<bool> {
	let client = reqwest::Client::new();
	let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");

	println!("{:?}", &user_json);

	let res = client.post(&server_url)
		//.body(user_json)
		.body("hi!")
		.send()
		.await?;

	println!("{:?}", &res);

	let user_registered = true;

	Ok(user_registered)
}

async fn is_username_invalid(username: &String) -> Result<bool> {
	if reqwest::get(&("https://www.purgomalum.com/service/containsprofanity?text=".to_owned() + &username))
		.await?
		.text()
		.await? == "true"{
			return Ok(true);
		}

/*
	let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
	let duplicate_checker = client.post(&server_url)
		//.body(user_json)
		.body("hi!")
		.send()
		.await?;
*/
	Ok(false)
}



pub async fn signup() -> Result<LoginResult> {
	loop {
		print!("\x1B[2J");
		let username: String = prompt("Please enter your username");
		
		if is_username_invalid(&username).await.unwrap() { //will be turned into a function that checks if someone has the username or not
			prompt_default("Username taken!", true);
			continue;
		}

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
				if send_json(user_json).await? {
					return Ok(LoginResult::SignedUp)
				}
				prompt_default("The email you entered is already being used. Please enter another email!", true);
				continue;
			}	
		}
		prompt_default("The email you entered is not valid. Please enter another email!", true);
	}
}