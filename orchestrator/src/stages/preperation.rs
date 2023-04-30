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
        for dep in self.dependencies() {
            // validation ensures this exists
            let device_type = self.config().device_types.get(&dep.spec.build_on).unwrap();
            if let DeviceClassification::Docker(spec) = &device_type.classification {
                log::info!("Starting docker container");
                let mut env_vars = Vec::new();
                let mut mount_points = Vec::new();
                {
                    let inner_input_path = &spec.htp_root.join("input");
                    env_vars.push(format!(
                        "HTP_BUILD_INPUT={}",
                        inner_input_path.to_str().unwrap()
                    ));
                    mount_points.push(format!(
                        "{}:{}",
                        dep.build_input_folder.0.to_str().unwrap(),
                        inner_input_path.to_str().unwrap()
                    ));
                }
                {
                    let inner_output_path = &spec.htp_root.join("output".to_owned());
                    env_vars.push(format!(
                        "HTP_BUILD_OUTPUT={}",
                        inner_output_path.to_str().unwrap()
                    ));
                    mount_points.push(format!(
                        "{}:{}",
                        dep.build_output_folder.0.to_str().unwrap(),
                        inner_output_path.to_str().unwrap()
                    ));
                }
                let container_config = bollard::container::Config {
                    image: Some(spec.image.clone()),
                    env: Some(env_vars),
                    tty: Some(true),
                    host_config: Some(bollard::service::HostConfig {
                        binds: Some(mount_points),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                let mut env = DockerEnvironment::new(&spec, container_config)
                    .await
                    .unwrap();
                env.exec(&dep.spec.build_script).await.unwrap();
                env.shutdown().await.unwrap();
            }
        }

        Ok(self.clone_into())
    }
}
