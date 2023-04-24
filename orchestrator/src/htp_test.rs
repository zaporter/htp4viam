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
    statistics::StatsSink,
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

// Using some fancy Rust generics & type system magic,
// we only expose certain methods on tests
// based on their current test-state.
//
// This is allows runtime state-transition logic
// to be checked by the compiler.
#[derive(Debug)]
pub struct Queued;
#[derive(Debug)]
pub struct Validated;
#[derive(Debug)]
pub struct Prepared;
#[derive(Debug)]
pub struct Aquiring;
#[derive(Debug)]
pub struct Runnable;
#[derive(Debug)]
pub struct Terminated;
trait TestStage {}
impl TestStage for Queued {}
impl TestStage for Validated {}
impl TestStage for Prepared {}
impl TestStage for Aquiring {}
impl TestStage for Runnable {}
impl TestStage for Terminated {}

#[derive(Debug)]
pub struct HtpTest<Stage = Queued> {
    pub id: TestID,

    pub config_folder: RcFolder,
    pub config: Option<Config>,
    pub test_spec_id: TestSpecificationID,
    pub priority: TestPriority,

    pub stats_sink: StatsSink,

    pub error: Option<anyhow::Error>,

    pub stage: std::marker::PhantomData<Stage>,
}

impl<Stage> HtpTest<Stage> {
    // TODO : Compile time checks for this.
    pub fn get_test_group(&self) -> Option<&TestGroup> {
        self.config
            .as_ref()
            .unwrap_or_else(|| panic!("Config cannot be accessed before validation"))
            .tests
            .get(&self.test_spec_id.0)
    }
    pub fn get_test_spec(&self) -> Option<&TestSpecification> {
        let test_group = self.get_test_group();
        let Some(test_group) = test_group else {
            return None
        };
        test_group.get_test(&self.test_spec_id.1)
    }
    pub fn clone_into<T>(self) -> HtpTest<T> {
        HtpTest {
            id: self.id,
            config_folder: self.config_folder,
            config: self.config,
            test_spec_id: self.test_spec_id,
            priority: self.priority,
            stats_sink: self.stats_sink,
            error: self.error,
            stage: PhantomData::default(),
        }
    }
    pub fn new(
        config_folder: RcFolder,
        test_spec_id: TestSpecificationID,
        priority: TestPriority,
    ) -> anyhow::Result<HtpTest<Queued>> {
        let test_id = 0; // TODO
        let test = HtpTest {
            id: test_id,
            config_folder,
            config: None,
            test_spec_id,
            priority,
            stats_sink: StatsSink::new(test_id),
            error: None,
            stage: PhantomData::default(),
        };
        Ok(test)
    }
}
