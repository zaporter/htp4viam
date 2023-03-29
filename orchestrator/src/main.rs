use std::path::PathBuf;

use crate::config::Config;

mod keygen;
mod ssh;
//mod git;
mod config;
mod rcfolder;
mod resources;
mod test_queue;
mod resource_ledger;
pub fn main() {
    println!("Started");
    let config_path = PathBuf::from("../config");
    let config = Config::new(&config_path).unwrap();

    println!("{:?}", config);
    println!("Finished");
}
