use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type DependenciesMap = HashMap<String, DependencySpecification>;

#[derive(Debug, Deserialize, Serialize)]
pub struct DependencySpecification {
    url: String,
    run_on_device: bool,
    targets: Vec<Target>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    device_types: Vec<String>,
    build_on_device: bool,
    build_script: String,
    install_script: String,
}

pub fn parse(path: PathBuf) -> Result<DependenciesMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let dependencies: DependenciesMap = json5::from_str(&json5_str)?;
    Ok(dependencies)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let path = PathBuf::from("../example_config/dependencies.json5");
        let dependencies_map = parse(path).unwrap();
        assert_eq!(dependencies_map.len(), 1);
        let spec = dependencies_map.get("viam_server_appimage").unwrap();
        assert_eq!(spec.url, "https://github.com/viamrobotics/rdk");
        assert_eq!(spec.run_on_device, true);
        let target = &spec.targets[0];
        assert_eq!(target.device_types, vec!["*"]);
        assert_eq!(target.build_on_device, false);
    }
}
