# git-wt

A simpler interface to git worktrees.

## Installation

### Using [mise](https://mise.jdx.dev) (Recommended)

```bash
mise use -g github:t128n/git-wt
```

### Manual Binary Releases

Download pre-built binaries from [GitHub Releases](https://github.com/t128n/git-wt/releases).

Available targets:
- `x86_64-unknown-linux-gnu` (Linux x64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `x86_64-pc-windows-gnu` (Windows x64)


## Configuration

git-wt reads config from `~/.config/git-wt/config.json`:

```json
{
    "base_worktree_path": "C:\\worktrees",
    "naming": "structured"
}
```

| Key | Default | Description |
|-----|---------|-------------|
| `base_worktree_path` | `~/worktrees` | Root directory for worktrees |
| `naming` | `structured` | `structured` = `<base>/<repo>/<name>`, `flat` = `<base>/<repo>-<name>` |

## Usage

### Add a worktree

```bash
git wt add feature-x                    # Create worktree for current branch
git wt add hotfix origin/main           # Create worktree from specific branch
```

### List worktrees

```bash
git wt list                             # List all worktrees with friendly names
```

### Remove a worktree

```bash
git wt remove feature-x                 # Remove a worktree by name
git wt remove feature-x --force         # Force removal even if dirty
```

### Prune stale worktrees

```bash
git wt prune                            # Clean up stale worktree data
```

### Navigate to worktree

```bash
git wt goto feature-x                   # Print worktree path (use with cd)
cd $(git wt goto feature-x)             # Change to worktree directory
```

### Shell Completions

Generate shell completions for your shell:

```bash
# Bash
git wt --completions bash > /etc/bash_completion.d/git-wt

# Zsh
git wt --completions zsh > ~/.zfunc/_git-wt

# Fish
git wt --completions fish > ~/.config/fish/completions/git-wt.fish

# PowerShell
git wt --completions powershell > git-wt.ps1
```

## License

MIT
