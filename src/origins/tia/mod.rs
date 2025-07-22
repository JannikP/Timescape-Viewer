mod general;
mod samples;
mod signals;

use anyhow::{anyhow, Context, Result};
use binrw::{BinRead, io::BufReader};
use std::fs::File;
use std::io::{Read, Seek};
use std::path::PathBuf;
use zip::ZipArchive;

use crate::state::{Source, Timeline, Track, Run};
use crate::state::signal::Signal;

use general::General;
use samples::{SampleBlock, Samples};
use signals::Signals;

pub fn read_tia_trace_file(path: &PathBuf) -> Result<Source> {
    File::open(path)
        .context("Could not open TIA trace file")
        .and_then(|file| read_tia_trace(BufReader::new(file)))
}

pub fn read_tia_trace<R>(reader: R) -> Result<Source>
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

    let track = Track {
        signals: signals.signals.iter().map(|s| Signal::from(s).reference_counted()).collect(),
        time: Timeline::ExplicitTime {
            timestamps: samples.timestamps.timestamps.clone(),
        },
        values: signals
            .signals
            .iter()
            .map(|s| samples
                .find_by_signal_id(s.signal_id)
                .map(SampleBlock::just_values)
                .ok_or_else(|| anyhow!("Signal ID '{}' not found in 'Samples' section.", s.signal_id))
            )
            .collect::<Result<Vec<Vec<f64>>>>()
            .context("Failed to collect samples for all signals.")?,
    };
    let run = Run::with_single_track(track)
        .with_title(general.record_name);
    let source = Source::with_single_run(run);

    Ok(source)
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
