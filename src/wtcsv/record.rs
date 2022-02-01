use crate::DELIMITER;

#[derive(Debug, Clone)]
pub struct Record {
	pub items: Vec<String>,
}

impl Record {
	#[must_use]
	pub fn from_wt_string(raw: &str) -> Self {
		let raw_fields = raw.split(DELIMITER).collect::<Vec<&str>>();

		let sanitized = raw_fields.iter()
			.map(|x|
				if x.is_empty() {
					(*x).to_string()
				} else {
					// Cropping away the leading and ending quotes
					let mut base_str = (*x).to_string();
					base_str.remove(0);
					base_str.pop();
					base_str
				}
			)
			.collect::<Vec<String>>();

		Self {
			items: sanitized,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::wtcsv::record::Record;

	#[test]
	fn test_wt_string() {
		let _ = Record::from_wt_string(r#""country_germany";"Germany";"Allemagne";"Germania";"Deutschland";"Alemania";"Германия";"Niemcy";"Německo";"Almanya";"德\t国";"ドイツ";"Alemanha";"Німеччина";"Nemačka";"Németország";"독일";"Германія";"Germania";"德\t國";"D\t系";;"#);
	}
}