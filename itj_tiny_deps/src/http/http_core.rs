use crate::errors::ErrorSmart;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpHeader {
	pub name: String,
	pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpGetResponse {
	pub body: String,
	#[allow(unused)] // TODO
	pub headers: Vec<HttpHeader>,
}

/// A trivial abstraction layer for making HTTP requests
pub trait Http: Debug {
	/// Perform an HTTP GET request on the given URL
	fn get(&self, url: &str) -> Result<HttpGetResponse, ErrorSmart>;
}

#[must_use]
pub fn get_header_value<'a>(headers: &'a Vec<HttpHeader>, header_name: &str) -> Option<&'a str> {
	for header in headers {
		if header.name == header_name {
			return Some(&header.value);
		}
	}
	None
}
