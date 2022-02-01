extern crate core;

#[allow(clippy::missing_errors_doc)]

pub mod wtcsv;
pub mod diff;

const DELIMITER: char = ';';
const RECORD_SEP: char  = '\n';

const CRLF: &str = "\r\n";