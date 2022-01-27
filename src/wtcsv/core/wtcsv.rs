use std::collections::HashMap;

use crate::DELIMITER;
use crate::wtcsv::header::Header;
use crate::wtcsv::record::Record;

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct WTCSV {
	pub base_file: String,
	pub header: Header,
	pub records: Vec<Record>,
}

impl WTCSV {
	/// Creates a record from a supported file
	pub fn new_from_file(file: String) -> Result<Self, String> {
		let header = Header::from_file(&file)?;

		let mut records: Vec<String> = Vec::new();

		let mut delim_count = 0; // Amount of delimiters ";" encountered since last buffer flush
		let mut buffer = "".to_owned();

		for char in file.chars() {
			buffer.push(char);

			// Subtracting one as there is always one less delimiter compared to headers
			if delim_count == header.len - 1 && char == '\n' { // End of record is indicated by \n as CLRF terminates with that
				// Cropping away the last two chars as they are CLRF - \r\n chars
				let new_buffer = buffer.clone()[..buffer.len() - 2].to_owned();
				records.push(new_buffer);
				buffer.clear();
				delim_count = 0;
			}

			if char == ';' {
				delim_count += 1;
			}
		}

		let mut wtcsv = Self {
			base_file: "".to_owned(),
			header,
			records: Vec::new(),
		};


		for record in records {
			wtcsv.insert_record(&record)?;
		}

		// The first record would be the header as it is not specifically distinguished
		wtcsv.records.remove(0);

		wtcsv.base_file = file;
		Ok(wtcsv)
	}

	/// Inserts record into struct from file, returns result if the process was successful
	pub fn insert_record(&mut self, record: &str) -> Result<(), String> {
		let serialized_record = Record::from_wt_string(record);
		if self.header.len == serialized_record.items.len() {
			self.records.push(serialized_record);
			Ok(())
		} else {
			Err(format!("Expected record header length of {}, found actual length of {}", self.header.len, serialized_record.items.len()))
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

			// Appending CLRF
			str_record.push_str("\r\n");

			file.push_str(&str_record);
		}

		file
	}

	pub fn edit_record_by_id(&mut self, id: &str, new_target: &str) -> Result<(), &str> {
		// Creating static baseline that does not affect &mut self
		let baseline = self.clone();

		// Using hashmap for performance
		let mut map: HashMap<&str, usize> = HashMap::new();
		for (i, record) in baseline.records.iter().enumerate() {
			map.insert(&record.items[0], i);
		}

		match map.get(id) {
			Some(target) => {
				for (i, _) in baseline.records[*target].items.iter().enumerate() {
					if i != 0 {
						self.records[*target].items[i] = new_target.to_string();
					}
				}
				Ok(())
			}
			None => {
				Err("Provided ID is not present in file")
			}
		}
	}
}

#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use std::fs;

	use crate::wtcsv::core::wtcsv::WTCSV;

	#[test]
	#[allow(unused_variables)]
	fn core_insert() {
		let file = fs::read_to_string("lang/units.csv").unwrap();

		let wtcsv = WTCSV::new_from_file(file).unwrap();
	}

	#[test]
	#[allow(unused_variables)]
	fn core_to_file() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let wtcsv = WTCSV::new_from_file(file.clone()).unwrap();

		assert_eq!(file, wtcsv.export_to_file())
	}

	#[test]
	#[allow(unused_variables)]
	fn edit_record() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let mut wtcsv = WTCSV::new_from_file(file.clone()).unwrap();

		wtcsv.edit_record_by_id("country_china", "west-taiwan").unwrap();

		let export = wtcsv.export_to_file();

		assert_eq!(fs::read_to_string("lang_edit/_common_languages.csv").unwrap(), export);
	}
}