use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type DevicesMap = HashMap<String, Device>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "type")]
    device_type: String,
    connected_apparatuses: Vec<String>,
}

pub fn parse(path: PathBuf) -> Result<DevicesMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let devices: DevicesMap = json5::from_str(&json5_str)?;
    Ok(devices)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let path = PathBuf::from("../example_config/devices.json5");
        let devices_map = parse(path).unwrap();
        assert_eq!(devices_map.len(), 2);
    }
}

