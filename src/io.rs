use std::io::Write;
pub use error::Error;
use crate::Result;
use argonautica::Hasher;

use addr::{Email, Host};
use promptly::{prompt};
use std::env;
use serde::{Deserialize, Serialize};
use validators::email::Email as Validator;

pub mod error {
	#[derive(Debug)]
	pub enum Error {
		InvalidEmail,
		InvalidPassword,
		InvalidUsername,
	}
}

#[derive(Debug, PartialEq)]
pub enum LoginStep {
	SignUp,
	SignIn
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


pub async fn email_prompt() -> Result<String> {
	loop {
		let email: String = prompt("Please enter your email");
		
		if validate_email(&email).await.unwrap() {
			return Ok(email)
		}
		
		print!("\x1B[2J");
		println!("The email you entered is not valid. Please enter another email!");
	} 	
}

fn hash(password: String) -> String {
	let mut hasher = Hasher::default();
	let hash = hasher
		.with_password(password)
		.with_salt("this will not be the actual salt")
		.with_secret_key("this will not be the secret key, just a placeholder")
		.hash()
		.unwrap();
	println!("{}", &hash);
	hash
}


pub fn password_prompt(choice: LoginStep) -> Result<String> {
	loop {
		print!("Please enter your password: ");	
		std::io::stdout().flush().unwrap();
		let password = rpassword::read_password().unwrap();
		if choice == LoginStep::SignUp {
			print!("Please enter your password again: ");	
			std::io::stdout().flush().unwrap();
			if password != rpassword::read_password().unwrap() {
				print!("\x1B[2J");
				println!("The passwords you entered do not match!");
				continue;
			}
		}
		return Ok(hash(password))
	}
}

pub async fn username_prompt() -> Result<String> {
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