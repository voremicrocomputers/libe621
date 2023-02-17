use crate::config::Config;

#[derive(Debug, Clone)]
pub struct BulgeContext {
    /// The root directory of the system; usually `/`.
    root: String,
    config: Config,
}

impl Default for BulgeContext {
    fn default() -> Self {
        Self {
            root: String::from("/"),
            config: Config::default(),
        }
    }
}