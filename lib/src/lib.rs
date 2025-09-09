pub mod models;
pub mod generator;
pub mod model_analyzer;
pub mod license_handler;
pub mod component_generator;

pub use models::*;
pub use generator::*;
pub use serde_json::Value;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
