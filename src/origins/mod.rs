pub mod csv;
pub mod tia;

use std::path::PathBuf;

use csv::CsvOptions;

#[derive(Debug, Clone)]
pub enum Origin {
    CsvFile(PathBuf, Box<CsvOptions>),
    TiaTraceFile(PathBuf),
}
