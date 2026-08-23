# Getting Started

This tutorial walks you through your first steps with git-wt.

## Prerequisites

- [git-wt installed](https://github.com/t128n/git-wt/releases)
- Git installed and available in your PATH
- A Git repository to work with

## Your First Worktree

Navigate to a Git repository and create a worktree:

```bash
cd /path/to/your/repo
git wt add feature-x
```

This creates a worktree at the configured base path:

```
~/worktrees/
└── your-repo/
    └── feature-x/
```

## Listing Worktrees

See all your worktrees:

```bash
git wt list
```

Output:

```
  feature-x  /home/you/worktrees/your-repo/feature-x  [feature-x]
  hotfix     /home/you/worktrees/your-repo/hotfix     [main]
```

## Creating Worktrees from Branches

Create a worktree from a specific branch:

```bash
git wt add hotfix origin/main
```

This checks out the `origin/main` branch in the new worktree.

## Removing Worktrees

Remove a worktree by name:

```bash
git wt remove feature-x
```

Or force removal if the worktree is dirty:

```bash
git wt remove feature-x --force
```

## Navigating to Worktrees

Get the path to a worktree (useful for scripting):

```bash
cd $(git wt goto feature-x)
```

## Understanding Path Resolution

By default, git-wt uses "structured" naming:

```
<base>/<repo>/<worktree-name>
```

For example, with base path `~/worktrees`:

- Repository: `your-repo`
- Worktree: `feature-x`
- Path: `~/worktrees/your-repo/feature-x`

You can also use "flat" naming:

```
<base>/<repo>-<worktree-name>
```

- Path: `~/worktrees/your-repo-feature-x`

## Configuration

Create a configuration file at `~/.config/git-wt/config.json`:

```json
{
    "base_worktree_path": "/home/you/worktrees",
    "naming": "structured"
}
```

See the [configuration guide](../how-to/configure-worktrees.md) for all options.

## Next Steps

- Learn more about [worktree configuration](../how-to/configure-worktrees.md)
- See the complete [CLI reference](../reference/cli.md)
- Understand the [architecture](../explanation/architecture.md)
