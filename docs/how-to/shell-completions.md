# Shell Completions

This guide explains how to enable tab completion for git-wt in your shell.

## Overview

git-wt can generate shell completions for Bash, Zsh, Fish, PowerShell, and Elvish.

## Generate Completions

### Bash

```bash
git-wt --completions bash > /etc/bash_completion.d/git-wt
```

Or for user-local installation:

```bash
git-wt --completions bash > ~/.local/share/bash-completion/completions/git-wt
```

### Zsh

```bash
git-wt --completions zsh > ~/.zfunc/_git-wt
```

Add to your `~/.zshrc`:

```bash
fpath=(~/.zfunc $fpath)
autoload -Uz compinit
compinit
```

### Fish

```bash
git-wt --completions fish > ~/.config/fish/completions/git-wt.fish
```

### PowerShell

```powershell
git-wt --completions powershell > git-wt.ps1
```

Then import in your PowerShell profile:

```powershell
. .\git-wt.ps1
```

### Elvish

```bash
git-wt --completions elvish > ~/.elvish/lib/git-wt.elv
```

## Verify

After installing completions, restart your shell and test:

```bash
git-wt <TAB>
```

You should see available commands:

```
add      - Create a named worktree
goto     - Print worktree path (use with cd)
help     - Print usage information
list     - List all worktrees with friendly names
prune    - Clean up stale worktree data
remove   - Remove a worktree by name or path
```

## Git Integration

For git-wt to work as a git subcommand (`git wt`), ensure the binary is in your PATH and named `git-wt`.

### Linux/macOS

```bash
# Install to /usr/local/bin
sudo cp git-wt /usr/local/bin/

# Or install to ~/.local/bin
mkdir -p ~/.local/bin
cp git-wt ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

### Windows

```powershell
# Add to PATH
$env:PATH += ";C:\path\to\git-wt"

# Or copy to a directory already in PATH
Copy-Item git-wt.exe "C:\Program Files\Git\usr\bin\"
```

## Troubleshooting

### Completions not working

- Ensure the completions file is in the correct location for your shell
- Restart your shell after installing
- Check that git-wt is in your PATH

### Permission denied

Use `sudo` for system-wide installation, or install to a user-local directory.

### Git subcommand not found

If `git wt` doesn't work:

1. Check that `git-wt` is in your PATH
2. Verify the binary is named correctly (`git-wt` not `git-wt.exe` on Linux/macOS)
3. Try running `git-wt` directly to see if it works

### Windows-specific Issues

On Windows, ensure:
- The binary is named `git-wt.exe`
- It's in a directory that's in your PATH
- Your shell can find the binary

## Next Steps

- Learn about [worktree configuration](configure-worktrees.md)
- See the [CLI reference](../reference/cli.md)
