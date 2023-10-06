use rust_strings::FileConfig;
use std::collections::HashSet;
use std::env;
use std::ffi::CString;
use std::path::PathBuf;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[link(name = "decompress")]
extern "C" {
    fn decompress_replay_file(input_path: *const i8, output_path: *const i8) -> u16;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Player {
    #[serde(rename = "v")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Team {
    #[serde(rename = "t")]
    name: Vec<String>,
}

#[derive(Debug, Clone)]
struct Replay {
    path: PathBuf,
}

impl Replay {
    fn new(path: PathBuf) -> Self {
        Self {
            path: path,
        }
    }

    unsafe fn decompress(&self, output: PathBuf) -> Result<DecompressedReplay, Box<dyn Error>> {
        let input_file = CString::new(
            self.path.to_str().ok_or(
                format!("CString fail: {:?}", self.path)
            )?
        )?;
        let output_file = CString::new(
            output.to_str().ok_or(
                format!("CString fail: {:?}", output)
            )?
        )?;
        unsafe {
            decompress_replay_file(input_file.as_ptr(), output_file.as_ptr());
        }
        Ok(DecompressedReplay::new(output))
    }
}

#[derive(Debug, Clone)]
struct DecompressedReplay {
    path: PathBuf,
    pub player_names: HashSet<String>,
    pub team_names: HashSet<String>,
}    

impl DecompressedReplay {
    fn new(path: PathBuf) -> Self {
        Self {
            path: path,
            player_names: HashSet::new(),
            team_names: HashSet::new(),
        }
    }

    fn extract_strings(&self) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
        let config = FileConfig::new(&self.path).with_min_length(5);
        rust_strings::strings(&config)
    }

    fn process_strings(&mut self) -> Result<(), Box<dyn Error>> {
        let extracted_strings = self.extract_strings()?;
        for (string, _) in extracted_strings {
            if string.starts_with("{\"v\":") {
                let player: Result<Player, _> = serde_json::from_str(&string);
                match player {
                    Ok(player) => {
                        if player.name.ends_with("...") {
                            continue;
                        }
                        self.player_names.insert(player.name);
                    },
                    Err(e) => {
                        eprintln!("Ignoring JSON deserialize error: {}", e);
                        continue;
                    }
                };
            }
            else if string.starts_with("{\"t\":[\"") {
                let team: Result<Team, _> = serde_json::from_str(&string);
                match team {
                    Ok(mut team) => {
                        if team.name[0] != "teamNames" {
                            continue;
                        }
                        let team_name = team.name.pop().unwrap();
                        self.team_names.insert(team_name);
                    },
                    Err(e) => {
                        eprintln!("Ignoring JSON deserialize error: {}", e);
                        continue;
                    }
                };
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: ./{} <input_replay_file> <output_replay_file>",
            args[0]
        );
        // FIXME: Can we return Err here?
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    let replay = Replay::new(input_path);

    let mut decompressed_replay = unsafe {
        replay.decompress(output_path)?
    };

    decompressed_replay.process_strings()?;

    for player_name in &decompressed_replay.player_names {
        println!("{}", player_name);
    }
    println!();
    for team_name in &decompressed_replay.team_names {
        println!("{}", team_name);
    }

    Ok(())
}
