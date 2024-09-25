use std::{env, fs, path::PathBuf};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::Deserialize;

pub mod search;

#[derive(Deserialize)]
pub struct Config {
    prefix: String,
    max_entries: usize,
    exact_match: bool,
    index_database_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/"));

        Self {
            prefix: ":nix".to_string(),
            max_entries: 3,
            exact_match: false,
            index_database_path: PathBuf::from(home_dir).join(".cache/nix-index/files"),
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

    match search::search(&input.to_string(), config) {
        Ok(matches) => matches,
        Err(error) => RVec::from_slice(&[Match {
            title: error.description().into(),
            use_pango: false,
            description: ROption::RSome(error.to_string().into()),
            icon: ROption::RNone,
            id: ROption::RNone,
        }]),
    }
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    // Handle the selected match and return how anyrun should proceed
    HandleResult::Close
}
