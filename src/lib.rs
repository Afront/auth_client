pub use error::Error;
pub use error::Result;

#[derive(Debug)]
pub enum LoginResult {
	AuthCode(String),
	Helped,
	Quit,
	SignedUp,
}

pub mod error;
pub mod io;

pub mod options { 
	pub use abort::abort;
	pub use help::help;
	pub use signin::signin;
	pub use signup::signup;
	
	pub mod abort;
	pub mod help;
	pub mod signin;
	pub mod signup;
}
