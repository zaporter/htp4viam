use std::time::SystemTime;

use crate::{config::tests::TestSpecificationID, htp_test::TestID};

// This system needs
// a new name and a revamp
//
#[derive(Debug)]
pub struct RunningTestMapEntry {
    pub id: TestSpecificationID,
    pub ver: String,
    pub stage: String,
    pub entry_time: SystemTime,
}
#[derive(Default, Debug)]
pub struct RunningTestMap {
    pub map: Vec<RunningTestMapEntry>,
}
