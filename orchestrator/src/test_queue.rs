use std::{marker::PhantomData, time::SystemTime};

use anyhow::anyhow;

use crate::{
    config::{
        tests::{TestGroup, TestSpecification, TestSpecificationID},
        Config,
    },
    htp_test::{HtpTest, Queued, Running, Terminated, TestID},
    rcfolder::RcFolder,
    resource_ledger::ResourceLedger,
    resources::ResourceCollection,
};

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
