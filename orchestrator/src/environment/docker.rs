use super::Environment;

use bollard::image::BuildImageOptions;
use bollard::Docker;
use futures_util::stream::StreamExt;
use hyper::body::Body;

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use std::env::args;
use tokio::time::{sleep, Duration};
pub struct DockerEnv {}
impl DockerEnv {
    pub fn new() -> Self {
        Self {}
    }
}
impl Environment for DockerEnv {
    fn setup(&mut self) {
        todo!()
    }
    fn run_script(&mut self, cmd: &str) {
        todo!()
    }
    fn cleanup(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn create() {
        let docker = Docker::connect_with_socket_defaults().unwrap();

let images = &docker.list_images(Some(bollard::image::ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    for image in images {
        println!("-> {:?}", image);
    }
        let image_options = BuildImageOptions {
            dockerfile: "Dockerfile",
            t: "rust-test",
            rm: true,
            ..Default::default()
        };
        println!("STArted");
        let filename = "Dockerfile";
        let archive = File::open(filename).await.expect("could not open file");
        let stream = FramedRead::new(archive, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        println!("wrapped");

        let mut image_build_stream = docker.build_image(image_options, None, Some(body));
        println!("vui");
    sleep(Duration::from_millis(1000)).await;

        while let Some(msg) = image_build_stream.next().await {
            println!("Message: {:?}", msg);
        }
        let env = DockerEnv::new();
    }
    #[tokio::test]
    async fn supercre(){
 let docker = Docker::connect_with_socket_defaults().unwrap();

    let mut build_image_args = HashMap::new();
    build_image_args.insert("dummy", "value");

    let mut build_image_labels = HashMap::new();
    build_image_labels.insert("maintainer", "somemaintainer");

    let build_image_options = BuildImageOptions {
        dockerfile: "Dockerfile",
        t: "bollard-build-example",
        extrahosts: Some("myhost:127.0.0.1"),
        remote:
            "https://raw.githubusercontent.com/viamrobotics/govanity/main/Dockerfile",
        q: false,
        nocache: false,
        cachefrom: vec![],
        pull: true,
        rm: true,
        forcerm: true,
        memory: Some(120000000),
        memswap: None,
        cpushares: Some(2),
        cpusetcpus: "0-3",
        cpuperiod: Some(2000),
        cpuquota: Some(1000),
        buildargs: build_image_args,
        shmsize: Some(1000000),
        squash: false,
        labels: build_image_labels,
        networkmode: "host",
        platform: "linux/x86_64",
        #[cfg(feature = "buildkit")]
        session: None,
        version: bollard::image::BuilderVersion::BuilderV1,
    };

    let mut image_build_stream = docker.build_image(build_image_options, None, None);

    while let Some(msg) = image_build_stream.next().await {
        println!("Message: {:?}", msg);
    }
    }
}
