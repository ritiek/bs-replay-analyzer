use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use libanalyze::{Header, Replay};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: ./{} <input_replay_file> <output_replay_file>",
            args[0]
        );
        return Ok(ExitCode::FAILURE);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    let replay = Replay::new(input_path);
    let header = replay.get_header()?;
    println!("File ID: {}", header.file_id);
    println!("Protocol Version: {}", header.protocol_version);
    println!();

    let mut decompressed_replay = unsafe { replay.decompress(output_path)? };
    let processed_strings = decompressed_replay.process_strings()?;
    for player_name in &processed_strings.player_names {
        println!("{}", player_name);
    }
    println!();
    for team_name in &processed_strings.team_names {
        println!("{}", team_name);
    }

    Ok(ExitCode::SUCCESS)
}
