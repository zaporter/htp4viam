use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub type DeviceTypeMap = HashMap<String, DeviceType>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceType {
    pub architecture: String,
    pub os: String,
    #[serde(flatten)]
    pub classification: DeviceClassification,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "classification")]
#[serde(rename_all = "snake_case")]
pub enum DeviceClassification {
    Real,
    Docker(DockerSpec),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DockerSpec {
    pub image: String,
    pub htp_root: PathBuf,
}

pub fn parse(path: &PathBuf) -> Result<DeviceTypeMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let device_types: DeviceTypeMap = json5::from_str(&json5_str)?;
    Ok(device_types)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_device_types() {
        let path = PathBuf::from("../example_config/device_types.json5");
        let device_types_map = parse(&path).unwrap();
        assert_eq!(device_types_map.len(), 3);
        let docker = device_types_map.get("docker").unwrap();
        match &docker.classification {
            DeviceClassification::Docker(spec) => {
                assert_eq!(spec.image, "ubuntu")
            }
            _ => panic!("wrong classification"),
        }
    }
}
