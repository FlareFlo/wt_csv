use crate::wtcsv::core::wtcsv::WTCSV;

#[derive(Debug)]
pub struct Diff {
	pub id: String,
	pub old: String,
	pub new: String,
}

impl Diff {
	pub fn diff_from_ref(left: &WTCSV, right: &WTCSV) -> Result<Vec<Self>, String> {
		left.is_same(right)?;

		let mut diffs: Vec<Self> =  Vec::new();

		for (i, record) in left.records.iter().enumerate() {
			for (j, item) in record.items.iter().enumerate() {
				if right.records[i].items[j] != *item {
					diffs.push(Diff {
						id: record.items[0].clone(),
						old: item.clone(),
						new: right.records[i].items[j].clone() ,
					});
					break;
				}
			}
		}

		Ok(diffs)
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use crate::diff::Diff;
	use crate::wtcsv::core::wtcsv::WTCSV;

	#[test]
	fn expect_diff() {
		let init = fs::read_to_string("lang/_common_languages.csv").unwrap();
		let init = WTCSV::new_from_file(init).unwrap();

		let mut diff = init.clone();
		diff.edit_record_by_id("country_britain", "tea").unwrap();

		let diff_res = Diff::diff_from_ref(&init, &diff).unwrap();
		eprintln!("diff_res = {:?}", diff_res);
	}
}