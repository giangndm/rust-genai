use crate::utils;
use derive_more::From;
use reqwest::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	ResponseFailedNotJson {
		content_type: String,
	},

	ResponseFailedStatus {
		status: StatusCode,
		body: String,
	},

	// -- Utils
	#[from]
	XValue(utils::x_value::Error),

	// -- Externals
	#[from]
	Reqwest(reqwest::Error),
	#[from]
	EventSourceClone(reqwest_eventsource::CannotCloneRequestError),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
