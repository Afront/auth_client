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
