use serde::Deserialize;
use crate::defaults::DEFAULT_CONFIG;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub architecture: String,
    pub toolchain: String,
    pub colour: bool,
    pub progressbar: bool,
    pub repos: Vec<RepoNode>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RepoNode {
    pub name: String,
    pub active: bool,
    pub url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_CONFIG).expect("Failed to deserialize default config")
    }
}