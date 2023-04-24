use std::{path::PathBuf, time::Duration};

use crate::{config::Config, orchestrator::Orchestrator};

mod htp_test;
mod keygen;
mod ssh;
mod stages;
//mod git;
mod config;
mod environment;
mod orchestrator;
mod rcfolder;
mod resource_ledger;
mod resources;
mod statistics;
mod test_queue;
pub fn main() {
    println!("Started");
    // let config_path = PathBuf::from("../config");
    // let config = Config::new(&config_path).unwrap();

    // println!("{:?}", config);
    let mut orchestrator = Orchestrator::new();
    orchestrator.start().unwrap();
    std::thread::sleep(Duration::from_millis(500));
    orchestrator.stop().unwrap();
    println!("Finished");
}
