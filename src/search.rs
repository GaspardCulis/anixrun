use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use abi_stable::std_types::{ROption, RVec};
use anyrun_plugin::*;
use error_chain::error_chain;
use nix_index::database;
use nix_index::files::FileTreeEntry;
use regex::bytes::Regex;

use crate::{MatchData, State};

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

pub fn search(query: &String, state: &mut State) -> Result<RVec<Match>> {
    let config = &state.config;

    let query_regex = if config.exact_match {
        format!("^/bin/{}$", regex::escape(query))
    } else {
        format!("/bin/{}", regex::escape(query))
    };

    let pattern = Regex::new(&query_regex).chain_err(|| ErrorKind::Grep(query_regex.clone()))?;

    let db = database::Reader::open(&config.index_database_path)
        .chain_err(|| ErrorKind::ReadDatabase(config.index_database_path.clone()))?;

    Ok(db
        .query(&pattern)
        .run()
        .chain_err(|| ErrorKind::Grep(query_regex.clone()))?
        .take(config.max_entries)
        .filter_map(|result| match result {
            Ok((store_path, FileTreeEntry { path, node: _ })) => {
                let id = {
                    let mut s = DefaultHasher::new();
                    store_path.hash().hash(&mut s);
                    s.finish()
                };

                let binary_path = String::from_utf8_lossy(&path).to_string();
                let data = MatchData {
                    package: store_path.name().to_string(),
                    package_noversion: store_path.origin().attr.clone(),
                    binary_name: binary_path
                        .split("/")
                        .last()
                        .expect("Should contain /bin")
                        .to_string(),
                    binary_path,
                };

                let run_match = Match {
                    title: data.package_noversion.clone().into(),
                    use_pango: true,
                    description: ROption::RSome(
                        format!(
                            "Run <big>{}</big> from <big>{}</big> package",
                            data.binary_path, data.package
                        )
                        .into(),
                    ),
                    icon: ROption::RNone,
                    id: ROption::RSome(id),
                };

                // Add match to state
                state.match_data.insert(id, data);

                Some(run_match)
            }
            Err(error) => {
                eprintln!("Encountered error while unwrapping result: {:?}", error);
                None
            }
        })
        .collect())
}
