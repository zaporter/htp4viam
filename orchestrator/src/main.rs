use std::{path::PathBuf, time::Duration};

use crate::{config::Config, orchestrator::Orchestrator};
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, IndexParts};
use std::process;
use tokio::runtime::Runtime;
use tracing_loki::url::Url;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use env_logger::Env;
use serde_json::{json, Value};

mod htp_test;
mod keygen;
mod ssh;
mod stages;
//mod git;
mod config;
mod environment;
mod folder;
mod orchestrator;
mod resource_ledger;
mod resources;
mod running_test_map;
mod statistics;
mod test_queue;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::info!("Started");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let transport = Transport::single_node("http://localhost:3060")?;
    let client = Elasticsearch::new(transport);
    let response = client
        .index(IndexParts::IndexId("tweets", "3a"))
        .body(json!({
            "user": "joe",
            "tag":"sam",
            "timestamp": "2023-05-09T10:00:00Z",
            "message": "Trying out Elasticsearch, so far so good?"
        }))
        .send()
        .await?;

    let successful = response.status_code().is_success();
    dbg!(successful);
    log::info!("Finished");
    Ok(())
}
// pub fn main() {
//     log::info!("Started");
//     env_logger::init_from_env(Env::default().default_filter_or("info"));
//     // println!("{:?}", config);
//     // run().unwrap();
//     let mut orchestrator = Orchestrator::new();
//     orchestrator.start().unwrap();
//     while !orchestrator.is_finished() {
//         std::thread::sleep(Duration::from_millis(5000));
//     }
//     orchestrator.stop().unwrap();
//     log::info!("Finished");
// }

fn run() -> anyhow::Result<()> {
    let runtime = Runtime::new()?;
    let handle = runtime.handle();
    let (layer, task) = tracing_loki::builder()
        .label("host", "mine")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3030").unwrap())?;

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(layer)
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        .init();

    // The background task needs to be spawned so the logs actually get
    // delivered.
    let join = handle.spawn(task);

    tracing::info!(
        label = "tracing_setup",
        result = "success",
        "tracing successfully set up yay",
    );
    tracing::info!(
        task = "tracing_setup",
        result = "success",
        "tracing successfully set up",
    );

    //  runtime.block_on(async {join.await})?;

    Ok(())
}
