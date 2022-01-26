use std::fmt::format;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use crate::{DELIMITER, RECORD_SEP};
use crate::generic_csv::header::Header;
use crate::generic_csv::record::Record;

#[derive(Debug)]
pub struct WTCSV {
	pub base_file: String,
	pub header: Header,
	pub records: Vec<Record>,
}

impl WTCSV {
	/// Creates an empty record with initialized header fields.
	pub fn new_from_file(file: String) -> Result<Self, String> {
		let header = Header::from_file(&file)?;

		let mut records: Vec<String> = Vec::new();

		let mut delim_count = 0;
		let mut buffer = "".to_owned();

		for char in file.chars() {
			buffer.push(char);

			if delim_count == header.len - 1 && char == '\n'  {
				// Cropping away the last two chars as they are CLRF - \r\n chars
				let new_buffer = buffer.clone()[..buffer.len() - 2].to_owned();
				records.push(new_buffer);
				buffer.clear();
				delim_count = 0;
			}

			if char == ';' {
				delim_count += 1;
			}
			// println!("Delim: {} Buffer: {}", delim_count, buffer);
		}

		let mut wtcsv = Self {
			base_file: "".to_owned(),
			header,
			records: Vec::new(),
		};


		for record in records {
			wtcsv.insert_record(&record)?;
		}

		wtcsv.base_file = file;
		Ok(wtcsv)
	}

	/// Inserts record into struct from file, returns result if  the process was successful
	pub fn insert_record(&mut self, record: &str) -> Result<(), String> {
		let serialized_record = Record::from_wt_string(record);
		if self.header.len == serialized_record.items.len() {
			self.records.push(serialized_record);
			Ok(())
		} else {
			println!("Failed record length {}", record);
			Err(format!("Expected record length of {}, found actual length of {}", self.header.len, serialized_record.items.len()))
		}
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use std::thread::sleep;
	use std::time::{Duration, Instant};

	use crate::generic_csv::core::WTCSV;

	#[test]
	fn test_core_insert() {
		let file = fs::read_to_string("lang/units.csv").unwrap();

		let start = Instant::now();

		let wtcsv = WTCSV::new_from_file(file).unwrap();

		eprintln!("start.elapsed() = {:?}", start.elapsed());

		// eprintln!("wtcsv = {:#?}", wtcsv.records);
	}
}