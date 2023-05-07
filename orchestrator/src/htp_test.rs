use std::{
    any,
    marker::PhantomData,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use anyhow::{anyhow, Context};

use crate::{
    config::{
        dependencies::DependencySpecification,
        device_types::{DeviceClassification, DeviceType, DockerSpec},
        orchestrator_config::OrchestratorConfig,
        tests::{TestGroup, TestMap, TestSpecification, TestSpecificationID},
        Config,
    },
    environment::docker_env::DockerEnvironment,
    folder::{DependencyFolderType, HtpFolder, TestFolderType},
    resource_ledger::ResourceLedger,
    resources::ResourceCollection,
    running_test_map::{RunningTestMap, RunningTestMapEntry},
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
pub trait TestStage {
    // This could be done with core_intrinsics unsafe {type_name::<Self>}
    // but that is an unstable feature.
    fn name() -> String;
}
impl TestStage for Queued {
    fn name() -> String {
        "Queued".into()
    }
}
impl TestStage for Validated {
    fn name() -> String {
        "Validated".into()
    }
}
impl TestStage for Prepared {
    fn name() -> String {
        "Prepared".into()
    }
}
impl TestStage for Aquiring {
    fn name() -> String {
        "Aquiring".into()
    }
}
impl TestStage for Runnable {
    fn name() -> String {
        "Runnable".into()
    }
}
impl TestStage for Terminated {
    fn name() -> String {
        "Terminated".into()
    }
}

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
    pub test_map: Arc<Mutex<RunningTestMap>>,

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
        test_map: Arc<Mutex<RunningTestMap>>,
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
        {
            let mut map = test_map.lock().unwrap();
            map.map.push(RunningTestMapEntry {
                id: test_spec_id.clone(),
                ver: "0".into(),
                stage: Queued::name(),
                entry_time: SystemTime::now(),
            });
        }
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
            test_map,
            stage: PhantomData::default(),
        })
    }

    pub fn clone_into<T: TestStage>(self) -> HtpTest<T> {
        {
            let mut map = self.test_map.lock().unwrap();
            let mut map_entry: &mut RunningTestMapEntry = map
                .map
                .iter_mut()
                .find(|p| p.id == self.test_spec_id)
                .unwrap();
            map_entry.stage = T::name();
            map_entry.entry_time = SystemTime::now();
        }
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
            test_map: self.test_map,
            stage: PhantomData::default(),
        }
    }
}

impl<Stage> HtpTest<Stage>
where
    Stage: TestStage + PostValidation,
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
    pub fn test_mount_map(&self, inner_htp_root: &PathBuf) -> EnvironmentMountMap {
        let mut map = EnvironmentMountMap::new();
        map.0.push(MountMapSet {
            env_var: "HTP_CONFIG".into(),
            host_path: self.config_folder.0.clone(),
            inner_path: inner_htp_root.join("config"),
        });

        map.0.push(MountMapSet {
            env_var: "HTP_PERSIST".into(),
            host_path: self.persist_folder.0.clone(),
            inner_path: inner_htp_root.join("persist"),
        });
        map
    }
}

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub ver: String,
    pub spec: DependencySpecification,
    pub build_input_folder: HtpFolder,
    pub build_output_folder: HtpFolder,
}
impl Dependency {
    pub fn new(
        orchestrator_config: &OrchestratorConfig,
        name: &str,
        specification: &DependencySpecification,
    ) -> anyhow::Result<Self> {
        //TODO
        let ver = "VER";
        let build_input_folder = HtpFolder::new_dependency(
            orchestrator_config,
            DependencyFolderType::BuildInput,
            name,
            ver,
        )
        .context("failed to create build input folder")?;
        let build_output_folder = HtpFolder::new_dependency(
            orchestrator_config,
            DependencyFolderType::BuildOutput,
            name,
            ver,
        )
        .context("failed to create build output folder")?;
        Ok(Self {
            name: name.into(),
            ver: ver.into(),
            spec: specification.clone(),
            build_input_folder,
            build_output_folder,
        })
    }
    pub fn dependency_mount_map(&self, inner_mount_root: &PathBuf) -> EnvironmentMountMap {
        let mut map = EnvironmentMountMap::default();

        let inner_root_path = inner_mount_root
            .join("dependencies")
            .join(format!("{}-{}", self.name, self.ver));
        map.0.push(MountMapSet {
            env_var: "HTP_BUILD_INPUT".into(),
            host_path: self.build_input_folder.0.clone(),
            inner_path: inner_root_path.join("input"),
        });
        map.0.push(MountMapSet {
            env_var: "HTP_BUILD_OUTPUT".into(),
            host_path: self.build_output_folder.0.clone(),
            inner_path: inner_root_path.join("output"),
        });

        map
    }

    // TODO: add protection to ensure this isn't called multiple times at once
    pub async fn build(&self, build_target_type: &DeviceType) -> anyhow::Result<()> {
        if let DeviceClassification::Docker(spec) = &build_target_type.classification {
            log::info!("Starting docker container");
            let mount_map = self.dependency_mount_map(&spec.htp_root);
            let container_config = bollard::container::Config {
                image: Some(spec.image.clone()),
                tty: Some(true),
                host_config: Some(bollard::service::HostConfig {
                    binds: Some(mount_map.mount_points()?),
                    ..Default::default()
                }),
                ..Default::default()
            };
            let mut env = DockerEnvironment::new(&spec, container_config).await?;

            env.exec(bollard::exec::CreateExecOptions {
                env: Some(mount_map.env_vars()?),
                cmd: Some(vec![
                    "/bin/bash".into(),
                    "-c".into(),
                    self.spec.build_script.clone(),
                ]),
                ..Default::default()
            })
            .await?;
            // env.exec(&dep.spec.build_script).await.unwrap();
            env.shutdown().await?;
            return Ok(());
        }
        todo!()
    }
    pub async fn install_on(
        &self,
        spec: &DockerSpec,
        env: &mut DockerEnvironment,
    ) -> anyhow::Result<()> {
        let mount_map = self.dependency_mount_map(&spec.htp_root);
        env.exec(bollard::exec::CreateExecOptions {
            env: Some(mount_map.env_vars()?),
            cmd: Some(vec![
                "/bin/bash".into(),
                "-c".into(),
                self.spec.install_script.clone(),
            ]),
            ..Default::default()
        })
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnvironmentMountMap(pub Vec<MountMapSet>);
#[derive(Debug, Clone)]
pub struct MountMapSet {
    pub env_var: String,
    pub host_path: PathBuf,
    pub inner_path: PathBuf,
}
impl EnvironmentMountMap {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn env_vars(&self) -> anyhow::Result<Vec<String>> {
        let mut env_vars = Vec::new();
        for set in &self.0 {
            env_vars.push(format!(
                "{}={}",
                set.env_var,
                set.inner_path
                    .to_str()
                    .ok_or(anyhow!("Failed to convert inner path to str"))?
            ))
        }
        Ok(env_vars)
    }
    pub fn mount_points(&self) -> anyhow::Result<Vec<String>> {
        let mut mount_points = Vec::new();
        for set in &self.0 {
            mount_points.push(format!(
                "{}:{}",
                set.host_path
                    .to_str()
                    .ok_or(anyhow!("Cannot convert input path to str"))?,
                set.inner_path
                    .to_str()
                    .ok_or(anyhow!("cannot convert inner input path to str"))?
            ));
        }
        Ok(mount_points)
    }
}
