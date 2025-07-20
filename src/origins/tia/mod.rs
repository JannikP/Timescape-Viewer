mod general;
mod samples;
mod signals;

use anyhow::{Context, Result};
use binrw::{BinRead, io::BufReader};
use std::io::{Read, Seek};
use zip::ZipArchive;

use general::General;
use samples::Samples;
use signals::Signals;

pub fn read_tia_trace<R>(reader: R) -> Result<()>
where
    R: Read + Seek,
{
    // .ttrecx are actually just ZIP archives.
    let mut archive =
        ZipArchive::new(reader).context("Failed to open TIA trace as ZIP archive.")?;

    // Read the general information
    let general_file = archive
        .by_name("General")
        .context("Failed to find 'General' entry in the .ttrecx file.")?;
    let general_reader = BufReader::new(general_file);
    let general: General = serde_xml_rs::from_reader(general_reader)
        .context("Failed to parse 'General' section of a .ttrecx file as XML.")?;

    // Read the signals information
    let signals_file = archive
        .by_name("Signals")
        .context("Failed to find 'Signals' entry in the  .ttrecx file.")?;
    let signals_reader = BufReader::new(signals_file);
    let signals: Signals = serde_xml_rs::from_reader(signals_reader)
        .context("Failed to parse 'Signals' section of a '.ttrecx' file as XML.")?;

    // Read the samples information
    let samples_file = archive
        .by_name_seek("Samples")
        .context("Failed to find 'Samples' entry in the '.ttrecx' file.")?;
    let mut samples_reader = BufReader::new(samples_file);
    let samples = Samples::read_le(&mut samples_reader)
        .context("Failed to parse 'Samples' section of a '.ttrecx' file.")?;

    // TODO: Use `general`, `signals` and `samples` to populate Timescape-Viewer's `Run` data structure.

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::read_tia_trace;
    use std::{fs::File, path::PathBuf};

    #[test]
    fn read_example_tia_trace() {
        let file: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "assets",
            "examples",
            "tia",
            "example_tia_trace.ttrecx",
        ]
        .iter()
        .collect();
        // As proposed here: https://stackoverflow.com/a/61107861
        if file.exists() {
            // Arrange
            let f = File::open(file).expect("Failed to open 'example_tia_trace.ttrecx'");
            // Act
            let result = read_tia_trace(f);
            // Assert
            assert!(result.is_ok());
        }
    }
}
