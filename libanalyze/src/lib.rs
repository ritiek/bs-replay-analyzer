mod replay;

pub use replay::{DecompressedReplay, Header, Replay, ReplayHeader, ReplayInfo};

pub const FILE_ID: u32 = 83749;
pub const PROTOCOL_VERSION: u16 = 33;
