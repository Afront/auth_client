use login_screen::LoginResult;
use login_screen::abort::abort;
use login_screen::help::help;
use login_screen::signup::signup;
use login_screen::signin::signin;
use login_screen::error::Error;

use promptly::{prompt}; //use promptly::{prompt, prompt_default};
use std::process;

async fn login_screen() -> Result<LoginResult, Error>{
	loop {
		print!("\x1B[2J");
		let input: String = prompt("Hello! Would you like to (R)egister or (S)ign in? ");
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
