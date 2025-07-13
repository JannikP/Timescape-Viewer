pub mod csv;

use std::path::PathBuf;

use csv::CsvOptions;

#[derive(Debug, Clone)]
pub enum Origin {
    CsvFile(PathBuf, Box<CsvOptions>),
}
