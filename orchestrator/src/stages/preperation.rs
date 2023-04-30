use std::{marker::PhantomData, time::SystemTime};

use anyhow::anyhow;
use crossbeam::channel::{Receiver, Sender};

use crate::{
    config::{
        device_types::{DeviceClassification, DockerSpec},
        Config,
    },
    environment::docker_env::DockerEnvironment,
    htp_test::{HtpTest, Prepared, Queued, Runnable, Terminated, Validated},
};

pub struct Preparer {
    input: Receiver<HtpTest<Validated>>,
    output: Sender<HtpTest<Prepared>>,
    output_terminated: Sender<HtpTest<Terminated>>,
}
impl Preparer {
    pub fn new(
        input: Receiver<HtpTest<Validated>>,
        output: Sender<HtpTest<Prepared>>,
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
    pub async fn process_one(&mut self) -> anyhow::Result<()> {
        let Ok(mut to_prepare) = self.input.try_recv() else {
            return Ok(());
        };

        to_prepare.stats_sink.write("preperation", "started");
        let prepared = to_prepare.prepare().await;
        match prepared {
            Ok(mut prepared) => {
                prepared
                    .stats_sink
                    .write("preperation", "finished successfully");

                self.output.send(prepared)?
            }
            Err(mut prepare_error) => {
                prepare_error
                    .terminated
                    .stats_sink
                    .write("preperation", "failed");
                println!("Err: {}", prepare_error.msg);
                self.output_terminated.send(prepare_error.terminated)?
            }
        };
        println!("Prepared");
        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Preperation error: {msg}")]
pub struct PreperationError {
    msg: String,
    #[source]
    source: anyhow::Error,
    terminated: HtpTest<Terminated>,
}

impl HtpTest<Validated> {
    pub async fn prepare(self) -> Result<HtpTest<Prepared>, PreperationError> {
        // build input should already be created. We will be building the dependencies

        // TODO Clean up this mess to dep.build() requires a mut reference to dep
        for dep in self.dependencies() {
            let build_target = self
                .config()
                .device_types
                .get(&dep.spec.build_on)
                .ok_or(anyhow!("Failed to find device type"))
                .unwrap();
            // validation ensures this exists
            let build_result = dep.build(&build_target).await;
            if let Err(build_result) = build_result {
                return Err(PreperationError {
                    msg: "Failed to build dep".into(),
                    source: build_result,
                    terminated: self.clone_into(),
                });
            }
        }

        Ok(self.clone_into())
    }
}
