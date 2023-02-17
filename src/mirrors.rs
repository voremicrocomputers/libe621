use std::str::FromStr;
use crate::config::Config;
use crate::defaults::DEFAULT_MIRROR;

#[derive(Debug, Clone)]
pub struct MirrorList {
    /// Vec of URLs to mirrors.
    pub mirrors: Vec<String>,
}

#[derive(Debug)]
pub enum MirrorListError {
    /// The mirror list is empty.
    Empty,
}

impl FromStr for MirrorList {
    type Err = MirrorListError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors = s.lines().filter(|l| !l.is_empty() && !l.starts_with('#'))
            .filter(|l| l.starts_with("http://") || l.starts_with("https://"))
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
        if mirrors.is_empty() {
            Err(MirrorListError::Empty)
        } else {
            Ok(MirrorList {
                mirrors
            })
        }
    }
}

impl Default for MirrorList {
    fn default() -> Self {
        Self {
            mirrors: vec![
                "https://repo.yiffos.gay/core/x86_64/knot".to_string(),
                "https://repo.yiffos.gay/extra/x86_64/knot".to_string(),
                "https://repo.yiffos.gay/community/x86_64/knot".to_string(),
            ]
        }
    }
}

impl MirrorList {
    pub fn default_from_config(config: &Config) -> Self {
        let arch = config.architecture.as_str();
        let toolchain = config.toolchain.as_str();
        let repo_names = config.repos.iter().map(|r| r.name.as_str()).collect::<Vec<&str>>();
        let mirrors = repo_names.iter()
            .map(|repo_name|
                DEFAULT_MIRROR
                    .replace("$repo", repo_name)
                    .replace("$arch", arch)
                    .replace("$toolchain", toolchain))
            .collect::<Vec<String>>();
        Self {
            mirrors
        }
    }
}