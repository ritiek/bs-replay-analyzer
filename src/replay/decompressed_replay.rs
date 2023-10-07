use rust_strings::FileConfig;
use std::error::Error;
use std::path::PathBuf;

use super::replay_header::Header;
use super::replay_info::ReplayInfo;

#[derive(Debug, Clone)]
pub struct DecompressedReplay {
    path: PathBuf,
}

impl DecompressedReplay {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn extract_strings(&self) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
        let config = FileConfig::new(&self.path).with_min_length(5);
        rust_strings::strings(&config)
    }

    pub fn process_strings(&mut self) -> Result<ReplayInfo, Box<dyn Error>> {
        let extracted_strings = self.extract_strings()?;
        Ok(ReplayInfo::new(extracted_strings))
    }
}

impl Header for DecompressedReplay {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
