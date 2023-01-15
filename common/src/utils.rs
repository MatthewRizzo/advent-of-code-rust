use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::errors::AdventResult;

pub fn get_project_root() -> AdventResult<PathBuf> {
    // Assumes the project was cloned as a git repository
    let root_cmd_proc = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;
    let root_cmd_res = String::from_utf8_lossy(&root_cmd_proc.stdout)
        .to_string()
        .replace('\n', "");
    let path = Path::new(&root_cmd_res).to_path_buf();
    Ok(path)
}
