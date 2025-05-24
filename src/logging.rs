use flexi_logger::{FileSpec, Logger, WriteMode};

pub fn setup_logger() {
    Logger::try_with_env_or_str("info")
        .expect("Could not create logger.")
        // .log_to_file()
        //.format(flexi_logger::colored_detailed_format)
        .log_to_file(FileSpec::default())
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .expect("Could not start logger.");
}
