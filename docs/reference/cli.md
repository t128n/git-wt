# CLI Reference

Complete reference for the git-wt command-line interface.

## Synopsis

```bash
git-wt [COMMAND] [OPTIONS]
git-wt [--completions <SHELL>]
git-wt [--help] [--version]
```

## Commands

### add

Create a named worktree.

```bash
git-wt add <NAME> [BRANCH]
```

**Arguments:**

- `<NAME>` - Name for the worktree
- `[BRANCH]` - Branch to checkout (optional, defaults to current branch)

**Examples:**

```bash
git-wt add feature-x
git-wt add hotfix origin/main
git-wt add bugfix HEAD~3
```

**Behavior:**

1. Resolves the worktree path based on configuration
2. Creates parent directories if needed
3. Runs `git worktree add <path> [branch]`
4. Prints the created worktree path

**Output:**

```
Created worktree 'feature-x' at /home/you/worktrees/my-project/feature-x
```

### list

List all worktrees with friendly names.

```bash
git-wt list
```

**Output:**

```
  feature-x  /home/you/worktrees/my-project/feature-x  [feature-x]
  hotfix     /home/you/worktrees/my-project/hotfix     [main]
```

**Behavior:**

1. Runs `git worktree list`
2. Parses the output to extract worktree information
3. Filters out the main worktree (repository root)
4. Extracts friendly names from worktree paths
5. Displays formatted output

**Output Format:**

```
  <name>  <path>  [<branch>]
```

### remove

Remove a worktree by name or path.

```bash
git-wt remove <TARGET> [--force]
```

**Arguments:**

- `<TARGET>` - Worktree name or full path

**Options:**

- `--force` - Force removal even if the worktree has uncommitted changes

**Examples:**

```bash
git-wt remove feature-x
git-wt remove feature-x --force
git-wt remove /home/you/worktrees/my-project/feature-x
```

**Behavior:**

1. Resolves the target to a path (by name or direct path)
2. Runs `git worktree remove <path> [--force]`
3. Prints confirmation message

**Output:**

```
Removed worktree at /home/you/worktrees/my-project/feature-x
```

### prune

Clean up stale worktree data.

```bash
git-wt prune
```

**Behavior:**

1. Runs `git worktree prune`
2. Removes references to worktrees that no longer exist
3. Prints confirmation message

**Output:**

```
Pruned stale worktrees.
```

### goto

Print the path to a worktree (useful for `cd` and scripting).

```bash
git-wt goto <NAME>
```

**Arguments:**

- `<NAME>` - Worktree name

**Examples:**

```bash
# Print the path
git-wt goto feature-x

# Change to the worktree directory
cd $(git wt goto feature-x)

# Use in scripts
cd $(git wt goto feature-x) && cargo test
```

**Output:**

```
/home/you/worktrees/my-project/feature-x
```

**Behavior:**

1. Resolves the worktree path based on configuration
2. Verifies the path exists
3. Prints the path to stdout

### help

Print usage information.

```bash
git-wt help
```

**Output:**

```
git-wt - A simpler interface to git worktrees

USAGE:
    git wt add <name> [branch]    Create a named worktree
    git wt list                   List all worktrees
    git wt remove <name> [--force] Remove a worktree
    git wt prune                  Clean up stale worktree data
    git wt goto <name>            Print worktree path (for cd)
    git wt help                   Print this help message

OPTIONS:
    --force        Force removal even if dirty
    --completions <SHELL>  Generate shell completions (bash, zsh, fish, powershell, elvish)

CONFIG: ~/.config/git-wt/config.json
```

## Global Options

### --completions

Generate shell completions for the specified shell.

```bash
git-wt --completions <SHELL>
```

**Supported shells:**

- `bash`
- `zsh`
- `fish`
- `powershell`
- `elvish`

### --help

Print help information.

### --version

Print version information.

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | Error (worktree creation failed, path exists, not in git repo, etc.) |

## Environment

- **HOME** - Used to find configuration at `~/.config/git-wt/config.json`
- **PWD** - Used as the repository root (must be inside a Git repository)

## Examples

### Basic Worktree Operations

```bash
# Create a worktree
git wt add feature-x

# List all worktrees
git wt list

# Remove a worktree
git wt remove feature-x
```

### Advanced Operations

```bash
# Create worktree from specific branch
git wt add hotfix origin/main

# Force remove dirty worktree
git wt remove feature-x --force

# Get path for scripting
cd $(git wt goto feature-x)
```

### Git Integration

```bash
# Use as git subcommand
git wt add feature-x
git wt list
git wt remove feature-x

# Or use directly
git-wt add feature-x
git-wt list
git-wt remove feature-x
```

## Notes

- All commands must be run from within a Git repository
- Worktree paths are resolved based on the configuration file
- The main worktree (repository root) is excluded from list output
- Stale worktree references can be cleaned up with `prune`
