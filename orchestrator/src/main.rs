
use anyhow::{anyhow, Context, Result};
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use log::{debug, error, info, trace, warn};
use env_logger::{Builder, Env};

const REPO_URL: &str = "https://github.com/zaporter/co2meter-rs";
const LOCAL_PATH: &str = "./sample4";

async fn clone_and_pull_updates() -> Result<()> {
    info!("Cloning repo: {REPO_URL}");
    // Clone the repository
    let repo = Repository::clone(REPO_URL, Path::new(LOCAL_PATH))
        .with_context(|| format!("Failed to clone repository: {}", REPO_URL))?;
    info!("Finished cloning");

    // Prepare the remote callbacks with credentials
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent("git")
    });

    // Set up the fetch options with the callbacks
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    loop {
        info!("Sleeping for 60 seconds");
        // Wait for a minute before pulling updates
        sleep(Duration::from_secs(60)).await;

        // Fetch and merge updates
        let mut remote = match repo.find_remote("origin") {
            Ok(remote) => remote,
            Err(e) => {
                println!("Failed to find remote 'origin': {}", e);
                continue;
            }
        };
        if let Err(e) = remote.fetch(&["refs/heads/main"], Some(&mut fetch_options), None) {
            println!("Failed to fetch updates: {}", e);
            continue;
        }

        let fetch_head = match repo.find_reference("FETCH_HEAD") {
            Ok(reference) => reference,
            Err(e) => {
                println!("Failed to find FETCH_HEAD reference: {}", e);
                continue;
            }
        };
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)
            .with_context(|| "Failed to get annotated commit for FETCH_HEAD")?;
        let analysis = repo.merge_analysis(&[&fetch_commit])
            .with_context(|| "Failed to perform merge analysis")?;

        if analysis.0.is_up_to_date() {
            println!("Already up to date.");
        } else if analysis.0.is_fast_forward() {
            println!("Fast-forwarding...");
            let refname = format!("refs/heads/main");
            let mut reference = repo.find_reference(&refname)
                .with_context(|| format!("Failed to find reference: {}", refname))?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")
                .with_context(|| "Failed to set target for reference")?;
            repo.set_head(&refname)
                .with_context(|| format!("Failed to set HEAD to reference: {}", refname))?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                .with_context(|| "Failed to checkout HEAD")?;
        } else {
            println!("Merge required, not supported in this example.");
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    clone_and_pull_updates().await
}
