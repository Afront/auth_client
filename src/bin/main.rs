use login_screen::options::*;
use login_screen::options::LoginResult;
use login_screen::options::abort::abort;
use login_screen::options::help::help;
use login_screen::options::signup::signup;
use login_screen::options::signin::signin;

use login_screen::options::error::Error;
use std::{io,process};
use std::io::Write;
use argonautica::Hasher;
use promptly::{prompt}; //use promptly::{prompt, prompt_default};
use reqwest::Response;
use rpassword::read_password;
use serde::{Deserialize, Serialize};

async fn login_screen() -> Result<LoginResult, Error>{
	loop {
		print!("\x1B[2J");
		print!("Hello! Would you like to (R)egister or (S)ign in? ");
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input)
				.expect("Failed to read line");
		match match input.trim().to_uppercase().as_str() {
				"ABORT" | "EXIT" | "Q" | "QUIT" => abort(),
				"HELP" | "H" => help(),
				"SIGN UP" | "SIGNUP" | "REGISTER" | "R" => signup().await,
				"SIGN IN" | "SIGNIN" | "LOGIN" | "LOG IN" | "S" => signin().await,
				_  => continue,
		} {
			Ok(LoginResult::Quit) => process::exit(0),
			Ok(LoginResult::Helped) => continue,
			Ok(LoginResult::SignedUp) => continue,
			Ok(LoginResult::AuthCode(auth_code)) => return Ok(LoginResult::AuthCode(auth_code)),
			Err(err) => println!("{:?}", err),
		};
	}
}

/**
fn do_something(){

}
**/

#[tokio::main]
async fn main() -> Result<(),Error> {
	match login_screen().await {
		Ok(_) => (),
		Err(_) => (),
	}

	Ok(())
}
