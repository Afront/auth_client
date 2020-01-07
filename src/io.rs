use std::io::Write;
pub use error::Error;
use crate::Result;
use argonautica::Hasher;

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