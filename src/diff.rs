use crate::wtcsv::core::wtcsv::WTCSV;

pub struct Diff {
	pub id: String,
	pub old: String,
	pub new: String,
}

impl Diff {
	pub fn diff_from_ref(left: &WTCSV, right: &WTCSV) -> Vec<Self> {
		let diffs: Vec<Self> =  Vec::new();

		vec![]
	}
}