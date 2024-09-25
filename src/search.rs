use abi_stable::std_types::{ROption, RVec};
use anyrun_plugin::*;
use nix_index::database;
use nix_index::files::FileTreeEntry;
use regex::bytes::Regex;

use crate::{Config, SearchEngine};

impl SearchEngine {
    pub fn search(&self, query: &String, config: &Config) -> RVec<Match> {
        match self {
            SearchEngine::Online => todo!(),
            SearchEngine::Offline => {
                let query_regex = if config.exact_match {
                    format!("^/bin/{}$", regex::escape(query))
                } else {
                    format!("/bin/{}", regex::escape(query))
                };

                let pattern = Regex::new(&query_regex).expect("Failed to build regex");

                let db = database::Reader::open(&config.index_database_path)
                    .expect("Failed to open database");

                db.query(&pattern)
                    .run()
                    .expect("Failed to query db")
                    .take(config.max_entries)
                    .filter_map(|result| match result {
                        Ok((store_path, FileTreeEntry { path, node })) => {
                            let run_match = Match {
                                title: store_path.name().into(),
                                description: ROption::RNone,
                                use_pango: false,
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
                    .collect()
            }
        }
    }
}
