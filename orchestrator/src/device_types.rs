use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type DeviceTypesMap = HashMap<String, DeviceType>;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceType {
    architecture: String,
    os: String,
    os_version: String,
}

pub fn parse_device_types(path: PathBuf) -> Result<DeviceTypesMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let device_types: DeviceTypesMap = json5::from_str(&json5_str)?;
    Ok(device_types)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_device_types() {
        let path = PathBuf::from("../example_config/device-types.json5");
        let device_types_map = parse_device_types(path).unwrap();
        assert_eq!(device_types_map.len(), 2);
    }
}
