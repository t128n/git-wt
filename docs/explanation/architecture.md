# Architecture

This document explains how git-wt works internally.

## Overview

git-wt is a Rust CLI tool that provides a simple interface to Git worktrees. It wraps Git's worktree commands with automatic path resolution and friendly naming.

## Design Principles

- **Simple configuration** - JSON-based config, easy to understand
- **Automatic path resolution** - Worktrees are organized by repository and name
- **Graceful degradation** - Works with sensible defaults
- **Git integration** - Uses Git's native worktree commands

## Components

### Main Entry Point (`main.rs`)

Parses CLI arguments and dispatches to the appropriate command handler.

```
CLI Parsing → Command Dispatch → Execution
```

Features:
- Clap-based CLI with derive macros
- Shell completion generation
- Subcommand routing

### Configuration (`config.rs`)

Handles loading and parsing the configuration file.

```
Config File → Parse JSON → Config Struct
```

Features:
- Loads from `~/.config/git-wt/config.json`
- Falls back to defaults if file doesn't exist
- Supports partial configuration (missing fields use defaults)
- Two naming modes: structured and flat

### Git Operations (`git.rs`)

Wraps Git CLI commands for worktree management.

```
git rev-parse --show-toplevel → Repository Root
git worktree add → Create Worktree
git worktree list → List Worktrees
git worktree remove → Remove Worktree
git worktree prune → Clean Stale Data
```

Features:
- Uses `-C` flag where possible to avoid directory changes
- Handles command output parsing
- Provides meaningful error messages

### Worktree Operations (`worktree.rs`)

Implements the core worktree management logic.

```
Resolve Path → Execute Git Command → Return Result
```

Features:
- Path resolution based on configuration
- Name extraction from worktree paths
- Cross-platform path handling
- Friendly name display in list output

## Data Flow

### Add Worktree

```
User Input (name, branch) → Resolve Worktree Path → Create Directory → Git Worktree Add → Output Path
```

### List Worktrees

```
Git Worktree List → Parse Output → Filter Main Worktree → Extract Names → Format Output
```

### Remove Worktree

```
User Input (name or path) → Resolve Path → Git Worktree Remove → Output Confirmation
```

### Goto Worktree

```
User Input (name) → Resolve Path → Verify Exists → Output Path
```

## Path Resolution

### Structured Naming

```
<base_worktree_path>/
└── <repo-name>/
    └── <worktree-name>/
```

Example:

```
/home/you/worktrees/
└── my-project/
    ├── feature-x/
    └── hotfix/
```

### Flat Naming

```
<base_worktree_path>/
└── <repo-name>-<worktree-name>/
```

Example:

```
/home/you/worktrees/
├── my-project-feature-x/
└── my-project-hotfix/
```

## Name Extraction

When listing worktrees, git-wt extracts friendly names from paths:

### Structured Mode

```
/home/you/worktrees/my-project/feature-x
                                 ↑
                                 Extract "feature-x"
```

Pattern: `<base>/<repo>/<name>` → `<name>`

### Flat Mode

```
/home/you/worktrees/my-project-feature-x
                           ↑
                           Extract "feature-x"
```

Pattern: `<base>/<repo>-<name>` → `<name>`

## Error Handling

git-wt uses `anyhow` for error handling:

- **Not in repository** - `git rev-parse --show-toplevel` fails
- **Path already exists** - Worktree directory exists
- **Worktree not found** - Name doesn't match any worktree
- **Git command failed** - Git operation returned non-zero exit code
- **Permission denied** - Cannot create/delete directories

Errors are printed to stderr and the process exits with code 1.

## Platform Support

- **Linux** - Full support
- **macOS** - Full support
- **Windows** - Full support (path separators are normalized)

### Path Handling

- Forward slashes are used internally
- Backslashes are normalized for comparison
- Case-insensitive comparison on Windows

## Performance

- **Add** - Limited by Git worktree creation speed
- **List** - Fast (single Git command + parsing)
- **Remove** - Limited by Git worktree removal speed
- **Prune** - Fast (single Git command)

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `anyhow` | Error handling |
| `serde` | JSON deserialization |
| `dirs` | Home directory detection |
| `clap_complete` | Shell completion generation |

## Testing

The project includes unit tests for:

- Configuration parsing
- Path resolution
- Path comparison

Run tests:

```bash
cargo test
```

## Comparison with git-tidy

| Feature | git-tidy | git-wt |
|---------|----------|--------|
| Purpose | Repository organization | Worktree management |
| Commands | clone, organize | add, list, remove, prune, goto |
| Configuration | Workspace routing | Base path and naming |
| URL Handling | Yes (normalization) | No (not needed) |
| Git Operations | clone, config | worktree commands |

## Future Considerations

Potential enhancements:

- Worktree templates
- Bulk operations
- Worktree metadata storage
- Integration with other Git tools
