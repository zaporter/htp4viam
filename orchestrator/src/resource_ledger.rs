use anyhow::anyhow;

use crate::htp_test::TestID;

// The ResourceLedger is a very interesting data structure
// because it is designed to allow communication between tests
// without trying too hard to prevent collisions.
// It is more of a publish-first, ask-never kind of architecture
// that is very amenable to greedy resource allocation.
#[derive(Default, Debug, Clone)]
pub struct ResourceLedger {
    // This could be done with a HashMap
    // but it would be more complex and probably slower
    //
    // testid (usize), resourceid, is_exclusively_locked
    resources: Vec<(TestID, String, bool)>,
}

impl ResourceLedger {
    pub fn allocated_count(&self, resource: &str) -> usize {
        let mut count = 0;
        for res in &self.resources {
            if res.1 == resource {
                count += 1;
            }
        }
        count
    }
    pub fn is_exclusively_locked(&self, resource: &str) -> bool {
        for res in &self.resources {
            if res.1 == resource && res.2 {
                return true;
            }
        }
        false
    }
    pub fn get_owners(&self, resource: &str) -> Vec<TestID> {
        let mut owners = Vec::new();
        for res in &self.resources {
            if res.1 == resource {
                owners.push(res.0);
            }
        }
        owners
    }

    pub fn acquire_resource(
        &mut self,
        testid: TestID,
        resource: &str,
        get_exclusive: bool,
    ) -> anyhow::Result<()> {
        if get_exclusive && self.allocated_count(resource) != 0 {
            return Err(anyhow!(
                "Tried to get an exclusive lock on {resource} by {testid} but it is already being used"
            ));
        }
        if self.is_exclusively_locked(resource) {
            return Err(anyhow!(
                "Tried to get a lock on {resource} by {testid} which is already exclusively locked"
            ));
        }
        if self.get_owners(resource).contains(&testid) {
            return Err(anyhow!(
                "Tried to get a lock on {resource} by {testid} which it already has aquired"
            ));
        }

        self.resources
            .push((testid, resource.into(), get_exclusive));
        Ok(())
    }
    pub fn release_resource(&mut self, testid: TestID, resource: &str) -> anyhow::Result<()> {
        let mut resource_index = None;
        for (idx, res) in self.resources.iter().enumerate() {
            if res.0 == testid && res.1 == resource {
                resource_index = Some(idx)
            }
        }
        if resource_index.is_none() {
            return Err(anyhow!(
                "TestID {testid} tried to free resource {resource} which it never owned"
            ));
        }
        self.resources.swap_remove(resource_index.unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocated_count() {
        let mut ledger = ResourceLedger::default();
        let testid1: TestID = 1;
        let testid2: TestID = 2;
        let resource = "resource1";

        ledger.acquire_resource(testid1, resource, false).unwrap();
        assert_eq!(ledger.allocated_count(resource), 1);

        ledger.acquire_resource(testid2, resource, false).unwrap();
        assert_eq!(ledger.allocated_count(resource), 2);

        ledger.release_resource(testid1, resource).unwrap();
        assert_eq!(ledger.allocated_count(resource), 1);

        ledger.release_resource(testid2, resource).unwrap();
        assert_eq!(ledger.allocated_count(resource), 0);
    }

    #[test]
    fn test_is_exclusively_locked() {
        let mut ledger = ResourceLedger::default();
        let testid: TestID = 1;
        let resource = "resource1";

        assert!(!ledger.is_exclusively_locked(resource));

        ledger.acquire_resource(testid, resource, true).unwrap();
        assert!(ledger.is_exclusively_locked(resource));

        ledger.release_resource(testid, resource).unwrap();
        assert!(!ledger.is_exclusively_locked(resource));
    }

    #[test]
    fn test_get_owners() {
        let mut ledger = ResourceLedger::default();
        let testid1: TestID = 1;
        let testid2: TestID = 2;
        let resource = "resource1";

        ledger.acquire_resource(testid1, resource, false).unwrap();
        assert_eq!(ledger.get_owners(resource), vec![testid1]);

        ledger.acquire_resource(testid2, resource, false).unwrap();
        assert_eq!(ledger.get_owners(resource), vec![testid1, testid2]);

        ledger.release_resource(testid1, resource).unwrap();
        assert_eq!(ledger.get_owners(resource), vec![testid2]);
    }

    #[test]
    fn test_acquire_resource() {
        let mut ledger = ResourceLedger::default();
        let testid1: TestID = 1;
        let testid2: TestID = 2;
        let resource = "resource1";

        ledger.acquire_resource(testid1, resource, false).unwrap();
        assert_eq!(ledger.allocated_count(resource), 1);

        let result = ledger.acquire_resource(testid1, resource, false);
        assert!(result.is_err());

        let result = ledger.acquire_resource(testid1, resource, true);
        assert!(result.is_err());

        ledger.acquire_resource(testid2, resource, false).unwrap();
        assert_eq!(ledger.allocated_count(resource), 2);
    }

    #[test]
    fn test_release_resource() {
        let mut ledger = ResourceLedger::default();
        let testid1: TestID = 1;
        let testid2: TestID = 2;
        let resource = "resource1";

        ledger.acquire_resource(testid1, resource, false).unwrap();
        assert_eq!(ledger.allocated_count(resource), 1);

        let result = ledger.release_resource(testid2, resource);
        assert!(result.is_err());

        ledger.release_resource(testid1, resource).unwrap();
        assert_eq!(ledger.allocated_count(resource), 0);
    }
}
