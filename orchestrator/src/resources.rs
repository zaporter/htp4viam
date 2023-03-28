use std::collections::{HashMap, HashSet};

use anyhow::anyhow;

use crate::config::{
    apparatuses::{Apparatus, ApparatusMap},
    devices::{Device, DeviceMap},
    Config,
};

// The ResourceCollection is the owner and core key/lock mechanism
// for test-orchestration
pub struct ResourceCollection<'a> {
    all_devices: &'a DeviceMap,
    all_apparatuses: &'a ApparatusMap,
    owned_devices: HashSet<String>,
    owned_apparatues: HashSet<String>,
}

impl<'a> ResourceCollection<'a> {
    pub fn new(config: &'a Config) -> ResourceCollection<'a> {
        ResourceCollection {
            all_devices: &config.devices,
            all_apparatuses: &config.apparatuses,
            owned_devices: HashSet::new(),
            owned_apparatues: HashSet::new(),
        }
    }
    pub fn insert_device(&mut self, device: &str) -> anyhow::Result<()> {
        if !self.all_devices.contains_key(device) {
            return Err(anyhow!(
                "Resource collection does not contain mapping for device {device}"
            ));
        }
        if self.owned_devices.contains(device) {
            return Err(anyhow!(
                "Resource collection already contains device {device}"
            ));
        }

        self.owned_devices.insert(device.to_string());
        return Ok(());
    }
    pub fn insert_apparatus(&mut self, apparatus: &str) -> anyhow::Result<()> {
        if !self.all_apparatuses.contains_key(apparatus) {
            return Err(anyhow!(
                "Resource collection does not contain mapping for apparatus {apparatus}"
            ));
        }
        let apparatus_meta = self.all_apparatuses.get(apparatus).unwrap();
        if self.owned_apparatues.contains(apparatus) && apparatus_meta.is_exclusively_locked {
            return Err(anyhow!(
                "Resource collection already contains apparatus {apparatus} and it is supposed to be exclusive"
            ));
        }

        self.owned_apparatues.insert(apparatus.to_string());
        return Ok(());
    }

    // If we have a device to give, return Some(device). This does not also get the apparatus.
    // however any returned device must also have the desired apparatus connected to it.
    pub fn take_device(&mut self, device_type: &str, required_apparatus: &str) -> Option<String> {
        for (device_name, device) in self.all_devices {
            if device.device_type == device_type
                && device
                    .connected_apparatuses
                    .contains(&required_apparatus.to_string())
                && self.owned_devices.contains(device_name)
            {
                // Remove our ownership and give up ownership
                self.owned_devices.remove(device_name);
                return Some(device_name.to_string());
            }
        }
        return None;
    }

    pub fn take_apparatus(&mut self, required_apparatus: &str) -> Option<String> {
        let apparatus_meta = self.all_apparatuses.get(required_apparatus);
        if let Some(apparatus_meta) = apparatus_meta {
            if !self.owned_apparatues.contains(required_apparatus) {
                return None;
            }
            if apparatus_meta.is_exclusively_locked {
                self.owned_apparatues.remove(required_apparatus);
            }
            return Some(required_apparatus.to_string());
        }
        // maybe panic? Shouldnt be possible given validation
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> ResourceCollection<'static> {
        let mut config = Config::default();
        config.devices.insert(
            "dev1".into(),
            Device {
                device_type: "linux".into(),
                login_username: "test".into(),
                connected_apparatuses: vec!["app1".into()],
            },
        );
        config.devices.insert(
            "dev2".into(),
            Device {
                device_type: "linux".into(),
                login_username: "test".into(),
                connected_apparatuses: vec!["app1".into(), "app2".into()],
            },
        );
        config.apparatuses.insert(
            "app1".into(),
            Apparatus {
                is_exclusively_locked: true,
                peripherals: vec![],
                wrapped_apparatuses: vec![],
            },
        );
        config.apparatuses.insert(
            "app2".into(),
            Apparatus {
                is_exclusively_locked: true,
                peripherals: vec![],
                wrapped_apparatuses: vec![],
            },
        );
        ResourceCollection::new(Box::leak(Box::new(config)))
    }

    #[test]
    fn test_insert_device() {
        let mut rc = setup();
        assert!(rc.insert_device("dev1").is_ok());
        assert_eq!(rc.owned_devices.contains("dev1"), true);
    }

    #[test]
    fn test_insert_device_already_owned() {
        let mut rc = setup();
        rc.insert_device("dev1").unwrap();
        assert!(rc.insert_device("dev1").is_err());
    }

    #[test]
    fn test_insert_device_not_present() {
        let mut rc = setup();
        assert!(rc.insert_device("dev_invalid").is_err());
    }

    #[test]
    fn test_insert_apparatus() {
        let mut rc = setup();
        assert!(rc.insert_apparatus("app1").is_ok());
        assert_eq!(rc.owned_apparatues.contains("app1"), true);
    }

    #[test]
    fn test_insert_apparatus_already_owned() {
        let mut rc = setup();
        rc.insert_apparatus("app1").unwrap();
        assert!(rc.insert_apparatus("app1").is_err());
    }

    #[test]
    fn test_insert_apparatus_not_present() {
        let mut rc = setup();
        assert!(rc.insert_apparatus("app_invalid").is_err());
    }

    #[test]
    fn test_take_device() {
        let mut rc = setup();
        rc.insert_device("dev1").unwrap();
        let device = rc.take_device("linux", "app1");
        assert_eq!(device, Some("dev1".to_string()));
        assert_eq!(rc.owned_devices.contains("dev1"), false);
    }

    #[test]
    fn test_take_device_no_suitable_device() {
        let mut rc = setup();
        let device = rc.take_device("linux", "app_invalid");
        assert_eq!(device, None);
    }

    #[test]
    fn test_take_apparatus() {
        let mut rc = setup();
        rc.insert_apparatus("app1").unwrap();
        let apparatus = rc.take_apparatus("app1");
        assert_eq!(apparatus, Some("app1".to_string()));
        assert_eq!(rc.owned_apparatues.contains("app1"), false);
    }

    #[test]
    fn test_take_apparatus_not_owned() {
        let mut rc = setup();
        let apparatus = rc.take_apparatus("app1");
        assert_eq!(apparatus, None);
    }

    #[test]
    fn test_take_apparatus_not_present() {
        let mut rc = setup();
        let apparatus = rc.take_apparatus("app_invalid");
        assert_eq!(apparatus, None);
    }
}
