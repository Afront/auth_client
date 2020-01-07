use crate::{Error,Result};
use crate::io;
pub use crate::io::{LoginStep, password_prompt};
use crate::{LoginResult};
use addr::{Email, Host};
use promptly::{prompt};
use std::env;
use serde::{Deserialize, Serialize};
use validators::email::Email as Validator;

#[derive(Serialize, Deserialize)]
struct User {
	username: String,
	email: String,
	password: String,
}

#[derive(Serialize, Deserialize)]
struct Username {
	username: String,
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

	return Ok(client.post(&server_url)
		//.body(user_json), commented to prevent leaking private data
		.body("hi")
		.send()
		.await?.text().await? == "true")
}

async fn username_prompt() -> Result<String> {
	let client = reqwest::Client::new();
	let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
	//add URL validator...
	
	loop {
	 	let username: String = prompt("Please enter your username");
	 	let username_backup = username.clone();

	 	if reqwest::get(&("https://www.purgomalum.com/service/containsprofanity?text=".to_owned() + &username))
			.await?
			.text()
			.await? == "false"{
				print!("\x1B[2J");
				
				if client.post(&server_url)
				.body(username)
				.send()
				.await?
				.text()
				.await? == "false" {
					return Ok(username_backup)
				}
		}

	 	print!("\x1B[2J");
		println!("Username taken!");
	}
}

async fn email_prompt() -> Result<String> {
	loop {
		let email: String = prompt("Please enter your email");
		
		if validate_email(&email).await.unwrap() {
			return Ok(email)
		}
		
		print!("\x1B[2J");
		println!("The email you entered is not valid. Please enter another email!");
	} 	
}


pub async fn signup() -> Result<LoginResult> {
	loop {
//		print!("\x1B[2J");

		let user = User {
					username: username_prompt().await?,
					email: email_prompt().await?,
					password: password_prompt(LoginStep::SignUp).unwrap(),	
		};
/*
		if validate_email(&user.email).await.unwrap() {
			let password = password_prompt(PasswordStep::First);
			if password == password_prompt(PasswordStep::Second) {
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
*/	}

}