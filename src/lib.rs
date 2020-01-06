use self::error::Error;
use std::env;
//	use std::io::Write;
use argonautica::Hasher;
use rpassword::read_password;

pub mod error {
	use crate::LoginResult;

	#[derive(Debug)]
	pub enum Error {
		Login(LoginResult),
		Reqwest(reqwest::Error),
		SerdeJSON(serde_json::error::Error),
	}

	impl std::convert::From<reqwest::Error> for Error {
		fn from(err: reqwest::Error) -> Error {
			Error::Reqwest(err)
		}
	}

	impl std::convert::From<serde_json::Error> for Error {
		fn from(err: serde_json::Error) -> Error {
			Error::SerdeJSON(err)
		}
	}

}

#[derive(Debug)]
pub enum LoginResult {
	AuthCode(String),
	Helped,
	Quit,
	SignedUp,
}

mod io {
	use std::io::Write;

	#[derive(Debug, PartialEq)]
	pub enum PasswordStep {
		First,
		Second
	}

	pub fn password_prompt(choice: PasswordStep) -> String {
		print!("Please enter your password{}: " , if choice == PasswordStep::Second {" again"} else {""});
		
		std::io::stdout().flush().unwrap();
		rpassword::read_password().unwrap()
	}	
}


pub mod abort {
	use crate::error::Error;
	use crate::LoginResult;

	pub fn abort() -> Result<LoginResult, Error>{
		println!("See you next time!");
		Ok(LoginResult::Quit)
	}
}

pub mod help {
	use crate::error::Error;
	use crate::LoginResult;

	pub fn help() -> Result<LoginResult, Error> {
		let something_wrong_happened = false;
		println!("You need help? I don't think I'm the right person to ask. Try calling someone on your phone.");

		match something_wrong_happened {
			true => Err(Error::Login(LoginResult::Helped)),
			false => Ok(LoginResult::Helped)
		}
	}
}


pub mod signin {
	use crate::error::Error;
	use crate::io::{PasswordStep, password_prompt};
	use crate::{hash,LoginResult};
	use promptly::prompt;
	use serde::{Deserialize, Serialize};

	#[derive(Serialize, Deserialize)]
	struct OldUser {
		id: String,
		password: String,
	}
	
	pub async fn signin() -> Result<LoginResult, Error> {
		loop {
			print!("\x1B[2J");
			let id: String = prompt("Please enter your username or your email");

			let user = OldUser {
				id: id,
				password: hash(password_prompt(PasswordStep::First))	
			};
			let user_json = serde_json::to_string(&user)?;
			println!("{:?}", user_json);
//				send_json(user_json).await?;
			break; //For testing, to prevent warning since this part is not finished yet
		}
		Ok(LoginResult::AuthCode(String::from("some_auth_code")))
	}	
}


pub mod signup {
	use crate::error::Error;
	use crate::io::{PasswordStep, password_prompt};
	use crate::{hash,LoginResult};
	use promptly::prompt;
	use reqwest::Response;
	use std::env;
	use serde::{Deserialize, Serialize};

	#[derive(Serialize, Deserialize)]
	struct NewUser {
		username: String,
		email: String,
		password: String,
	}

	pub async fn send_json(user_json: String) -> Result<Response, Error> {
		let client = reqwest::Client::new();
		let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");

		let res = client.post(&server_url)
			.body("hi!")
			.send()
			.await?;

		println!("{:?}", &user_json);
		println!("{:?}", &res);

		Ok(res)
	}

	pub async fn signup() -> Result<LoginResult, Error> {
		loop {
			print!("\x1B[2J");
			let username: String = prompt("Please enter your username");
			let email: String = prompt("Please enter your email");

			let password = password_prompt(PasswordStep::First);
			if password == password_prompt(PasswordStep::Second) {
				let user = NewUser {
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


pub fn hash(password: String) -> String {
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
