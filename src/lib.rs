extern crate core;

#[allow(clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::module_name_repetitions)]

pub mod wtcsv;
pub mod diff;

const DELIMITER: char = ';';
const RECORD_SEP: char  = '\n';

const CRLF: &str = "\r\n";