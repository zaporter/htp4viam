use std::collections::HashMap;

use bollard::container::{Config, RemoveContainerOptions};
use bollard::Docker;

use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;

use crate::config::device_types::DockerSpec;

pub struct DockerEnvironment {
    container_id: String,
}
impl DockerEnvironment {
    pub async fn new(spec: &DockerSpec, container_config: Config<String>) -> anyhow::Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;
        log::info!("Creating docker image");

        docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: spec.image.clone(),
                    ..Default::default()
                }),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await?;

        log::info!("Creating docker container");

        log::info!("Creating container");
        let id = docker
            .create_container::<&str, String>(None, container_config)
            .await?
            .id;
        log::info!("Starting docker container");
        docker.start_container::<String>(&id, None).await?;
        Ok(Self { container_id: id })
    }
    // This does not have to be mutable but I am using the borrow checker to ensure
    // this isn't concurrently modified
    pub async fn exec_simple(&mut self, command: &str) -> anyhow::Result<()> {
        self.exec(CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(vec!["/bin/bash".into(), "-c".into(), command.into()]),
            ..Default::default()
        })
        .await
    }
    pub async fn exec(&mut self, mut options: CreateExecOptions<String>) -> anyhow::Result<()> {
        options.attach_stdout = Some(true);
        options.attach_stderr = Some(true);
        log::info!("Executing {:?} in docker container", options.cmd);
        let docker = Docker::connect_with_socket_defaults()?;
        let exec = docker.create_exec(&self.container_id, options).await?.id;
        if let StartExecResults::Attached { mut output, .. } =
            docker.start_exec(&exec, None).await?
        {
            while let Some(Ok(msg)) = output.next().await {
                print!("{}", msg);
            }
        } else {
            unreachable!();
        }
        Ok(())
    }
    // This does not have to be mutable but I am using the borrow checker to ensure
    // this isn't concurrently modified
    pub async fn shutdown(self) -> anyhow::Result<()> {
        log::info!("Shutting down docker container");
        let docker = Docker::connect_with_socket_defaults()?;
        docker
            .remove_container(
                &self.container_id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }
}
// pub async fn main2() -> Result<(), Box<dyn std::error::Error + 'static>> {
//     log::info!("Started main2");
//     log::info!("Container started");

//     // non interactive

//     log::info!("Shuttuing down container");

//     Ok(())
// }
