use flexi_logger::{FileSpec, Logger, WriteMode};

pub fn setup_logger() {
    let mut logger = Logger::try_with_env_or_str("info").expect("Could not create logger.");
    if cfg!(debug_assertions) {
        logger = logger
            .log_to_stderr()
            .format(flexi_logger::colored_detailed_format);
    } else {
        logger = logger
            .log_to_file(FileSpec::default())
            .write_mode(WriteMode::BufferAndFlush);
    }
    logger.start().expect("Could not start logger.");
}
