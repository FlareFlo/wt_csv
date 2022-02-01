use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::{CRLF, DELIMITER, RECORD_SEP};
use crate::wtcsv::core::error::WTCSVError;
use crate::wtcsv::header::Header;
use crate::wtcsv::record::Record;

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct WTCSV {
	pub name: String,
	pub header: Header,
	pub records: Vec<Record>,
	pub crlf: bool,
}

impl WTCSV {
	pub fn new_from_path(path: impl AsRef<Path>, name: &str) -> Result<Self, Box<dyn Error>> {
		let file = fs::read_to_string(path)?;
		WTCSV::new_from_file(file, name)
	}

	/// Creates a record from a supported file
	#[must_use]
	pub fn new_from_file(file: String, name: &str) -> Result<Self, Box<dyn Error>> {
		let crlf = file.contains(CRLF);

		let header = Header::from_file(&file)?;

		let mut records: Vec<String> = Vec::new();

		let mut delim_count = 0; // Amount of delimiters ";" encountered since last buffer flush
		let mut buffer = "".to_owned();

		let chars = file.chars().collect::<Vec<char>>();

		for i in 0..chars.len() {
			let char = chars[i];

			buffer.push(char);

			if char == DELIMITER {
				delim_count += 1;
			}

			// Subtracting one as there is always one less delimiter compared to headers
			if delim_count == header.len - 1 && (char == RECORD_SEP) { // End of record is indicated by \n as CLRF terminates with that
				// Cropping away the last two chars as they are CLRF - \r\n chars
				let new_buffer = buffer.clone()[..buffer.len() - if crlf {2 } else {1}].to_owned();
				records.push(new_buffer);
				buffer.clear();
				delim_count = 0;
			}
		}

		let mut wtcsv = Self {
			name: name.to_string(),
			header,
			records: Vec::new(),
			crlf,
		};


		for record in records {
			wtcsv.insert_record(&record)?;
		}

		// The first record would be the header as it is not specifically distinguished
		wtcsv.records.remove(0);

		Ok(wtcsv)
	}

	/// Inserts record into struct from file, returns result if the process was successful
	#[must_use]
	pub fn insert_record(&mut self, record: &str) -> Result<(), Box<dyn Error>> {
		let serialized_record = Record::from_wt_string(record);
		if self.header.len == serialized_record.items.len() {
			self.records.push(serialized_record);
			Ok(())
		} else {
			Err(Box::new(WTCSVError::HeaderLen(self.header.len, serialized_record.items.len())))
		}
	}

	#[must_use]
	pub fn export_to_file(&self) -> String {
		// Taking in raw un-sanitized header for ease of use
		let mut file = self.header.raw_header.clone();

		for record in &self.records {
			// Null values are not quoted
			let quote = |x: &String| if x.is_empty() {
				x.clone()
			} else {
				format!("\"{}\"", x)
			};

			// Combining all individual items into one string record, with the required quotes and delimiter
			let mut str_record = record.items
				.iter()
				.map(quote)
				.collect::<Vec<String>>()
				.join(&DELIMITER.to_string());

			// Appending proper line-feed
			str_record.push_str(&if self.crlf {
				CRLF.to_string()
			} else {
				RECORD_SEP.to_string()
			});

			file.push_str(&str_record);
		}

		file
	}

	#[must_use]
	pub fn edit_record_by_id(&mut self, id: &str, new_target: &str) -> Result<(), Box<dyn Error>> {
		// Creating static baseline that does not affect &mut self
		let baseline = self.clone();

		// Using hashmap for performance
		let mut map: HashMap<&str, usize> = HashMap::new();
		for (i, record) in baseline.records.iter().enumerate() {
			map.insert(&record.items[0], i);
		}

		let result = map.get(id).ok_or(WTCSVError::RecordIdNotFound(id.to_string(), self.name.clone()))?;
		for (i, _) in baseline.records[*result].items.iter().enumerate() {
			if i != 0 {
				self.records[*result].items[i] = new_target.to_string();
			}
		}
		Ok(())
	}

	/// The fastest for smaller files, currently the fastest
	#[must_use]
	pub fn get_record_by_id_vec(&self, id: &str) -> Result<Record, Box<dyn Error>> {
		for record in &self.records {
			if record.items[0] == id {
				return Ok(record.clone());
			}
		}
		Err(Box::new(WTCSVError::RecordIdNotFound(id.to_string(), self.name.clone())))
	}
}

#[cfg(test)]
#[allow(unused_variables, deprecated)]
mod tests {
	#[allow(unused_imports)]
	use std::fs;

	use crate::wtcsv::core::wtcsv::WTCSV;

	#[test]
	fn core_read_all() {
		for dir in fs::read_dir("lang").unwrap().enumerate() {
			let path = dir.1.unwrap().file_name().to_str().unwrap().to_owned();
			if path.contains(".csv") {
				if WTCSV::new_from_path(format!("lang/{}", path), "").is_err() {
					eprintln!("path = {:?}", path);
				}
			}
		}
	}

	#[test]
	fn core_to_file() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let wtcsv = WTCSV::new_from_file(file.clone(), "units").unwrap();

		assert_eq!(file, wtcsv.export_to_file())
	}

	#[test]
	fn edit_record() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let mut wtcsv = WTCSV::new_from_file(file.clone(), "_common_languages").unwrap();

		wtcsv.edit_record_by_id("country_china", "west-taiwan").unwrap();

		let export = wtcsv.export_to_file();

		assert_eq!(fs::read_to_string("lang_edit/_common_languages.csv").unwrap(), export);
	}

	#[test]
	fn get_good_record_vec() {
		let file = fs::read_to_string("lang/units.csv").unwrap();
		let wtcsv = WTCSV::new_from_file(file.clone(), "units").unwrap();

		let result = wtcsv.get_record_by_id_vec("ussr_mpk_201k_2");

		assert_eq!(true, result.is_ok())
	}

	#[test]
	fn get_bad_record_vec() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let wtcsv = WTCSV::new_from_file(file.clone(), "_common_languages").unwrap();

		assert_eq!(true, wtcsv.get_record_by_id_vec("country_fake").is_err())
	}
}