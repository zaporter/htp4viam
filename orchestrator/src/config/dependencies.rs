use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub type DependencyMap = HashMap<String, DependencySpecification>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DependencySpecification {
    pub url: String,
    pub build_on: String,
    pub build_script: String,
    pub install_script: String,
}

pub fn parse(path: &PathBuf) -> Result<DependencyMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let dependencies: DependencyMap = json5::from_str(&json5_str)?;
    Ok(dependencies)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let path = PathBuf::from("../example_config/dependencies.json5");
        let dependencies_map = parse(&path).unwrap();
        assert_eq!(dependencies_map.len(), 1);
        let spec = dependencies_map.get("viam_server_appimage").unwrap();
        assert_eq!(spec.url, "https://github.com/viamrobotics/rdk");
    }
}
