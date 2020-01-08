use crate::Error;
use crate::LoginResult;

pub fn help() -> Result<LoginResult, Error> {
	println!("(H)elp  => Prints this help screen");
	println!("(Q)uit | Abort | Exit  => Exit the app");
	println!("Sign (U)p | Register | Signup => Sign up to the app");
	println!("Sign (I)n | Signin | Login | Log in | Log on => Sign in to the app");

	let something_wrong_happened = false;
	
	match something_wrong_happened {
		true => Err(Error::Login(LoginResult::Helped)),
		false => Ok(LoginResult::Helped)
	}
}
