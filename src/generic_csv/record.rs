use crate::DELIMITER;

#[derive(Debug)]
pub struct Record {
	pub items: Vec<String>,
}

impl Record {
	pub fn from_wt_string(raw: &str) -> Self {
		let raw_fields = raw.split(DELIMITER).collect::<Vec<&str>>();

		let sanitized = raw_fields.iter()
			.map(|x|x.replace("\"", ""))
			.collect::<Vec<String>>();

		Self {
			items: sanitized,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::generic_csv::record::Record;

	#[test]
	fn test_wt_string() {
		Record::from_wt_string(r#""country_germany";"Germany";"Allemagne";"Germania";"Deutschland";"Alemania";"Германия";"Niemcy";"Německo";"Almanya";"德\t国";"ドイツ";"Alemanha";"Німеччина";"Nemačka";"Németország";"독일";"Германія";"Germania";"德\t國";"D\t系";;"#);
	}
}