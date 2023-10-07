use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
pub struct ReplayInfo {
    pub player_names: HashSet<String>,
    pub team_names: HashSet<String>,
}

impl ReplayInfo {
    pub fn new(strings: Vec<(String, u64)>) -> Self {
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
