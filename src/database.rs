use std::collections::{HashMap, HashSet};
use std::path::{PathBuf};
use std::str::FromStr;
use rusqlite::{Connection, params};
use crate::defaults::bulge_package;
use crate::helpers::{string_to_vec, vec_to_string};

/// An in-memory representation of the database.
/// This should only be used if you need to access the entire database at once,
/// as it is not optimized for individual queries.
#[derive(Debug, Clone)]
pub struct BulgeDB {
    pub installed_packages: Vec<Package>,
    pub repos: Vec<Repo>,
    installed_packages_index: HashMap<String, usize>,
    repos_index: HashMap<String, usize>,
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
        let installed_packages = vec![bulge_package()];
        let mut installed_packages_index = HashMap::new();
        installed_packages_index.insert(installed_packages[0].name.clone(), 0);
        Self {
            installed_packages,
            repos: Vec::new(),
            installed_packages_index,
            repos_index: Default::default(),
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
        let mut stmt = conn.prepare("SELECT * FROM installed_packages")
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
        let mut installed_packages_index = HashMap::new();
        for (i, p) in installed_packages.iter().enumerate() {
            installed_packages_index.insert(p.name.clone(), i);
        }

        let mut stmt = conn.prepare("SELECT * FROM repos")
            .map_err(BulgeDBError::UnexpectedDBError)?;
        let repos = stmt.query_map([], |row| {
            Ok(Repo {
                name: row.get(0)?,
                repo_hash: row.get(1)?,
                last_updated: std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(row.get::<usize, String>(2)?.parse::<u64>().unwrap()),
            })
        }).map_err(BulgeDBError::UnexpectedDBError)?
            .collect::<Result<Vec<Repo>, rusqlite::Error>>()
            .map_err(BulgeDBError::UnexpectedDBError)?;
        let mut repos_index = HashMap::new();
        for (i, r) in repos.iter().enumerate() {
            repos_index.insert(r.name.clone(), i);
        }

        Ok(Self {
            installed_packages,
            repos,
            installed_packages_index,
            repos_index,
        })
    }

    pub fn to_file(self, bulge_db_path: impl Into<PathBuf>) -> Result<(), BulgeDBError> {
        let conn = Connection::open(bulge_db_path.into()).map_err(BulgeDBError::InvalidDBFile)?;
        conn.execute("CREATE TABLE IF NOT EXISTS installed_packages (
            name TEXT PRIMARY KEY,
            groups TEXT,
            source TEXT,
            version TEXT,
            epoch INTEGER,
            installed_files TEXT,
            provides TEXT,
            conflicts TEXT,
            dependencies TEXT
        )", []).map_err(BulgeDBError::UnexpectedDBError)?;
        conn.execute("CREATE TABLE IF NOT EXISTS repos (
            name TEXT PRIMARY KEY,
            repo_hash TEXT,
            last_updated INTEGER
        )", []).map_err(BulgeDBError::UnexpectedDBError)?;
        let mut stmt = conn.prepare("INSERT OR REPLACE INTO installed_packages VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .map_err(BulgeDBError::UnexpectedDBError)?;
        for p in self.installed_packages {
            stmt.execute(params![
                p.name,
                vec_to_string(p.groups),
                format!("{},{}", p.source.name, p.source.url.as_ref().unwrap_or(&"".to_string())),
                p.version,
                p.epoch,
                vec_to_string(p.installed_files),
                vec_to_string(p.provides),
                vec_to_string(p.conflicts),
                vec_to_string(p.dependencies),
            ]).map_err(BulgeDBError::UnexpectedDBError)?;
        }
        let mut stmt = conn.prepare("INSERT OR REPLACE INTO repos VALUES (?, ?, ?)")
            .map_err(BulgeDBError::UnexpectedDBError)?;
        for r in &self.repos {
            stmt.execute(params![
                r.name,
                r.repo_hash,
                r.last_updated.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            ]).map_err(BulgeDBError::UnexpectedDBError)?;
        }
        Ok(())
    }

    pub fn update_cached_repo(&mut self, name: &String, new_hash: &str) -> Result<(), BulgeDBError> {
        let index = self.repos_index.get(name).ok_or(BulgeDBError::InvalidSourceEntry(name.clone()))?;
        self.repos[*index].repo_hash = new_hash.to_string();
        self.repos[*index].last_updated = std::time::SystemTime::now();
        Ok(())
    }

    pub fn get_package(&self, name: &String) -> Option<&Package> {
        self.installed_packages_index.get(name).map(|i| &self.installed_packages[*i])
    }

    /// Returns a list of packages that depend on the given package,
    /// as well as the packages that depend on those packages, and so on.
    /// The list is sorted as a flattened tree, with the first element being the
    /// package that all others depend on, and the last element being a package
    /// that no other package depends on.
    /// If circular dependencies are found, they will be listed twice or more in the order required to resolve them.
    /// Note that this operation may take some time as it needs to query the database to find all dependents.
    /// Example:
    /// ```rust
    /// use libe621::database::BulgeDB;
    /// let db = BulgeDB::from_file("/etc/bulge/databases/bulge.db").expect("Failed to open database");
    /// let deps = db.find_and_order_dependents_of_package("glib2").iter().map(|x| x.name.clone()).collect::<Vec<String>>();
    /// println!("{:#?}", deps);
    /// ```
    pub fn find_and_order_dependents_of_package(&self, name: impl Into<String> + Clone) -> Vec<&Package> {
        let mut dependents = Vec::new();
        let mut stack = vec![name.clone().into()];
        let mut last_package = stack.last().unwrap().clone(); // used so that we don't have tons of the same package in a row
        let mut forbidden = HashSet::new(); // used to prevent circular dependencies from causing an infinite loop
        while let Some(name) = stack.pop() {
            forbidden.insert(name.clone());
            for p in &self.installed_packages {
                if p.dependencies.iter().any(|x| x == &name) && last_package != p.name && !forbidden.contains(&p.name) {
                    dependents.push(p);
                    stack.push(p.name.clone());
                    last_package = p.name.clone();
                }
            }
        }
        dependents
    }
}