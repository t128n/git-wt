# Configure Worktrees

This guide explains how to configure git-wt for your workflow.

## Overview

git-wt uses a JSON configuration file to define where worktrees are stored and how they're named.

## Configuration File

Create or edit `~/.config/git-wt/config.json`:

```json
{
    "base_worktree_path": "/home/you/worktrees",
    "naming": "structured"
}
```

## Configuration Options

### base_worktree_path

The root directory where worktrees are created.

**Default:** `~/worktrees`

```json
{
    "base_worktree_path": "/home/you/worktrees"
}
```

### naming

How worktree directories are named.

**Default:** `"structured"`

```json
{
    "naming": "structured"
}
```

Two naming modes are available:

#### Structured

```
<base>/<repo>/<worktree-name>
```

Example:
- Base: `~/worktrees`
- Repository: `my-project`
- Worktree: `feature-x`
- Path: `~/worktrees/my-project/feature-x`

#### Flat

```
<base>/<repo>-<worktree-name>
```

Example:
- Base: `~/worktrees`
- Repository: `my-project`
- Worktree: `feature-x`
- Path: `~/worktrees/my-project-feature-x`

## Naming Examples

### Structured Naming

```json
{
    "base_worktree_path": "/home/you/worktrees",
    "naming": "structured"
}
```

Worktrees are organized by repository:

```
/home/you/worktrees/
├── project-a/
│   ├── feature-1/
│   ├── feature-2/
│   └── hotfix/
├── project-b/
│   ├── feature-x/
│   └── bugfix/
```

### Flat Naming

```json
{
    "base_worktree_path": "/home/you/worktrees",
    "naming": "flat"
}
```

Worktrees are all in one directory:

```
/home/you/worktrees/
├── project-a-feature-1/
├── project-a-feature-2/
├── project-a-hotfix/
├── project-b-feature-x/
└── project-b-bugfix/
```

## When to Use Each Mode

### Use Structured When:

- You have many worktrees per repository
- You want clear visual organization
- You prefer hierarchical directory structures

### Use Flat When:

- You have few worktrees per repository
- You want all worktrees at the same level
- You prefer simpler paths

## Verification

Test your configuration by creating a worktree:

```bash
cd /path/to/your/repo
git wt add test-worktree
ls -la ~/worktrees/your-repo/test-worktree
```

The worktree should appear in the correct location based on your naming configuration.

## Troubleshooting

### Worktree created in wrong location

- Check that `base_worktree_path` is set correctly
- Verify the path exists and is writable
- Ensure you're running git-wt from a Git repository

### Permission denied

- Check that you have write permissions to `base_worktree_path`
- Create the directory manually if needed:
  ```bash
  mkdir -p ~/worktrees
  ```

### Configuration not loaded

- Verify the file is at `~/.config/git-wt/config.json`
- Check for JSON syntax errors
- Ensure the file is readable

## Next Steps

- Learn about [managing worktrees](manage-worktrees.md)
- See the [CLI reference](../reference/cli.md)
- Understand the [configuration reference](../reference/configuration.md)
