use crate::wtcsv::core::wtcsv::WTCSV;

/// Returns this enum when the two files are incompatible, left is expected right is provided
#[derive(Debug)]
pub enum SameErr {
	HeaderLen(usize, usize),
	HeaderContent(String, String),
	RecordLength(usize, usize),
}

impl ToString for SameErr {
	fn to_string(&self) -> String {
		match self {
			SameErr::HeaderLen(left, right) => {format!("Header Length mismatch, expected {left} but found {right}")}
			SameErr::HeaderContent(left, right) => {format!("Header content mismatch, expected {left} but found {right}")}
			SameErr::RecordLength(left, right) => {format!("Record content mismatch, expected {left} but found {right}")}
		}
	}
}

impl From<SameErr> for String {
	fn from(err: SameErr) -> Self {
		err.to_string()
	}
}

impl WTCSV {
	pub fn is_same(&self, other: &Self) -> Result<(), SameErr> {
		// Checking header length first
		if self.header.params.len() != other.header.params.len() {
			return Err(SameErr::HeaderLen(self.header.params.len(), other.header.params.len()));
		}

		// Both CSVs must have an equal amount of records
		if self.records.len() != other.records.len() {
			return Err(SameErr::RecordLength(self.records.len(), other.records.len()))
		}

		// Verifying if headers are matching and in order
		for (i, param) in self.header.params.iter().enumerate() {
			if other.header.params[i] != *param {
				return Err(SameErr::HeaderContent(param.to_owned(), other.header.params[i].to_owned()));
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use crate::wtcsv::core::compatibility::SameErr;
	use crate::wtcsv::core::wtcsv::WTCSV;

	#[test]
	fn test_equal() {
		let units = fs::read_to_string("lang/units.csv").unwrap();

		let wtcsv = WTCSV::new_from_file(units).unwrap();

		wtcsv.is_same(&wtcsv).unwrap()
	}

	#[test]
	// These two are not equal as they have different contents
	fn test_not_equal() {
		let units = fs::read_to_string("lang/units.csv").unwrap();
		let lang = fs::read_to_string("lang/_common_languages.csv").unwrap();

		let wtcsv_units = WTCSV::new_from_file(units).unwrap();
		let wtcsv_lang = WTCSV::new_from_file(lang).unwrap();

		match wtcsv_units.is_same(&wtcsv_lang) {
			Err(SameErr::RecordLength(10432, 35)) => {
			}
			_ => {
				panic!("See test for more details")
			}
		}
	}
}