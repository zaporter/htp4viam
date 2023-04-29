use std::{marker::PhantomData, path::PathBuf, time::SystemTime};

use anyhow::{anyhow, Context};

use crate::{
    config::{
        dependencies::DependencySpecification,
        orchestrator_config::OrchestratorConfig,
        tests::{TestGroup, TestSpecification, TestSpecificationID},
        Config,
    },
    folder::{DependencyFolderType, HtpFolder, TestFolderType},
    resource_ledger::ResourceLedger,
    resources::ResourceCollection,
    statistics::StatsSink,
};

#[derive(Debug)]
pub struct TestPriority(usize, &'static str);

pub const PRIORITY_ADMIN: TestPriority = TestPriority(0, "Admin (Manual)");
// When a human wants to run a single test on a single device
// pub const PRIORITY_MANUAL_ONESHOT: TestPriority = TestPriority(1, "Highest (Manual)");
// // When a human wants to run a test on many devices
// pub const PRIORITY_MANUAL: TestPriority = TestPriority(2, "High (Manual)");
// // If this ever gets used for CI
// pub const PRIORITY_CI: TestPriority = TestPriority(3, "Medium (CI)");
// // Constant automatic background checks when dependencies update
// pub const PRIORITY_CANARY: TestPriority = TestPriority(4, "Low (Canary)");

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
pub trait TestStage {}
impl TestStage for Queued {}
impl TestStage for Validated {}
impl TestStage for Prepared {}
impl TestStage for Aquiring {}
impl TestStage for Runnable {}
impl TestStage for Terminated {}

pub trait PostValidation {}
impl PostValidation for Validated {}
impl PostValidation for Prepared {}
impl PostValidation for Aquiring {}
impl PostValidation for Runnable {}

#[derive(Debug)]
pub struct HtpTest<Stage = Queued> {
    pub id: TestID,

    pub orchestrator_config: OrchestratorConfig,

    pub config_folder: HtpFolder,
    pub persist_folder: HtpFolder,

    pub config: Option<Config>,
    pub dependencies: Option<Vec<Dependency>>,
    pub test_spec_id: TestSpecificationID,
    pub priority: TestPriority,

    pub stats_sink: StatsSink,

    pub error: Option<anyhow::Error>,

    pub stage: std::marker::PhantomData<Stage>,
}

impl<Stage> HtpTest<Stage>
where
    Stage: TestStage,
{
    pub fn new(
        config_folder_path: &PathBuf,
        orchestrator_config: OrchestratorConfig,
        test_spec_id: TestSpecificationID,
        priority: TestPriority,
    ) -> anyhow::Result<HtpTest<Queued>> {
        let test_id = 0; // TODO
        let config_folder = HtpFolder::new_test(
            &orchestrator_config,
            TestFolderType::Config,
            &test_spec_id,
            "0",
        )?;
        config_folder.copy_from(config_folder_path)?;
        let persist_folder = HtpFolder::new_test(
            &orchestrator_config,
            TestFolderType::Persist,
            &test_spec_id,
            "0",
        )?;

        Ok(HtpTest {
            id: test_id,
            config_folder,
            persist_folder,
            orchestrator_config,
            config: None,
            dependencies: None,
            test_spec_id,
            priority,
            stats_sink: StatsSink::new(test_id),
            error: None,
            stage: PhantomData::default(),
        })
    }

    pub fn clone_into<T>(self) -> HtpTest<T> {
        HtpTest {
            id: self.id,
            config_folder: self.config_folder,
            persist_folder: self.persist_folder,
            orchestrator_config: self.orchestrator_config,
            config: self.config,
            dependencies: self.dependencies,
            test_spec_id: self.test_spec_id,
            priority: self.priority,
            stats_sink: self.stats_sink,
            error: self.error,
            stage: PhantomData::default(),
        }
    }
}

impl<Stage> HtpTest<Stage>
where
    Stage: PostValidation,
{
    pub fn config(&self) -> &Config {
        self.config
            .as_ref()
            .unwrap_or_else(|| panic!("Config was not present post validation"))
    }
    pub fn get_test_group(&self) -> &TestGroup {
        self.config()
            .tests
            .get(&self.test_spec_id.0)
            .as_ref()
            .unwrap_or_else(|| panic!("Config got past validation without test group"))
    }
    pub fn get_test_spec(&self) -> &TestSpecification {
        let test_group = self.get_test_group();
        test_group
            .get_test(&self.test_spec_id.1)
            .as_ref()
            .unwrap_or_else(|| panic!("Config got past validation without test specification"))
    }
    pub fn dependencies(&self) -> &Vec<Dependency> {
        self.dependencies
            .as_ref()
            .unwrap_or_else(|| panic!("Config got past validation without creating dependencies"))
    }
    pub fn dependencies_mut(&mut self) -> &mut Vec<Dependency> {
        self.dependencies
            .as_mut()
            .unwrap_or_else(|| panic!("Config got past validation without creating dependencies"))
    }
}

#[derive(Debug)]
pub struct Dependency {
    name: String,
    specification: DependencySpecification,
    build_input_folder: HtpFolder,
    build_output_folder: HtpFolder,
}
impl Dependency {
    pub fn new(
        orchestrator_config: &OrchestratorConfig,
        name: &str,
        specification: &DependencySpecification,
    ) -> anyhow::Result<Self> {
        let build_input_folder = HtpFolder::new_dependency(
            orchestrator_config,
            DependencyFolderType::BuildInput,
            name,
            "0",
        )
        .context("failed to create build input folder")?;
        let build_output_folder = HtpFolder::new_dependency(
            orchestrator_config,
            DependencyFolderType::BuildOutput,
            name,
            "0",
        )
        .context("failed to create build output folder")?;
        Ok(Self {
            name: name.into(),
            specification: specification.clone(),
            build_input_folder,
            build_output_folder,
        })
    }
}
