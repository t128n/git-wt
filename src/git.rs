use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Get the repository root for the current directory.
pub fn get_repo_root() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .with_context(|| "Failed to execute `git rev-parse --show-toplevel`")?;

    if !output.status.success() {
        anyhow::bail!("Not inside a git repository");
    }

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if path.is_empty() {
        anyhow::bail!("Not inside a git repository");
    }

    Ok(path)
}

/// Add a worktree at the given path, optionally checking out a specific branch.
pub fn worktree_add(path: &Path, branch: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("worktree").arg("add").arg(path);

    if let Some(branch) = branch {
        cmd.arg(branch);
    }

    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute `git worktree add {}`", path.display()))?;

    if !status.success() {
        anyhow::bail!("`git worktree add {}` exited with status {status}", path.display());
    }

    Ok(())
}

/// List all worktrees. Returns raw output lines.
pub fn worktree_list() -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("worktree")
        .arg("list")
        .output()
        .with_context(|| "Failed to execute `git worktree list`")?;

    if !output.status.success() {
        anyhow::bail!("`git worktree list` exited with status {}", output.status);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

/// Remove a worktree at the given path.
pub fn worktree_remove(path: &Path, force: bool) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("worktree").arg("remove").arg(path);

    if force {
        cmd.arg("--force");
    }

    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute `git worktree remove {}`", path.display()))?;

    if !status.success() {
        anyhow::bail!(
            "`git worktree remove {}` exited with status {status}",
            path.display()
        );
    }

    Ok(())
}

/// Prune stale worktree data.
pub fn worktree_prune() -> Result<()> {
    let status = Command::new("git")
        .arg("worktree")
        .arg("prune")
        .status()
        .with_context(|| "Failed to execute `git worktree prune`")?;

    if !status.success() {
        anyhow::bail!("`git worktree prune` exited with status {status}");
    }

    Ok(())
}
