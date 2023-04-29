use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrchestratorConfig {
    pub htp_folder_root: PathBuf,
    pub persist_test_runs: bool,
    pub host_addr: String,
    pub loki_addr: String,
}

pub fn parse(path: &PathBuf) -> anyhow::Result<OrchestratorConfig> {
    let json5_str = std::fs::read_to_string(path)?;
    let dependencies: OrchestratorConfig = json5::from_str(&json5_str)?;
    Ok(dependencies)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let path = PathBuf::from("../example_config/orchestrator.json5");
        let orchestrator = parse(&path).unwrap();
    }
}
