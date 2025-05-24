use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Origin {
    File(PathBuf),
}
