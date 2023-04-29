use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub type DeviceMap = HashMap<String, Device>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "type")]
    pub device_type: String,
    pub login_username: String,
    pub connected_apparatuses: Vec<String>,
}

pub fn parse(path: &PathBuf) -> Result<DeviceMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let devices: DeviceMap = json5::from_str(&json5_str)?;
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let path = PathBuf::from("../example_config/devices.json5");
        let devices_map = parse(&path).unwrap();
        assert_eq!(devices_map.len(), 2);
    }
}
