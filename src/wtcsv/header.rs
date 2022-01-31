use std::error::Error;
use crate::{DELIMITER, RECORD_SEP};
use crate::wtcsv::core::error::WTCSVError;

#[derive(Debug, Clone)]
pub struct Header {
	pub len: usize,
	pub params: Vec<String>,
	pub raw_params: Vec<String>,
	pub raw_header: String,
}

impl Header {
	pub fn from_file(file: &str) -> Result<Self, Box<dyn Error>> {
		let header = file.split(RECORD_SEP).collect::<Vec<&str>>()[0];

		let headers = header.split(DELIMITER).collect::<Vec<&str>>();

		let mut raw_header = header.to_owned();

		// Splitting on the record separator pops it, so we re-append it
		raw_header.push(RECORD_SEP);

		if headers.len() <= 1 {
			Err(Box::new(WTCSVError::HeaderTooShort(1)))
		} else {
			Ok(Self {
				len: headers.len(),
				params: headers.iter().map(|x|sanitize_header(x)).collect(),
				raw_params: headers.iter().map(|x|(*x).to_string()).collect(),
				raw_header,
			})
		}
	}
}

fn sanitize_header(raw: &str) -> String {
	raw
		.replace("\"", "")
		.replace("<", "")
		.replace(">", "")
		.replace("\r", "")
		.split('|').collect::<Vec<&str>>()[0].to_string()
}

#[cfg(test)]
mod tests {
	use std::fs;
	use super::*;

	#[test]
	#[allow(unused_variables)]
	fn test_header_to_file() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();

		let header = Header::from_file(&file);
	}
}