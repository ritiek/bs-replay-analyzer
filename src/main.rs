use rust_strings::FileConfig;
use std::collections::HashSet;
use std::env;
use std::ffi::CString;
use std::path::Path;

#[link(name = "decompress")]
extern "C" {
    fn decompress_replay_file(input_path: *const i8, output_path: *const i8) -> u16;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: ./{} <input_replay_file> <output_replay_file>",
            args[0]
        );
        std::process::exit(1);
    }
    let input_file = CString::new(args[1].as_str()).unwrap_or_else(|e| {
        eprintln!("CString Error: {}", e);
        std::process::exit(1);
    });
    let output_file = CString::new(args[2].as_str()).unwrap_or_else(|e| {
        eprintln!("CString Error: {}", e);
        std::process::exit(1);
    });

    unsafe {
        decompress_replay_file(input_file.as_ptr(), output_file.as_ptr());
    }

    let config = FileConfig::new(Path::new(&args[2])).with_min_length(5);
    let extracted_strings = rust_strings::strings(&config).unwrap_or_else(|e| {
        eprintln!("Strings Error: {}", e);
        std::process::exit(1);
    });

    // TODO: Refactor this crazy.
    let mut player_ids = HashSet::new();
    for (string, _) in extracted_strings {
        // if string.starts_with("{\"v\":") || string.starts_with("{\"t\":[\"") {
        if string.starts_with("{\"v\":") {
            let possible_json: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&string);
            let player_id: String = match &possible_json {
                Ok(json) => {
                    // println!("{}", &json);
                    let player_id: String = json["v"].as_str().unwrap().to_string();
                    // TODO: Also parse `{"t":["teamNames","Good Guys"]}`.
                    if player_id.ends_with("...") {
                        continue;
                    }
                    player_id
                },
                Err(e) => {
                    eprintln!("Ignoring JSON deserialize error: {}", e);
                    continue;
                }
            };
            player_ids.insert(player_id);
        }
    }
    for player_id in &player_ids {
        println!("{}", player_id);
    }
}
