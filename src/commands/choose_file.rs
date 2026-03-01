use native_dialog::DialogBuilder;

use crate::origins::{Origin, csv::CsvOptions};

pub async fn choose_file() -> Option<Origin> {
    DialogBuilder::file()
        .set_location("~/Desktop")
        .add_filter("CSV Tables", ["csv", "tsv", "psv"])
        .open_single_file()
        .spawn()
        .await
        .unwrap()
        .map(|path| Origin::CsvFile(path, Box::default()))
}
