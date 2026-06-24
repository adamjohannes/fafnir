use crate::cli::RemoteOperationArgs;
use crate::commands::git_utilities::utils::run_with_action;
use log::{debug, error};
use std::path::PathBuf;
use std::process::Command;

pub fn run(args: &RemoteOperationArgs) -> Result<(), anyhow::Error> {
    run_with_action(args, on_ok_push)
}

fn on_ok_push(path: PathBuf) -> anyhow::Result<(), anyhow::Error> {
    debug!("Pushing to '{}'... ", path.display());

    let output = Command::new("git")
        .arg("push")
        .current_dir(path.clone())
        .output()?;

    if !output.status.success() {
        error!("Failed to push to origin in '{}'", path.display());
    }

    Ok(())
}
