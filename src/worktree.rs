use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::config::{Config, Naming};
use crate::git;

/// Resolve the target path for a worktree based on config and naming convention.
pub fn resolve_worktree_path(config: &Config, repo_root: &str, name: &str) -> PathBuf {
    let repo_name = Path::new(repo_root)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    match config.naming {
        Naming::Structured => config.base_worktree_path.join(&*repo_name).join(name),
        Naming::Flat => config
            .base_worktree_path
            .join(format!("{repo_name}-{name}")),
    }
}

/// Create a new worktree.
pub fn add_worktree(config: &Config, name: &str, branch: Option<&str>) -> Result<()> {
    let repo_root = git::get_repo_root()?;
    let path = resolve_worktree_path(config, &repo_root, name);

    if path.exists() {
        anyhow::bail!("Path already exists: {}", path.display());
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    git::worktree_add(&path, branch)?;

    println!("Created worktree '{}' at {}", name, path.display());

    Ok(())
}

/// List all worktrees with friendly names.
pub fn list_worktrees(config: &Config) -> Result<()> {
    let repo_root = git::get_repo_root()?;
    let lines = git::worktree_list()?;

    for line in &lines {
        if let Some(parsed) = parse_worktree_line(line, &repo_root, config) {
            println!(
                "  {}  {}  [{}]",
                parsed.name, parsed.path, parsed.branch
            );
        }
    }

    Ok(())
}

struct WorktreeInfo {
    name: String,
    path: String,
    branch: String,
}

fn parse_worktree_line(line: &str, repo_root: &str, config: &Config) -> Option<WorktreeInfo> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let wt_path = parts[0];
    let wt_branch = parts[2];

    // Skip the main worktree (the repo root itself)
    if paths_equal(wt_path, repo_root) {
        return None;
    }

    let repo_name = Path::new(repo_root)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    let wt_name = extract_worktree_name(wt_path, &repo_name, config);

    Some(WorktreeInfo {
        name: wt_name,
        path: wt_path.to_string(),
        branch: wt_branch.to_string(),
    })
}

fn extract_worktree_name(wt_path: &str, repo_name: &str, config: &Config) -> String {
    let base_path = config.base_worktree_path.to_string_lossy();

    match config.naming {
        Naming::Structured => {
            let pattern = format!("{}/{}", base_path, repo_name);
            if let Some(suffix) = wt_path.strip_prefix(&pattern) {
                let suffix = suffix.trim_start_matches('/').trim_start_matches('\\');
                if !suffix.is_empty() {
                    return suffix.to_string();
                }
            }
        }
        Naming::Flat => {
            let pattern = format!("{}/{}-", base_path, repo_name);
            if let Some(suffix) = wt_path.strip_prefix(&pattern)
                && !suffix.is_empty()
            {
                return suffix.to_string();
            }
        }
    }

    // Fallback: use directory name
    Path::new(wt_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// Remove a worktree by name or path.
pub fn remove_worktree(config: &Config, target: &str, force: bool) -> Result<()> {
    let path = if Path::new(target).exists() {
        PathBuf::from(target)
    } else {
        let repo_root = git::get_repo_root()?;
        resolve_worktree_path(config, &repo_root, target)
    };

    if !path.exists() {
        eprintln!("Warning: Path does not exist: {}", path.display());
    }

    git::worktree_remove(&path, force)?;

    println!("Removed worktree at {}", path.display());

    Ok(())
}

/// Prune stale worktree data.
pub fn prune_worktrees() -> Result<()> {
    git::worktree_prune()?;
    println!("Pruned stale worktrees.");
    Ok(())
}

/// Get the path for a worktree (for use with cd).
pub fn goto_worktree(config: &Config, name: &str) -> Result<PathBuf> {
    let repo_root = git::get_repo_root()?;
    let path = resolve_worktree_path(config, &repo_root, name);

    if !path.exists() {
        anyhow::bail!("Worktree path does not exist: {}", path.display());
    }

    Ok(path)
}

/// Compare two path strings, normalizing separators and case for cross-platform support.
fn paths_equal(a: &str, b: &str) -> bool {
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    let a_normalized = a_lower.replace('\\', "/");
    let b_normalized = b_lower.replace('\\', "/");
    a_normalized == b_normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, Naming};
    use std::path::PathBuf;

    fn test_config() -> Config {
        Config {
            base_worktree_path: PathBuf::from("/worktrees"),
            naming: Naming::Structured,
        }
    }

    #[test]
    fn resolve_structured() {
        let config = test_config();
        let path = resolve_worktree_path(&config, "/home/user/myrepo", "feature-x");
        assert_eq!(path, PathBuf::from("/worktrees/myrepo/feature-x"));
    }

    #[test]
    fn resolve_flat() {
        let mut config = test_config();
        config.naming = Naming::Flat;
        let path = resolve_worktree_path(&config, "/home/user/myrepo", "feature-x");
        assert_eq!(path, PathBuf::from("/worktrees/myrepo-feature-x"));
    }

    #[test]
    fn paths_equal_basic() {
        assert!(paths_equal("/foo/bar", "/foo/bar"));
        assert!(!paths_equal("/foo/bar", "/foo/baz"));
    }

    #[test]
    fn paths_equal_case_insensitive() {
        assert!(paths_equal("/Foo/Bar", "/foo/bar"));
    }

    #[test]
    fn paths_equal_separator_normalization() {
        assert!(paths_equal("/foo\\bar", "/foo/bar"));
    }
}
