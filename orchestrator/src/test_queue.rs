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
    config_folder: RcFolder,
    config: Config,
    test_spec_id: TestSpecificationID,
    priority: TestPriority,
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
    meta: TestMetadata,
    id: TestID,
    queue_start_time: SystemTime,
    execution_start_time: Option<SystemTime>,
    execution_end_time: Option<SystemTime>,

    acquired_device_ids: Vec<String>,
    acquired_apparatus_ids: Vec<String>,

    was_terminated_before_finished: Option<bool>,

    stage: std::marker::PhantomData<Stage>,
}
impl<Stage> HtpTest<Stage> {
    fn get_test_group(&self) -> Option<&TestGroup> {
        self.meta.config.tests.get(&self.meta.test_spec_id.0)
    }
    fn get_test_spec(&self) -> Option<&TestSpecification> {
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

impl HtpTest<Queued> {
    pub fn device_dependencies_satisfied(&self) -> bool {
        self.acquired_device_ids.len() > 0
    }
    pub fn try_acquire_devices(&mut self, device_ledger: &mut ResourceLedger) {
        // checked to be !none in new()
        let test_spec = (*self.get_test_spec().unwrap()).clone();
        for (device_id, device) in &self.meta.config.devices {
            if self.device_dependencies_satisfied() {
                return;
            }
            if device.connected_apparatuses.contains(&test_spec.apparatus) {
                let success = device_ledger
                    .acquire_resource(self.id, &device_id, true)
                    .is_ok();
                if success {
                    self.acquired_device_ids.push(device_id.clone());
                }
            }
        }
    }

    pub fn apparatus_dependencies_satisfied(&self) -> bool {
        self.acquired_apparatus_ids.len() > 0
    }
    pub fn try_acquire_apparatuses(&mut self, apparatus_ledger: &mut ResourceLedger) {
        // checked to be !none in new()
        let test_spec = self.get_test_spec().unwrap();

        if self.apparatus_dependencies_satisfied() {
            return;
        }

        let success = apparatus_ledger
            .acquire_resource(self.id, &test_spec.apparatus, true)
            .is_ok();
        if success {
            self.acquired_device_ids.push(test_spec.apparatus.clone());
        }
    }
    pub fn can_start(&self) -> bool {
        self.apparatus_dependencies_satisfied() && self.device_dependencies_satisfied()
    }
    pub fn try_start(self) -> anyhow::Result<HtpTest<Running>> {
        // TODO
        // Actually start it...
        Ok(HtpTest {
            meta: self.meta,
            id: self.id,
            queue_start_time: self.queue_start_time,
            execution_start_time: Some(SystemTime::now()),
            execution_end_time: self.execution_end_time,
            acquired_device_ids: self.acquired_device_ids,
            acquired_apparatus_ids: self.acquired_apparatus_ids,
            was_terminated_before_finished: self.was_terminated_before_finished,
            stage: PhantomData::default(),
        })
    }
}
impl HtpTest<Running> {
    pub fn is_finished(&self) -> bool {
        todo!()
    }
    pub fn terminate(&mut self) -> anyhow::Result<HtpTest<Terminated>> {
        if !self.is_finished() {
            self.was_terminated_before_finished = Some(true);
        }
        self.execution_end_time = Some(SystemTime::now());
        todo!()
    }
}
impl HtpTest<Terminated> {}

// #[derive(Default, Debug)]
// struct TestOrchestrator {
//     device_ledger: ResourceLedger,
//     apparatus_ledger: ResourceLedger,
//     // All tests of priority 0 go in slot 0,
//     // all tests of priority 1 go in slot 1 ...
//     queued_tests: Vec<Vec<TestMetadata>>,
//     running_tests: Vec<RunningTest>,
//     finished_tests: Vec<RunningTest>,
// }

// impl TestOrchestrator {
//     pub fn add_to_queue(&mut self, test: TestMetadata) -> anyhow::Result<()> {
//         let priority_idx = test.priority.0;
//         if self.queued_tests.get(priority_idx).is_none() {
//             return Err(anyhow::anyhow!("Invalid priority level {}", priority_idx));
//         }
//         self.queued_tests[priority_idx].push(test);
//         Ok(())
//     }

//     pub fn start_possible_tests(&mut self) -> anyhow::Result<()> {
//         for i in (0..5).rev() {
//             if let Some(test) = self.queued_tests[i]
//                 .iter()
//                 .find(|t| self.free_resources.contains(&t.required_resources))
//             {
//                 let test_idx = self.queued_tests[i].iter().position(|t| t == test).unwrap();
//                 let test = self.queued_tests[i].remove(test_idx);
//                 let running_test = RunningTest {
//                     meta: test,
//                     start_time: SystemTime::now(),
//                     end_time: None,
//                 };
//                 self.running_tests.push(running_test);
//                 self.free_resources = self
//                     .free_resources
//                     .difference(&running_test.meta.required_resources);
//             }
//         }
//         Ok(())
//     }

//     pub fn poll_running_tests(&mut self) -> anyhow::Result<()> {
//         let now = SystemTime::now();
//         let mut idx_to_remove = vec![];
//         for i in 0..self.running_tests.len() {
//             let test = &mut self.running_tests[i];
//             if let Some(end_time) = test.meta.test.max_duration_sec.map(|d| test.start_time + std::time::Duration::from_secs(d)) {
//                 if now >= end_time {
//                     test.end_time = Some(now);
//                     idx_to_remove.push(i);
//                     self.free_resources = self.free_resources.union(&test.meta.required_resources);
//                 }
//             }
//             else if let Some(end_time) = test.end_time {
//                 let finished_test = self.running_tests.remove(i);
//                 idx_to_remove.push(i);
//                 self.finished_tests.push(finished_test);
//                 self.free_resources = self.free_resources.union(&finished_test.meta.required_resources);
//             }
//         }
//         for i in idx_to_remove.iter().rev() {
//             self.running_tests.remove(*i);
//         }
//         Ok(())
//     }

//     pub fn terminate_all_tests(&mut self) -> anyhow::Result<()> {
//         self.running_tests.clear();
//         self.queued_tests.iter_mut().for_each(|q| q.clear());
//         self.free_resources.clear();
//         Ok(())
//     }
// }
