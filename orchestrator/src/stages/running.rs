use std::{
    collections::HashMap, marker::PhantomData, path::PathBuf, process::Command, time::SystemTime,
};

use anyhow::anyhow;
use bollard::service::PortBinding;
use crossbeam::channel::{Receiver, Sender};
use futures_util::__private::async_await;

use crate::{
    config::{device_types::DockerSpec, Config},
    environment::docker_env::DockerEnvironment,
    htp_test::{
        EnvironmentMountMap, HtpTest, MountMapSet, Queued, Runnable, Terminated, Validated,
    },
};

pub struct Runner {
    input: Receiver<HtpTest<Runnable>>,
    output: Sender<HtpTest<Terminated>>,
    output_terminated: Sender<HtpTest<Terminated>>,
}
impl Runner {
    pub fn new(
        input: Receiver<HtpTest<Runnable>>,
        output: Sender<HtpTest<Terminated>>,
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
        let Ok(mut to_run) = self.input.try_recv() else {
            return Ok(());
        };

        to_run.stats_sink.write("running", "started");
        let rund = to_run.run().await;
        match rund {
            Ok(mut rund) => {
                rund.stats_sink.write("running", "finished successfully");

                self.output.send(rund)?
            }
            Err(mut run_error) => {
                run_error.terminated.stats_sink.write("running", "failed");
                println!("Err: {}", run_error.msg);
                self.output_terminated.send(run_error.terminated)?
            }
        };
        println!("Ran");
        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Aquisition error: {msg}")]
pub struct RunningError {
    msg: String,
    #[source]
    source: anyhow::Error,
    terminated: HtpTest<Terminated>,
}

impl HtpTest<Runnable> {
    pub async fn run(mut self) -> Result<HtpTest<Terminated>, RunningError> {
        // TODO support non-docker
        let spec = DockerSpec {
            image: "ghcr.io/viamrobotics/canon:amd64-cache".into(),
            htp_root: "/htp".into(),
        };

        let test_mount_map = self.test_mount_map(&spec.htp_root);

        let mut mount_points = Vec::new();

        mount_points.append(&mut test_mount_map.clone().mount_points().unwrap());

        for dep in self.dependencies() {
            // TODO support non-docker
            let mount_map = dep.dependency_mount_map(&spec.htp_root);
            mount_points.append(&mut mount_map.mount_points().unwrap())
        }

        let mut ports = HashMap::new();
        ports.insert(
            "22".into(),
            Some(vec![PortBinding {
                host_ip: Some("localhost".into()),
                host_port: Some("4321".into()),
            }]),
        );
        let container_config = bollard::container::Config {
            image: Some(spec.image.clone()),
            tty: Some(true),
            host_config: Some(bollard::service::HostConfig {
                binds: Some(mount_points),
                port_bindings: Some(ports),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut env = DockerEnvironment::new(&spec, container_config)
            .await
            .unwrap();
        for dep in self.dependencies() {
            // TODO support non-docker
            let install_result = dep.install_on(&spec, &mut env).await;
            if let Err(install_result) = install_result {
                return Err(RunningError {
                    msg: "Failed to install dep".into(),
                    source: install_result,
                    terminated: self.clone_into(),
                });
            }
        }
        let command = &self.get_test_spec().on_device_test_script.as_ref().unwrap();

        env.exec(bollard::exec::CreateExecOptions {
            env: Some(test_mount_map.env_vars().unwrap()),
            working_dir: Some(spec.htp_root.join("config").to_str().unwrap().into()),
            cmd: Some(vec![
                "/usr/bin/env".into(),
                "bash".into(),
                "-c".into(),
                // "sleep 1000".into()
                command.clone().into(),
            ]),
            ..Default::default()
        })
        .await
        .unwrap();
        env.shutdown().await.unwrap();
        Ok(self.clone_into())
    }
}
