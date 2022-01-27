use crate::wtcsv::core::wtcsv::WTCSV;

/// Returns this enum when the two files are incompatible, the contents of enums always refer to other
#[derive(Debug)]
pub enum SameErr {
	HeaderLen(usize),
	HeaderContent(String),
	RecordLength(usize),
}

impl WTCSV {
	pub fn is_same(&self, other: &Self) -> Result<(), SameErr> {
		// Checking header length first
		if self.header.params.len() != other.header.params.len() {
			return Err(SameErr::HeaderLen(other.header.params.len()));
		}

		// Both CSVs must have an equal amount of records
		if self.records.len() != other.records.len() {
			return Err(SameErr::RecordLength(other.records.len()))
		}

		// Verifying if headers are matching and in order
		for (i, param) in self.header.params.iter().enumerate() {
			if other.header.params[i] != *param {
				return Err(SameErr::HeaderContent(param.clone()));
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
			Err(SameErr::RecordLength(35)) => {
			}
			_ => {
				panic!("See test for more details")
			}
		}
	}
}