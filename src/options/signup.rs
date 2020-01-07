use crate::Result;
pub use crate::io::{LoginStep, email_prompt, password_prompt, username_prompt};
use crate::{LoginResult};
use std::env;
use serde::{Deserialize, Serialize};

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



pub async fn signup() -> Result<LoginResult> {
	print!("\x1B[2J");
	loop {
		let user = User {
					username: username_prompt().await?,
					email: email_prompt().await?,
					password: password_prompt(LoginStep::SignUp).unwrap(),	
		};

		let user_json = serde_json::to_string(&user)?;
		println!("{:?}", user_json);

		if send_json(user_json).await? {
			return Ok(LoginResult::SignedUp)
		}

		print!("\x1B[2J");
		println!("The email you entered is not valid. Please enter another email!");
	}	
}
	

