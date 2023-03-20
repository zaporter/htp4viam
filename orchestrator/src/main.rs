
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

const REPO_URL: &str = "https://github.com/zaporter/rdk";
const LOCAL_PATH: &str = "../sample2";

async fn clone_and_pull_updates() {
    // Clone the repository
    let repo = match Repository::clone(REPO_URL, Path::new(LOCAL_PATH)) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repository: {}", e),
    };

    // Prepare the remote callbacks with credentials
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent("git")
    });

    // Set up the fetch options with the callbacks
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    loop {
        // Wait for a minute before pulling updates
        sleep(Duration::from_secs(60)).await;

        // Fetch and merge updates
        if let Err(e) = repo.find_remote("origin") {
            println!("Failed to find remote 'origin': {}", e);
            continue;
        }
        let mut remote = repo.find_remote("origin").unwrap();
        if let Err(e) = remote.fetch(&["refs/heads/main"], Some(&mut fetch_options), None) {
            println!("Failed to fetch updates: {}", e);
            continue;
        }

        let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
        let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();

        if analysis.0.is_up_to_date() {
            println!("Already up to date.");
        } else if analysis.0.is_fast_forward() {
            println!("Fast-forwarding...");
            let refname = format!("refs/heads/main");
            let mut reference = repo.find_reference(&refname).unwrap();
            reference.set_target(fetch_commit.id(), "Fast-Forward").unwrap();
            repo.set_head(&refname).unwrap();
            repo.checkout_head(None).unwrap();
        } else {
            println!("Merge required, not supported in this example.");
        }
    }
}

#[tokio::main]
async fn main() {
    clone_and_pull_updates().await;
}
