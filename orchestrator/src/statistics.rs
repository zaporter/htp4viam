use std::fmt::Debug;

use serde::Serialize;

use crate::htp_test::TestID;
#[derive(Debug, Clone)]
pub struct StatsSink {
    key: TestID,
}
impl StatsSink {
    pub fn new(key: TestID) -> Self {
        Self { key }
    }
    pub fn write<T>(&mut self, stage: &str, val: T)
    where
        T: Serialize + Debug + Clone,
    {
        println!("STATS: {}, {}, {:?}", self.key, stage, val)
    }
}
