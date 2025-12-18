use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // pub shared_directories: Vec<PathBuf>,
    // pub ignored_paths: Vec<PathBuf>,
    // pub local_directory: PathBuf,
}

pub fn config() {
    println!("{:?}", Config {});
}
