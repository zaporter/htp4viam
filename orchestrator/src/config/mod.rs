use std::path::PathBuf;

use anyhow::Context;

use self::{
    apparatuses::ApparatusMap, dependencies::DependencyMap, device_types::DeviceTypeMap,
    devices::DeviceMap, tests::TestMap,
};

pub mod apparatuses;
pub mod dependencies;
pub mod device_types;
pub mod devices;
pub mod tests;

#[derive(Debug, Default)]
pub struct Config {
    pub apparatuses: ApparatusMap,
    pub dependencies: DependencyMap,
    pub devices: DeviceMap,
    pub device_types: DeviceTypeMap,
    pub tests: TestMap,
}

impl Config {
    pub fn new(base_path: &PathBuf) -> anyhow::Result<Config> {
        let result = Config {
            apparatuses: apparatuses::parse(&base_path.join("apparatuses.json5"))
                .context("Failed to read apparatuses.json5")?,
            dependencies: dependencies::parse(&base_path.join("dependencies.json5"))
                .context("Failed to read dependencies.json5")?,
            devices: devices::parse(&base_path.join("devices.json5"))
                .context("Failed to read devices.json5")?,
            device_types: device_types::parse(&base_path.join("device_types.json5"))
                .context("Failed to read device_types.json5")?,
            tests: tests::parse(&base_path.join("tests.json5"))
                .context("Failed to read tests.json5")?,
        };
        Ok(result)
    }
}


#[cfg(test)]
mod test_mod {
    use super::*;
    #[test]
    fn test_parse() -> anyhow::Result<()>{
        let base_path = PathBuf::from("../example_config");
        let _ = Config::new(&base_path)?;
        Ok(())
    }

}

