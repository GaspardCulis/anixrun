use std::process::Command;

use abi_stable::std_types::{ROption, RVec};
use anyrun_plugin::*;
use serde_json::{json, Map, Value};

use crate::{Config, SearchEngine};

impl SearchEngine {
    pub fn search(&self, query: &String, config: &Config) -> RVec<Match> {
        match self {
            SearchEngine::Online => todo!(),
            SearchEngine::Offline => {
                let output = Command::new("nix")
                    .arg("search")
                    .arg("--json")
                    .arg("nixpkgs")
                    .arg(query)
                    .output()
                    .expect("Failed to execute command");

                let json_output = String::from_utf8_lossy(&output.stdout);

                let parsed_json: Value = serde_json::from_str(&json_output).unwrap_or(json!("{}"));

                parsed_json
                    .as_object()
                    .unwrap_or(&Map::new())
                    .iter()
                    .take(config.max_entries)
                    .map(|(_, metadata)| {
                        let package_name =
                            metadata.get("pname").and_then(Value::as_str).unwrap_or("");

                        let description = metadata
                            .get("description")
                            .and_then(Value::as_str)
                            .unwrap_or("");

                        Match {
                            title: package_name.into(),
                            icon: ROption::RSome("".into()),
                            use_pango: false,
                            description: ROption::RSome(description.into()),
                            id: ROption::RNone, // The ID can be used for identifying the match later, is not required
                        }
                    })
                    .collect()
            }
        }
    }
}
