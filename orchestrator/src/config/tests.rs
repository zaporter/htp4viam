use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

// Test type, test name
pub type TestSpecificationID = (String, String);

pub type TestMap = HashMap<String, TestGroup>;

#[derive(Debug, Deserialize, Serialize)]
pub struct TestGroup(Vec<TestSpecification>);

impl TestGroup {
    pub fn validate(&self) -> anyhow::Result<()> {
        // Ensure no duplicate test names
        let mut test_names = vec![];
        for test in &self.0 {
            if test_names.contains(&test.name) {
                return Err(anyhow!("Duplicate test name {}", &test.name));
            }
            test_names.push(test.name.clone())
        }

        Ok(())
    }
    pub fn get_test(&self, test_name: &str) -> Option<&TestSpecification> {
        for test in &self.0 {
            if test.name == test_name {
                return Some(&test);
            }
        }
        return None;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestSpecification {
    pub name: String,
    pub dependencies: HashMap<String, String>,
    pub excluded_device_types: Vec<String>,
    pub apparatus: String,
    pub robot_config: String,
    #[serde(default)]
    pub remote_test_script: Option<String>,
    #[serde(default)]
    pub on_device_test_script: Option<String>,
}

pub fn parse(path: &PathBuf) -> Result<TestMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let tests: TestMap = json5::from_str(&json5_str)?;
    for test_group in tests.values() {
        test_group.validate()?;
    }
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
        assert_eq!(test_map.get("general").unwrap().0.len(), 2);
    }
}
