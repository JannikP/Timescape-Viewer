/// Options for interpreting the content of .csv files.
///
/// This struct backs the [crate::views::csv_import_modal::csv_import_modal]
/// modal dialog widget.
///
/// Compare with [Pandas](https://pandas.pydata.org/docs/reference/api/pandas.read_csv.html)
/// and the csv crate's [ReaderBuilder](https://docs.rs/csv/1.3.1/csv/struct.ReaderBuilder.html).
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct CsvOptions {
    pub delimiter: char,
    pub quote_char: char,
    pub time_column: Option<usize>,
    pub header: usize,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            delimiter: ',',
            quote_char: '"',
            time_column: Some(0),
            header: 1,
        }
    }
}
