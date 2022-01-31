use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::wtcsv::core::wtcsv::WTCSV;

/// Returns this enum when the two files are incompatible, left is expected right is provided
#[derive(Debug)]
pub enum WTCSVError {
	HeaderLen(usize, usize),
	HeaderContent(String, String),
	HeaderTooShort(usize),
	RecordLength(usize, usize),
	RecordIdNotFound(String, String),
}

impl Display for WTCSVError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			WTCSVError::HeaderLen(left, right) => {write!(f, "Header Length mismatch, expected {left} but found {right}")}
			WTCSVError::HeaderContent(left, right) => {write!(f, "Header content mismatch, expected {left} but found {right}")}
			WTCSVError::HeaderTooShort(left) => {write!(f, "Header too short, expected at least 2 but found {left}")}
			WTCSVError::RecordLength(left, right) => {write!(f, "Record content mismatch, expected {left} but found {right}")}
			WTCSVError::RecordIdNotFound(left, right) => {write!(f, "File {left} does not contain record with id {right}")}
		}
	}
}

impl Error for WTCSVError {

}

impl From<WTCSVError> for String {
	fn from(err: WTCSVError) -> Self {
		err.to_string()
	}
}

impl WTCSV {
	pub fn is_same(&self, other: &Self) -> Result<(), WTCSVError> {
		// Checking header length first
		if self.header.params.len() != other.header.params.len() {
			return Err(WTCSVError::HeaderLen(self.header.params.len(), other.header.params.len()));
		}

		// Both CSVs must have an equal amount of records
		if self.records.len() != other.records.len() {
			return Err(WTCSVError::RecordLength(self.records.len(), other.records.len()))
		}

		// Verifying if headers are matching and in order
		for (i, param) in self.header.params.iter().enumerate() {
			if other.header.params[i] != *param {
				return Err(WTCSVError::HeaderContent(param.clone(), other.header.params[i].clone()));
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use crate::wtcsv::core::error::WTCSVError;
	use crate::wtcsv::core::wtcsv::WTCSV;

	#[test]
	fn test_equal() {
		let units = fs::read_to_string("lang/units.csv").unwrap();

		let wtcsv = WTCSV::new_from_file(units, "units").unwrap();

		wtcsv.is_same(&wtcsv).unwrap()
	}

	#[test]
	// These two are not equal as they have different contents
	fn test_not_equal() {
		let units = fs::read_to_string("lang/units.csv").unwrap();
		let lang = fs::read_to_string("lang/_common_languages.csv").unwrap();

		let wtcsv_units = WTCSV::new_from_file(units, "units").unwrap();
		let wtcsv_lang = WTCSV::new_from_file(lang, "_common_languages").unwrap();

		match wtcsv_units.is_same(&wtcsv_lang) {
			Err(WTCSVError::RecordLength(10432, 35)) => {
			}
			_ => {
				panic!("See test for more details")
			}
		}
	}
}