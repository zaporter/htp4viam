use std::time::SystemTime;

use crate::{rcfolder::RcFolder, resources::ResourceCollection, config::{Config, tests::TestSpecification}};


pub struct TestPriority(usize, &'static str);

pub const PRIORITY_ADMIN : TestPriority = TestPriority(0, "Admin (Manual)");
// When a human wants to run a single test on a single device
pub const PRIORITY_MANUAL_ONESHOT : TestPriority = TestPriority(1, "Highest (Manual)");
// When a human wants to run a test on many devices
pub const PRIORITY_MANUAL : TestPriority = TestPriority(2, "High (Manual)");
// If this ever gets used for CI
pub const PRIORITY_CI : TestPriority = TestPriority(3, "Medium (CI)");
// Constant automatic background checks when dependencies update
pub const PRIORITY_CANARY : TestPriority = TestPriority(4, "Low (Canary)");

pub type TestID = usize;

pub struct TestMetadata {
    config_folder: RcFolder,
    test: TestSpecification,
    priority: TestPriority,
    id : TestID,
}

pub struct RunningTest {
    meta : TestMetadata,
    start_time : SystemTime,
    end_time : Option<SystemTime>,
}

impl RunningTest{
    pub fn start(meta: TestMetadata) -> anyhow::Result<Self> {
        todo!()
        // omitted
    }
    pub fn terminate() -> anyhow::Result<()> {
        todo!()
        // omitted
    }
}

// struct TestOrchestrator {
//     free_resources: ResourceCollection,
//     // All tests of priority 0 go in slot 0,
//     // all tests of priority 1 go in slot 1 ...
//     queued_tests: Vec<Vec<TestMetadata>>,
//     running_tests : Vec<RunningTest>,
//     finished_tests : Vec<RunningTest>
// }

// impl TestOrchestrator {
//     // all_resources is a Vec<Resource> containing all of the
//     // resources available to the orchestrator
//     pub fn new(all_resources : ResourceList) -> Self {
//         TestOrchestrator {
//             free_resources: all_resources,
//             queued_tests: vec![vec![], vec![], vec![], vec![], vec![]],
//             running_tests: vec![],
//             finished_tests: vec![]
//         }
//     }
    
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
//             if let Some(test) = self.queued_tests[i].iter().find(|t| self.free_resources.contains(&t.required_resources)) {
//                 let test_idx = self.queued_tests[i].iter().position(|t| t == test).unwrap();
//                 let test = self.queued_tests[i].remove(test_idx);
//                 let running_test = RunningTest {
//                     meta: test,
//                     start_time: SystemTime::now(),
//                     end_time: None
//                 };
//                 self.running_tests.push(running_test);
//                 self.free_resources = self.free_resources.difference(&running_test.meta.required_resources);
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
