use std::path::PathBuf;

use crate::config::Config;

mod keygen;
mod ssh;
//mod git;
mod config;
mod environment;
mod rcfolder;
mod resource_ledger;
mod resources;
mod statistics;
mod test_queue;
pub fn main() {
    println!("Started");
    let config_path = PathBuf::from("../config");
    let config = Config::new(&config_path).unwrap();

    println!("{:?}", config);
    println!("Finished");
}
