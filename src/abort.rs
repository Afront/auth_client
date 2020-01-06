use crate::error::Error;
use crate::LoginResult;

pub fn abort() -> Result<LoginResult, Error>{
	println!("See you next time!");
	Ok(LoginResult::Quit)
}