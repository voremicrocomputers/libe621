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

}
