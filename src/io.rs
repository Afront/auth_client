pub use error::Error;

use crate::Result;
use addr::{Email, Host};
use argonautica::Hasher;
use promptly::{prompt};
use std::io::Write;
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

//Helper functions
fn hash(password: String) -> String {
	let mut hasher = Hasher::default();
	let hash = hasher
		.with_password(password)
		.with_salt("this will not be the actual salt")
		.with_secret_key("this will not be the secret key, just a placeholder")
		.hash()
		.unwrap();
	hash
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

//Technically an IO function
pub async fn send_json(user_json: String, url: &String) -> Result<reqwest::Response> {
	let client = reqwest::Client::new();

	return Ok(client.post(url)
		.body(user_json)
		.send()
		.await?)
}

//Prompts
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

pub async fn username_prompt(url: &String) -> Result<String> {
	let client = reqwest::Client::new();
	
	loop {
	 	let username: String = prompt("Please enter your username");
	 	let username_backup = username.clone();

	 	if reqwest::get(&("https://www.purgomalum.com/service/containsprofanity?text=".to_owned() + &username))
			.await?
			.text()
			.await? == "false"{
				if client.post(url)
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