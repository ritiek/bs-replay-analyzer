use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ReplayHeader {
    pub file_id: u32,
    pub protocol_version: u16,
}

pub trait Header {
    fn get_path(&self) -> PathBuf;

    fn get_header(&self) -> Result<ReplayHeader, Box<dyn Error>> {
        let mut replay = File::open(self.get_path())?;

        let mut file_id = [0; 4];
        replay.read_exact(&mut file_id)?;

        let mut protocol_version = [0; 2];
        replay.read_exact(&mut protocol_version)?;

        Ok(ReplayHeader {
            file_id: u32::from_le_bytes(file_id),
            protocol_version: u16::from_le_bytes(protocol_version),
        })
    }
}
