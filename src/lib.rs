use std::fs;
use std::process::Command;

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::Deserialize;
use serde_json::{json, Map, Value};

#[derive(Deserialize)]
pub struct Config {
    prefix: String,
    max_entries: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: ":nix".to_string(),
            max_entries: 3,
        }
    }
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/nix.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Anixrun".into(),
        icon: "flake".into(), // Icon from the icon theme
    }
}

#[get_matches]
fn get_matches(input: RString, config: &Config) -> RVec<Match> {
    let input = if let Some(input) = input.strip_prefix(&config.prefix) {
        input.trim()
    } else {
        return RVec::new();
    };

    let output = Command::new("nix")
        .arg("search")
        .arg("--json")
        .arg("nixpkgs")
        .arg(&input.to_string())
        .output()
        .expect("Failed to execute command");

    let json_output = String::from_utf8_lossy(&output.stdout);

    println!("{}", json_output);

    let parsed_json: Value = serde_json::from_str(&json_output).unwrap_or(json!("{}"));

    parsed_json
        .as_object()
        .unwrap_or(&Map::new())
        .iter()
        .take(config.max_entries)
        .map(|(package_name, metadata)| {
            let description = metadata
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or("");

            Match {
                title: package_name.clone().into(),
                icon: ROption::RSome("".into()),
                use_pango: false,
                description: ROption::RSome(description.into()),
                id: ROption::RNone, // The ID can be used for identifying the match later, is not required
            }
        })
        .collect()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    // Handle the selected match and return how anyrun should proceed
    HandleResult::Close
}
