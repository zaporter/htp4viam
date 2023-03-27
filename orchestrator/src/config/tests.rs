use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type TestMap = HashMap<String, Vec<TestSpecification>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct TestSpecification {
    name: String,
    dependencies: HashMap<String, String>,
    excluded_device_types: Vec<String>,
    apparatus: String,
    robot_config: String,
    #[serde(default)]
    remote_test_script: Option<String>,
    #[serde(default)]
    on_device_test_script: Option<String>,
}

pub fn parse(path: &PathBuf) -> Result<TestMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let tests: TestMap = json5::from_str(&json5_str)?;
    Ok(tests)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_tests() {
        let path = PathBuf::from("../example_config/tests.json5");
        let test_map = parse(&path).unwrap();
        assert_eq!(test_map.len(), 1);
        assert_eq!(test_map.get("general").unwrap().len(), 2);
    }
}
