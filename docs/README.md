# git-wt

A simpler interface to git worktrees.

## Overview

git-wt helps you manage Git worktrees with a clean, simple interface:

- Create named worktrees with automatic path resolution
- List all worktrees with friendly names
- Remove worktrees by name or path
- Clean up stale worktree data
- Navigate to worktrees easily

## Quick Start

```bash
# Create a worktree
git wt add feature-x

# List worktrees
git wt list

# Remove a worktree
git wt remove feature-x
```

## Documentation

### [Tutorials](tutorials/getting-started.md)

Learn the basics of git-wt step by step.

- [Getting Started](tutorials/getting-started.md) - Your first steps with git-wt

### [How-to Guides](how-to/configure-worktrees.md)

Practical guides for common tasks.

- [Configure Worktrees](how-to/configure-worktrees.md) - Set up worktree paths and naming
- [Manage Worktrees](how-to/manage-worktrees.md) - Add, list, remove, and prune worktrees
- [Shell Completions](how-to/shell-completions.md) - Enable tab completion

### [Reference](reference/cli.md)

Complete documentation of all features.

- [CLI Reference](reference/cli.md) - Command-line interface
- [Configuration Reference](reference/configuration.md) - Configuration options

### [Explanation](explanation/architecture.md)

Understanding how git-wt works.

- [Architecture](explanation/architecture.md) - Design and implementation details

## Installation

Download pre-built binaries from [GitHub Releases](https://github.com/t128n/git-wt/releases).

## License

MIT
