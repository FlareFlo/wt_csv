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

			if delim_count == header.len - 1 && char == '\n' {
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

		wtcsv.records.remove(0);

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

	pub fn export_to_file(&self) -> Result<String, String> {
		let mut file = self.header.raw_header.clone();

		for record in &self.records {
			let opt_mark = |x: &String| if !x.is_empty() {
				format!("\"{}\"", x)
			} else {
				x.to_owned()
			};

			let mut str_record = record.items
				.iter()
				.map(opt_mark)
				.collect::<Vec<String>>()
				.join(";");

			str_record.push_str("\r");
			file.push_str(&str_record);
		}

		Ok(file)
	}
}

#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use std::fs;
	use std::time::Instant;

	use crate::generic_csv::core::WTCSV;

	#[test]
	#[allow(unused_variables)]
	fn test_core_insert() {
		let file = fs::read_to_string("lang/units.csv").unwrap();

		let start = Instant::now();

		let wtcsv = WTCSV::new_from_file(file).unwrap();

		eprintln!("start.elapsed() = {:?}", start.elapsed());

		// eprintln!("wtcsv = {:#?}", wtcsv.records);
	}

	#[test]
	#[allow(unused_variables)]
	fn core_to_file() {
		let file = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let wtcsv = WTCSV::new_from_file(file.clone()).unwrap();

		assert_eq!(file, wtcsv.export_to_file().unwrap())
	}
}