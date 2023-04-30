use std::{marker::PhantomData, time::SystemTime};

use anyhow::anyhow;
use crossbeam::channel::{Receiver, Sender};

use crate::{
    config::Config,
    htp_test::{Dependency, HtpTest, Queued, Terminated, Validated},
};

pub struct Validator {
    input: Receiver<HtpTest<Queued>>,
    output: Sender<HtpTest<Validated>>,
    output_terminated: Sender<HtpTest<Terminated>>,
}
impl Validator {
    pub fn new(
        input: Receiver<HtpTest<Queued>>,
        output: Sender<HtpTest<Validated>>,
        output_terminated: Sender<HtpTest<Terminated>>,
    ) -> Self {
        Self {
            input,
            output,
            output_terminated,
        }
    }
    // Dont put a value greater than 5sec. That would be stupid
    pub fn desired_poll_delay(&mut self) -> tokio::time::Duration {
        tokio::time::Duration::from_millis(100)
    }
    pub fn process_one(&mut self) -> anyhow::Result<()> {
        let Ok(mut to_validate) = self.input.try_recv() else {
            return Ok(());
        };

        to_validate.stats_sink.write("validation", "started");
        let validated = to_validate.validate();
        //TODO(is_ok) is wrong
        match validated {
            Ok(mut validated) => {
                validated
                    .stats_sink
                    .write("validation", "finished successfully");

                self.output.send(validated)?
            }
            Err(mut validate_error) => {
                validate_error
                    .terminated
                    .stats_sink
                    .write("validation", "failed");
                log::error!("Err: {:?}", validate_error);
                self.output_terminated.send(validate_error.terminated)?
            }
        };
        println!("Validated");
        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Validation error: {msg}")]
pub struct ValidateError {
    msg: String,
    #[source]
    source: anyhow::Error,
    terminated: HtpTest<Terminated>,
}

impl HtpTest<Queued> {
    pub fn validate(mut self) -> Result<HtpTest<Validated>, ValidateError> {
        // TODO
        // Actually start it...
        let config = Config::new(&self.config_folder.0);
        match config {
            Ok(config) => {
                // Ensure we can run get_test_group()
                // and get_test_spec() with this config
                let test_group = config.tests.get(&self.test_spec_id.0);
                let test_group = match test_group {
                    Some(k) => k,
                    None => {
                        return Err(ValidateError {
                            msg: "Unable to find test group for test specification".into(),
                            source: anyhow!("Config creation"),
                            terminated: self.clone_into(),
                        });
                    }
                };
                let test_specification = test_group.get_test(&self.test_spec_id.1);
                let test_specification = match test_specification {
                    Some(k) => k,
                    None => {
                        return Err(ValidateError {
                            msg: "Unable to find test specification in test group".into(),
                            source: anyhow!("Config creation"),
                            terminated: self.clone_into(),
                        });
                    }
                };
                // prepare dependencies
                let mut dependencies = Vec::new();
                for dep_name in test_specification.dependencies.keys() {
                    self.stats_sink
                        .write("validation", format!("Creating dependency on {}", dep_name));
                    let dep_spec = config.dependencies.get(dep_name);
                    let dep_spec = match dep_spec {
                        Some(k) => k,
                        None => {
                            return Err(ValidateError {
                                msg: "Unable to find specified dependency".into(),
                                source: anyhow!("Config creation"),
                                terminated: self.clone_into(),
                            });
                        }
                    };
                    let dep = Dependency::new(&self.orchestrator_config, dep_name, dep_spec);
                    match dep {
                        Ok(dep) => dependencies.push(dep),
                        Err(err) => {
                            return Err(ValidateError {
                                msg: "Unable to find create a dependency".into(),
                                source: err,
                                terminated: self.clone_into(),
                            });
                        }
                    }
                }
                for dep in &dependencies {
                    if !config.device_types.contains_key(&dep.spec.build_on) {
                        return Err(ValidateError {
                            msg: "Dependency is trying to run on a device_type that doesn't exist"
                                .into(),
                            source: anyhow!("Config creation"),
                            terminated: self.clone_into(),
                        });
                    }
                }
                self.dependencies = Some(dependencies);
                // set the config
                self.config = Some(config);

                Ok(self.clone_into())
            }
            Err(err) => Err(ValidateError {
                msg: "Config creation failed".into(),
                source: err,
                terminated: self.clone_into(),
            }),
        }
    }
}
