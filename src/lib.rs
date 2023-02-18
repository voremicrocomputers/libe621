pub mod context;
pub mod defaults;
pub mod config;
pub mod mirrors;
pub mod database;
pub mod helpers;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::database::BulgeDB;
    use super::*;

    #[test]
    fn list_glib_deps() {
        let db = BulgeDB::from_file("/etc/bulge/databases/bulge.db").expect("Failed to open database");
        let deps = db.find_and_order_dependents_of_package("glib2").iter().map(|x| x.name.clone()).collect::<Vec<String>>();
        println!("{:#?}", deps);
    }
}
