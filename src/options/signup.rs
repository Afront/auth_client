use crate::Result;
pub use crate::io::{LoginStep, email_prompt, password_prompt, username_prompt};
use crate::{LoginResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
	username: String,
	email: String,
	password: String,
}

async fn send_json(user_json: String, url: &String) -> Result<bool> {
	let client = reqwest::Client::new();
	println!("{:?}", &user_json);

	return Ok(client.post(url)
		.body(user_json)
		.send()
		.await?.text().await? == "true")
}

pub async fn signup(url: String) -> Result<LoginResult> {
	//Add url validator?

	print!("\x1B[2J");
	loop {
		let user = User {
					username: username_prompt(&url).await?,
					email: email_prompt().await?,
					password: password_prompt(LoginStep::SignUp).unwrap(),	
		};

		let user_json = serde_json::to_string(&user)?;
		println!("{:?}", user_json);

		if send_json(user_json, &url).await? {
			return Ok(LoginResult::SignedUp)
		}

		print!("\x1B[2J");
		println!("The email you entered is not valid. Please enter another email!");
	}	
}