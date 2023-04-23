use std::{marker::PhantomData, time::SystemTime};

use anyhow::anyhow;

use crate::{
    config::{
        tests::{TestGroup, TestSpecification, TestSpecificationID},
        Config,
    },
    rcfolder::RcFolder,
    resource_ledger::ResourceLedger,
    resources::ResourceCollection,
};

#[derive(Debug)]
pub struct TestPriority(usize, &'static str);

pub const PRIORITY_ADMIN: TestPriority = TestPriority(0, "Admin (Manual)");
// When a human wants to run a single test on a single device
pub const PRIORITY_MANUAL_ONESHOT: TestPriority = TestPriority(1, "Highest (Manual)");
// When a human wants to run a test on many devices
pub const PRIORITY_MANUAL: TestPriority = TestPriority(2, "High (Manual)");
// If this ever gets used for CI
pub const PRIORITY_CI: TestPriority = TestPriority(3, "Medium (CI)");
// Constant automatic background checks when dependencies update
pub const PRIORITY_CANARY: TestPriority = TestPriority(4, "Low (Canary)");

pub type TestID = usize;

#[derive(Debug)]
pub struct TestMetadata {
    pub config_folder: RcFolder,
    pub config: Config,
    pub test_spec_id: TestSpecificationID,
    pub priority: TestPriority,
}

// Using some fancy Rust generics,
// we only expose certain methods on tests
// based on their current test-state.
//
// This is allows runtime state-transition logic
// to be checked by the compiler.
pub struct Queued;
pub struct Running;
pub struct Terminated;

#[derive(Debug)]
pub struct HtpTest<Stage = Queued> {
    pub meta: TestMetadata,
    pub id: TestID,
    pub queue_start_time: SystemTime,
    pub execution_start_time: Option<SystemTime>,
    pub execution_end_time: Option<SystemTime>,

    pub acquired_device_ids: Vec<String>,
    pub acquired_apparatus_ids: Vec<String>,

    pub was_terminated_before_finished: Option<bool>,

    pub stage: std::marker::PhantomData<Stage>,
}

impl<Stage> HtpTest<Stage> {
    pub fn get_test_group(&self) -> Option<&TestGroup> {
        self.meta.config.tests.get(&self.meta.test_spec_id.0)
    }
    pub fn get_test_spec(&self) -> Option<&TestSpecification> {
        let test_group = self.get_test_group();
        let Some(test_group) = test_group else {
            return None
        };
        test_group.get_test(&self.meta.test_spec_id.1)
    }
    pub fn new(meta: TestMetadata) -> anyhow::Result<HtpTest<Queued>> {
        let test = HtpTest {
            meta,
            id: 0, // TODO
            queue_start_time: SystemTime::now(),
            execution_start_time: None,
            execution_end_time: None,
            was_terminated_before_finished: None,
            acquired_device_ids: vec![],
            acquired_apparatus_ids: vec![],
            stage: PhantomData::default(),
        };
        // Ensure we have a valid TestSpecification to work with
        if test.get_test_spec().is_none() {
            return Err(anyhow!(
                "Unable to create HtpTest for {:?} because there is no TestSpecification",
                test.meta.test_spec_id
            ));
        }
        Ok(test)
    }
}
