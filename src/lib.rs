use std::fs;

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::Deserialize;

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
        name: "Demo".into(),
        icon: "help-about".into(), // Icon from the icon theme
    }
}

#[get_matches]
fn get_matches(input: RString) -> RVec<Match> {
    // The logic to get matches from the input text in the `input` argument.
    // The `data` is a mutable reference to the shared data type later specified.
    vec![Match {
        title: "Test match".into(),
        icon: ROption::RSome("help-about".into()),
        use_pango: false,
        description: ROption::RSome("Test match for the plugin API demo".into()),
        id: ROption::RNone, // The ID can be used for identifying the match later, is not required
    }]
    .into()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    // Handle the selected match and return how anyrun should proceed
    HandleResult::Close
}
