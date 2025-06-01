use crate::errors::ErrorSmart;
use crate::http::http_core::HttpGetResponse;
use crate::http::http_core::HttpHeader;
use crate::http::Http;
use std::process::Command;

#[derive(Debug)]
pub struct Curl {
	user_agent: String,
}

impl Curl {
	pub fn new(user_agent: &str) -> Self {
		Self {
			user_agent: user_agent.to_string(),
		}
	}
}

impl Http for Curl {
	fn get(&self, url: &str) -> Result<HttpGetResponse, ErrorSmart> {
		// if crate::VERBOSE {
		println!("Curl - {url}");
		// }

		let mut cmd = Command::new("curl");
		cmd.arg("--silent")
			.arg("--dump-header")
			.arg("-")
			.arg("--header")
			.arg(format!("User-Agent: {agent}", agent = self.user_agent))
			.arg(url);
		let output = cmd.output().expect("failed to execute process").stdout;

		let Ok(output) = std::str::from_utf8(&output) else {
			return ErrorSmart::new_heavy(format!(
				"Response from {url:?} could not be parsed as UTF-8"
			));
		};

		// For some reason curl uses Windows line endings -_-
		let delimiter = "\r\n\r\n";
		let index = output.find(delimiter).unwrap();
		let headers = &output[..index];
		let body = &output[index + delimiter.len()..];

		let headers: Vec<HttpHeader> = headers
			.lines()
			.skip(1) // First line isn't a header (e.g. `HTTP/2 200`)
			.map(|header| {
				let index = header.find(":").unwrap();
				HttpHeader {
					name: header[..index].to_string(),
					// `+2` skips the `: `.
					value: header[index + 2..].to_string(),
				}
			})
			.collect();

		Ok(HttpGetResponse {
			body: body.to_string(),
			headers,
		})
	}
}
