use crate::generic_csv::header::Header;
use crate::generic_csv::record::Record;

pub struct WTCSV {
	pub header: Header,
	pub records: Vec<Record>,
}

impl WTCSV {
	/// Creates an empty record with initialized header fields.
	pub fn new_from_file(file: &str) -> Result<Self, String> {
		let header = Header::from_file(file)?;
		Ok(Self {
			header,
			records: vec![],
		})
	}

	/// Inserts record into struct from file, returns result if  the process was successful
	pub fn insert_record(&mut self, record: &str) -> Result<(), String> {
		let serialized_record = Record::from_wt_string(record);
		if self.header.len == serialized_record.items.len() {
			self.records.push(serialized_record);
			Ok(())
		} else {
			Err(format!("Record length of {} does not match header definition length of {}", serialized_record.items.len(), self.header.len))
		}
	}
}