use crate::database::{Package, Source};

pub const DEFAULT_CONFIG: &str = {
    r#"{
        "architecture": "x86_64",
        "toolchain": "knot",
        "colour": true,
        "progressbar": true,
        "repos": [
            {
                "name": "core",
                "active": true
            },
            {
                "name": "extra",
                "active": true
            },
            {
                "name": "community",
                "active": true
            },
            {
                "name": "external",
                "active": false,
                "url": "https://www.example.com"
            }
        ]
    }"#
};

pub const DEFAULT_MIRROR: &str = {
   "https://repo.yiffos.gay/$repo/$arch/$toolchain"
};

// TODO: find a way to automatically update this
pub const CURRENT_BULGE_VERSION: &str = {
    "0.2.0"
};

pub fn bulge_package() -> Package {
    Package {
        name: "bulge".to_string(),
        groups: vec!["core".to_string()],
        source: Source {
            name: "core".to_string(),
            url: None,
        },
        version: CURRENT_BULGE_VERSION.to_string(),
        epoch: 0,
        installed_files: vec![],
        provides: vec!["bulge".to_string()],
        conflicts: vec![],
        dependencies: vec!["curl".to_string(), "sqlite".to_string()],
    }
}