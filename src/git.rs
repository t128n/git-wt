use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use crate::config::Config;

pub fn get_repo_root() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .with_context(|| "Failed to execute `git rev-parse --show-toplevel`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        anyhow::bail!("Not inside a git repository: {}", stderr);
    }

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if path.is_empty() {
        anyhow::bail!("Not inside a git repository");
    }

    Ok(path)
}

pub fn get_main_branch(config: &Config) -> Result<String> {
    if let Some(branch) = &config.default_branch {
        return Ok(branch.clone());
    }

    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .output()
        .with_context(|| "Failed to execute `git symbolic-ref refs/remotes/origin/HEAD`")?;

    if output.status.success() {
        let ref_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if let Some(branch) = ref_path.strip_prefix("refs/remotes/origin/") {
            return Ok(branch.to_string());
        }
    }

    Ok("main".to_string())
}

pub fn branch_exists(branch: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify"])
        .arg(branch)
        .output()
        .with_context(|| format!("Failed to check if branch '{}' exists", branch))?;

    Ok(output.status.success())
}

pub fn worktree_add(path: &Path, branch: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("worktree").arg("add").arg(path);

    if let Some(branch) = branch {
        cmd.arg(branch);
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute `git worktree add {}`", path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        anyhow::bail!(
            "`git worktree add {}` failed: {}",
            path.display(),
            stderr
        );
    }

    Ok(())
}

pub fn worktree_add_with_branch(
    path: &Path,
    new_branch: &str,
    base_branch: &str,
) -> Result<()> {
    let output = Command::new("git")
        .args(["worktree", "add", "-b"])
        .arg(new_branch)
        .arg(path)
        .arg(base_branch)
        .output()
        .with_context(|| {
            format!(
                "Failed to execute `git worktree add -b {} {} {}`",
                new_branch,
                path.display(),
                base_branch
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        anyhow::bail!(
            "`git worktree add -b {} {} {}` failed: {}",
            new_branch,
            path.display(),
            base_branch,
            stderr
        );
    }

    Ok(())
}

pub fn worktree_list() -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("worktree")
        .arg("list")
        .output()
        .with_context(|| "Failed to execute `git worktree list`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        anyhow::bail!("`git worktree list` failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

pub fn worktree_remove(path: &Path, force: bool) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("worktree").arg("remove").arg(path);

    if force {
        cmd.arg("--force");
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute `git worktree remove {}`", path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        anyhow::bail!(
            "`git worktree remove {}` failed: {}",
            path.display(),
            stderr
        );
    }

    Ok(())
}

pub fn worktree_prune() -> Result<()> {
    let output = Command::new("git")
        .arg("worktree")
        .arg("prune")
        .output()
        .with_context(|| "Failed to execute `git worktree prune`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        anyhow::bail!("`git worktree prune` failed: {}", stderr);
    }

    Ok(())
}
