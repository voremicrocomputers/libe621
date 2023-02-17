use std::path::{Path, PathBuf};
use std::str::FromStr;
use rusqlite::Connection;
use crate::config::Config;
use crate::defaults::bulge_package;
use crate::helpers::string_to_vec;

#[derive(Debug, Clone)]
pub struct BulgeDB {
    installed_packages: Vec<Package>,
    repos: Vec<Repo>,
}

#[derive(Debug)]
pub enum BulgeDBError {
    InvalidDBFile(rusqlite::Error),
    UnexpectedDBError(rusqlite::Error),
    InvalidSourceEntry(String),
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub groups: Vec<String>,
    pub source: Source,
    pub version: String,
    pub epoch: i32,
    pub installed_files: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub dependencies: Vec<String>
}

#[derive(Debug, Clone)]
pub struct PackageIntermediary {
    pub name: String,
    pub groups: String,
    pub source: String,
    pub version: String,
    pub epoch: i32,
    pub installed_files: String,
    pub provides: String,
    pub conflicts: String,
    pub dependencies: String
}

#[derive(Debug, Clone)]
pub struct Source {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Repo {
    pub name: String,
    pub repo_hash: String,
    pub last_updated: std::time::SystemTime,
}

impl Default for BulgeDB {
    fn default() -> Self {
        Self {
            installed_packages: vec![bulge_package()],
            repos: Vec::new(),
        }
    }
}

impl FromStr for Source {
    type Err = BulgeDBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, ',');
        let name = split.next().unwrap();
        let url = split.next();
        Ok(Source {
            name: name.to_string(),
            url: url.map(|s| s.to_string()),
        })
    }
}

impl BulgeDB {
    pub fn from_file(bulge_db_path: impl Into<PathBuf>) -> Result<Self, BulgeDBError> {
        let conn = Connection::open(bulge_db_path.into()).map_err(BulgeDBError::InvalidDBFile)?;
        let mut stmt = conn.prepare("SELECT * FROM installed_packages?")
            .map_err(BulgeDBError::UnexpectedDBError)?;
        let installed_packages = stmt.query_map([], |row| {
            Ok(PackageIntermediary {
                name: row.get(0)?,
                groups: row.get(1)?,
                source: row.get(2)?,
                version: row.get(3)?,
                epoch: row.get(4)?,
                installed_files: row.get(5)?,
                provides: row.get(6)?,
                conflicts: row.get(7)?,
                dependencies: row.get(8)?,
            })
        }).map_err(BulgeDBError::UnexpectedDBError)?
            .collect::<Result<Vec<PackageIntermediary>, rusqlite::Error>>()
            .map_err(BulgeDBError::UnexpectedDBError)?
            .into_iter()
            .map(|p| Ok(Package {
                name: p.name,
                groups: string_to_vec(p.groups),
                source: Source::from_str(&p.source)?,
                version: p.version,
                epoch: p.epoch,
                installed_files: string_to_vec(p.installed_files),
                provides: string_to_vec(p.provides),
                conflicts: string_to_vec(p.conflicts),
                dependencies: string_to_vec(p.dependencies),
            }))
            .collect::<Result<Vec<Package>, BulgeDBError>>()?;

        unimplemented!()
    }
}