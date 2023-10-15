use std::error::Error;
use std::ffi::CString;
use std::path::PathBuf;

use super::decompressed_replay::DecompressedReplay;
use super::replay_header::Header;

#[link(name = "brp")]
extern "C" {
    fn decompress_replay_file(input_path: *const i8, output_path: *const i8) -> u16;
}

#[derive(Debug, Clone)]
pub struct Replay {
    path: PathBuf,
}

impl Replay {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// # Safety
    ///
    /// This function is unsafe because it calls a C function.
    pub unsafe fn decompress(&self, output: PathBuf) -> Result<DecompressedReplay, Box<dyn Error>> {
        let input_file = CString::new(
            self.path
                .to_str()
                .ok_or(format!("CString fail: {:?}", self.path))?,
        )?;
        let output_file = CString::new(
            output
                .to_str()
                .ok_or(format!("CString fail: {:?}", output))?,
        )?;
        unsafe {
            decompress_replay_file(input_file.as_ptr(), output_file.as_ptr());
        }
        Ok(DecompressedReplay::new(output))
    }
}

impl Header for Replay {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
