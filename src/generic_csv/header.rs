use crate::{DELIMITER, RECORD_SEP};

#[derive(Debug)]
pub struct Header {
	pub len: usize,
	pub params: Vec<String>,
}

impl Header {
	pub fn from_file(file: &str) -> Result<Self, &str> {
		let header = file.split(RECORD_SEP).collect::<Vec<&str>>()[0];

		let headers = header.split(DELIMITER).collect::<Vec<&str>>();

		if headers.len() <= 1 {
			Err("Only one or less headers could be identified")
		} else {
			Ok(Self {
				len: headers.len(),
				params: headers.iter().map(|x|sanitize_header(x)).collect::<Vec<String>>(),
			})
		}
	}
}

pub fn sanitize_header(raw: &str) -> String {
	raw
		.replace("\"", "")
		.replace("<", "")
		.replace(">", "")
		.replace("\r", "")
		.split("|").collect::<Vec<&str>>()[0].to_string()
}

#[cfg(test)]
mod tests {
	use std::fs;
	use super::*;

	#[test]
	fn test_header_to_file() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();

		let header = Header::from_file(&file);

		eprintln!("header = {:?}", header);
	}
}