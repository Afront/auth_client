use crate::{LoginResult, Result};
use crate::Error::Login;
pub use crate::io::{LoginStep, password_prompt, send_json};
use promptly::prompt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
	id: String,
	password: String,
}

pub async fn signin(url: String) -> Result<LoginResult> {
	loop {
		print!("\x1B[2J");
		let id: String = prompt("Please enter your username or your email");

		let user = User {
			id: id,
			password: password_prompt(LoginStep::SignIn).unwrap()
		};
		let user_json = serde_json::to_string(&user)?;
		let response = send_json(user_json, &url).await?;
		
		if response.status().as_u16() == 401 {
			print!("\x1B[2J");
			println!("The id or the password is incorrect.");
			continue;
		}
		
		return match response.status().is_success() {
			true => Ok(LoginResult::AuthCode(String::from(response.text().await?))),
			_ => Err(Login(LoginResult::AuthCode("An error occured: ".to_owned() + response.status().as_str())))
		}		
	}
}