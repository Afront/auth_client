use crate::LoginResult;

#[derive(Debug)]
pub enum Error {
	Login(LoginResult),
	Reqwest(reqwest::Error),
	SerdeJSON(serde_json::error::Error),
}

impl std::convert::From<reqwest::Error> for Error {
	fn from(err: reqwest::Error) -> Error {
		Error::Reqwest(err)
	}
}

impl std::convert::From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Error {
		Error::SerdeJSON(err)
	}
}

