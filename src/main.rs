use rust_strings::FileConfig;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::CString;
use std::path::PathBuf;

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
        Self { path }
    }

    unsafe fn decompress(&self, output: PathBuf) -> Result<DecompressedReplay, Box<dyn Error>> {
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

#[derive(Debug, Clone)]
struct ReplayInfo {
    pub player_names: HashSet<String>,
    pub team_names: HashSet<String>,
}

impl ReplayInfo {
    fn new(strings: Vec<(String, u64)>) -> Self {
        let mut player_names = HashSet::new();
        let mut team_names = HashSet::new();

        for (string, _) in strings {
            if string.starts_with("{\"v\":") {
                Self::process_player_name(string)
                    .map(|player_name| player_names.insert(player_name));
            } else if string.starts_with("{\"t\":[\"") {
                Self::process_team_name(string).map(|team_name| team_names.insert(team_name));
            }
        }

        Self {
            player_names,
            team_names,
        }
    }

    fn process_player_name(string: String) -> Option<String> {
        let player: Result<Player, _> = serde_json::from_str(&string);
        player
            .ok()
            .and_then(|player| match player.name.ends_with("...") {
                true => None,
                false => Some(player.name),
            })
    }

    fn process_team_name(string: String) -> Option<String> {
        let team: Result<Team, _> = serde_json::from_str(&string);
        team.ok().and_then(|mut team| match team.name[0].as_str() {
            "teamNames" => {
                let team_name = team.name.pop().unwrap();
                Some(team_name)
            }
            _ => None,
        })
    }
}

#[derive(Debug, Clone)]
struct DecompressedReplay {
    path: PathBuf,
}

impl DecompressedReplay {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn extract_strings(&self) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
        let config = FileConfig::new(&self.path).with_min_length(5);
        rust_strings::strings(&config)
    }

    fn process_strings(&mut self) -> Result<ReplayInfo, Box<dyn Error>> {
        let extracted_strings = self.extract_strings()?;
        Ok(ReplayInfo::new(extracted_strings))
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

    let mut decompressed_replay = unsafe { replay.decompress(output_path)? };

    let processed_strings = decompressed_replay.process_strings()?;

    for player_name in &processed_strings.player_names {
        println!("{}", player_name);
    }
    println!();
    for team_name in &processed_strings.team_names {
        println!("{}", team_name);
    }

    Ok(())
}
