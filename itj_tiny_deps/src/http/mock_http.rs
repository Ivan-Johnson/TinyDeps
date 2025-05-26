use crate::errors::ErrorSmart;
use crate::http::Http;
use crate::http::HttpGetResponse;
use crate::time::MockTime;
use crate::time::Time;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::rc::Rc;
use std::time::Instant;

/// Stores the data for a single mocked function call
#[derive(Debug)]
struct MockHttpDataPoint {
	url: String,
	t_expected: Instant,
	result: Result<HttpGetResponse, ErrorSmart>,
}

/// Stores the data for many mocked function calls
type MockHttpDataset = Rc<RefCell<VecDeque<MockHttpDataPoint>>>;

pub struct MockHttp {
	data: MockHttpDataset,
	time: MockTime,
}

#[derive(Default)]
pub struct MockHttpController {
	data: MockHttpDataset,
	time: MockTime,
}

impl MockHttpController {
	pub fn get_mock_time(&self) -> &MockTime {
		&self.time
	}

	pub fn get_expected_time_of_most_recent_expected_fn_call(&self) -> Option<Instant> {
		let data = self.data.borrow();
		let data = data.front()?;
		Some(data.t_expected)
	}

	pub fn add_expected_fn_call(
		self,
		url: String,
		result: Result<HttpGetResponse, ErrorSmart>,
	) -> MockHttpController {
		let current_time = self.time.now_instant();
		let t_expected = self
			.get_expected_time_of_most_recent_expected_fn_call()
			.unwrap_or(current_time);
		self.add_timed_expected_fn_call(t_expected, url, result)
	}

	pub fn add_timed_expected_fn_call(
		self,
		t_expected: Instant,
		url: String,
		result: Result<HttpGetResponse, ErrorSmart>,
	) -> MockHttpController {
		let data = MockHttpDataPoint {
			url,
			result,
			t_expected,
		};
		self.data.borrow_mut().push_back(data);
		self
	}

	pub fn to_impl(self) -> MockHttp {
		MockHttp {
			data: self.data.clone(),
			time: self.time,
		}
	}
}

impl Http for MockHttp {
	fn get(&self, url: &str) -> Result<HttpGetResponse, ErrorSmart> {
		let next = self
			.data
			.borrow_mut()
			.pop_front()
			.expect("Error: MockHttp's `get` function called too many times");
		let t_actual = self.time.now_instant();
		let t_expected = next.t_expected;
		let mut errors: Vec<String> = vec![];
		if t_actual != t_expected {
			errors.push(format!(
				"HttpMock called at {t_actual:?}, expected {t_expected:?}"
			));
		}
		if next.url != url {
			errors.push(format!(
				"Error: Expected HTTP get of {}, but instead did HTTP get on {}",
				next.url, url
			));
		}
		if errors.len() > 0 {
			drop(next);
			// Even if the test is marked as `should_panic`, the
			// Rust test framework doesn't like it if we panic
			// multiple times in a single test.
			//
			// In order to avoid a second panic during test cleanup,
			// we need to clear the list of expected function calls
			// before panicing the first time.
			self.clear();
			panic!("{errors:?}");
		}
		next.result
	}
}

impl MockHttp {
	pub fn clear(&self) {
		self.data.borrow_mut().clear();
	}
}

impl Debug for MockHttp {
	fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		todo!()
	}
}

fn data_is_clean(data: &MockHttpDataset) -> Result<(), ()> {
	if data.borrow().len() == 0 {
		Ok(())
	} else {
		Err(())
	}
}

impl Drop for MockHttp {
	fn drop(&mut self) {
		data_is_clean(&self.data).expect("Error: was not called enough");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::time::DURATION_SECOND;
	use std::time::Duration;

	const URL1: &str = "url1";
	const URL2: &str = "url2";
	const URL_UNUSED: &str = "url999";
	const RESPONSE1: &str = "response1";
	const RESPONSE2: &str = "response2";

	fn minimal_response_from_body(body: &str) -> HttpGetResponse {
		HttpGetResponse {
			headers: vec![],
			body: body.to_string(),
		}
	}

	fn get_standard_impl() -> (impl Http, MockTime) {
		let controller = MockHttpController::default()
			.add_expected_fn_call(URL1.to_string(), Ok(minimal_response_from_body(RESPONSE1)))
			.add_expected_fn_call(URL2.to_string(), Ok(minimal_response_from_body(RESPONSE2)));
		let time = controller.get_mock_time().shallow_clone();
		(controller.to_impl(), time)
	}

	#[test]
	fn test_happy() {
		let (http, _) = get_standard_impl();

		assert_eq!(Ok(minimal_response_from_body(RESPONSE1)), http.get(URL1));
		assert_eq!(Ok(minimal_response_from_body(RESPONSE2)), http.get(URL2));
	}

	/// Verify the mock obj panics if one of the expected function calls is
	/// not performed
	#[test]
	#[should_panic]
	fn test_missing_call() {
		let (http, _) = get_standard_impl();

		assert_eq!(Ok(minimal_response_from_body(RESPONSE1)), http.get(URL1));
		// skip http.get(URL2)
	}

	/// Verify that trying to request the same URL an extra time panics
	#[test]
	#[should_panic]
	fn test_repeated_call() {
		let (http, _) = get_standard_impl();

		assert_eq!(Ok(minimal_response_from_body(RESPONSE1)), http.get(URL1));
		let _ = http.get(URL1);
	}

	/// Verify that calling URL2 before URL1 panics
	#[test]
	#[should_panic]
	fn test_calls_in_wrong_order() {
		let (http, _) = get_standard_impl();

		let _ = http.get(URL2);
	}

	/// Verify that the obj panics if we try to get a URL it doesn't expect
	/// us to get.
	#[test]
	#[should_panic]
	fn test_unknown_url() {
		let (http, _) = get_standard_impl();

		let _ = http.get(URL_UNUSED);
	}

	/// Verify that the obj panics if try to do the get at the wrong time
	#[test]
	#[should_panic]
	fn test_late() {
		let (http, time) = get_standard_impl();

		assert_eq!(Ok(minimal_response_from_body(RESPONSE1)), http.get(URL1));

		time.sleep(Duration::from_secs(1));
		assert_eq!(Ok(minimal_response_from_body(RESPONSE2)), http.get(URL2));
	}

	/// Verify that the obj panics if try to do the get at the wrong time
	#[test]
	#[should_panic]
	fn test_early() {
		let controller = MockHttpController::default();
		let time = controller.get_mock_time().shallow_clone();
		let controller = controller.add_timed_expected_fn_call(
			time.now_instant() + DURATION_SECOND,
			URL1.to_string(),
			Ok(minimal_response_from_body(RESPONSE1)),
		);
		let http = controller.to_impl();
		let _ = http.get(URL1);
	}
}
