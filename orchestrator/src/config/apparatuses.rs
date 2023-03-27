use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type ApparatusMap = HashMap<String, Apparatus>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Apparatus {
    #[serde(default = "default_exclusively_locked")]
    is_exclusively_locked: bool,
    peripherals: Vec<String>,
    #[serde(default)]
    wrapped_apparatuses: Vec<String>,
}

impl Default for Apparatus {
    fn default() -> Self {
        Apparatus {
            is_exclusively_locked: default_exclusively_locked(),
            peripherals: Vec::new(),
            wrapped_apparatuses: Vec::new(),
        }
    }
}

fn default_exclusively_locked() -> bool {
    true
}

pub fn parse(path: &PathBuf) -> Result<ApparatusMap, anyhow::Error> {
    let json5_str = std::fs::read_to_string(path)?;
    let apparatuses: ApparatusMap = json5::from_str(&json5_str)?;
    Ok(apparatuses)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let path = PathBuf::from("../example_config/apparatuses.json5");
        let apparatus_map = parse(&path).unwrap();
        assert_eq!(apparatus_map.len(), 4);
    }
}
