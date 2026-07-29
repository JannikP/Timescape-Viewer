use std::path::PathBuf;

use native_dialog::DialogBuilder;

use crate::origins::{Origin, csv::CsvOptions};

pub async fn choose_file() -> Option<Origin> {
    DialogBuilder::file()
        .set_location("~/Desktop")
        .add_filter("CSV Tables", ["csv", "tsv", "psv"])
        .add_filter("TIA Traces", ["ttrecx"])
        .open_single_file()
        .spawn()
        .await
        .unwrap()
        .map(identify_file)
        .flatten()
}

fn identify_file(path: PathBuf) -> Option<Origin> {
    let extension = path
        .extension()
        .map(|os_str| os_str.to_string_lossy())
        .unwrap_or_default();

    if extension == "ttrecx" {
        Some(Origin::TiaTraceFile(path))
    } else if ["csv", "tsv", "psv"].contains(&extension.as_ref()) {
        Some(Origin::CsvFile(path, Box::new(CsvOptions::default())))
    } else {
        None
    }

}
