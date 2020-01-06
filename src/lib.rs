pub use error::Error;
pub use error::Result;
use argonautica::Hasher;

#[derive(Debug)]
pub enum LoginResult {
	AuthCode(String),
	Helped,
	Quit,
	SignedUp,
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

pub mod error;
mod io;

//these 4 modules might be put under one module (options) later on
pub mod abort;
pub mod help;
pub mod signin;
pub mod signup;
