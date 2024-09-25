use std::path::PathBuf;

use abi_stable::std_types::{ROption, RVec};
use anyrun_plugin::*;
use error_chain::error_chain;
use nix_index::database;
use nix_index::files::FileTreeEntry;
use regex::bytes::Regex;

use crate::{Config, SearchEngine};

error_chain! {
    errors {
        ReadDatabase(database: PathBuf) {
            description("Database read error")
            display("Reading from the database at '{}' failed.\n\
                     This may be caused by a corrupt or missing database, try (re)running `nix-index` to generate the database. \n\
                     If the error persists please file a bug report at https://github.com/nix-community/nix-index.", database.to_string_lossy())
        }
        Grep(pattern: String) {
            description("Regex builder error")
            display("Constructing the regular expression from the pattern '{}' failed.", pattern)
        }
    }
}

impl SearchEngine {
    pub fn search(&self, query: &String, config: &Config) -> Result<RVec<Match>> {
        match self {
            SearchEngine::Online => todo!(),
            SearchEngine::Offline => {
                let query_regex = if config.exact_match {
                    format!("^/bin/{}$", regex::escape(query))
                } else {
                    format!("/bin/{}", regex::escape(query))
                };

                let pattern =
                    Regex::new(&query_regex).chain_err(|| ErrorKind::Grep(query_regex.clone()))?;

                let db = database::Reader::open(&config.index_database_path)
                    .chain_err(|| ErrorKind::ReadDatabase(config.index_database_path.clone()))?;

                Ok(db
                    .query(&pattern)
                    .run()
                    .chain_err(|| ErrorKind::Grep(query_regex.clone()))?
                    .take(config.max_entries)
                    .filter_map(|result| match result {
                        Ok((store_path, FileTreeEntry { path, node: _ })) => {
                            let binary = String::from_utf8_lossy(&path);
                            let run_match = Match {
                                title: binary.split("/").last().unwrap_or("Error").into(),
                                use_pango: true,
                                description: ROption::RSome(
                                    format!(
                                        "Run <big>{}</big> from <big>{}</big> package",
                                        binary,
                                        store_path.name()
                                    )
                                    .into(),
                                ),
                                icon: ROption::RNone,
                                id: ROption::RNone,
                            };
                            Some(run_match)
                        }
                        Err(error) => {
                            eprintln!("Encountered error while unwrapping result: {:?}", error);
                            None
                        }
                    })
                    .collect())
            }
        }
    }
}
