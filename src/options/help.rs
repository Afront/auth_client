use crate::Error;
use crate::LoginResult;

pub fn help() -> Result<LoginResult, Error> {
	println!("(H)elp  => Prints this help screen");
	println!("(Q)uit | Abort | Exit  => Exit the app");
	println!("(R)egister | Signup | Signup => Sign up to the app");
	println!("(S)ign in | Signin => Sign in to the app");

	let something_wrong_happened = false;
	
	match something_wrong_happened {
		true => Err(Error::Login(LoginResult::Helped)),
		false => Ok(LoginResult::Helped)
	}
}
