# Manage Worktrees

This guide explains how to add, list, remove, and prune worktrees.

## Overview

git-wt provides simple commands to manage Git worktrees throughout their lifecycle.

## Adding Worktrees

### Create a Worktree for Current Branch

```bash
git wt add feature-x
```

This creates a worktree and checks out the current branch.

### Create a Worktree from a Specific Branch

```bash
git wt add hotfix origin/main
```

This creates a worktree and checks out the `origin/main` branch.

### Create a Worktree from a New Branch

```bash
git wt add new-feature
git checkout -b new-feature
```

Or combine the steps:

```bash
git wt add new-feature && cd $(git wt goto new-feature) && git checkout -b new-feature
```

## Listing Worktrees

### List All Worktrees

```bash
git wt list
```

Output:

```
  feature-x  /home/you/worktrees/my-project/feature-x  [feature-x]
  hotfix     /home/you/worktrees/my-project/hotfix     [main]
```

### Understanding the Output

Each line shows:
- **Name** - The worktree name (derived from path or configured name)
- **Path** - Full path to the worktree directory
- **Branch** - Current branch checked out in the worktree

## Removing Worktrees

### Remove by Name

```bash
git wt remove feature-x
```

### Remove by Path

```bash
git wt remove /home/you/worktrees/my-project/feature-x
```

### Force Removal

If the worktree has uncommitted changes:

```bash
git wt remove feature-x --force
```

**Warning:** This will discard uncommitted changes in the worktree.

### What Happens on Removal

1. Git removes the worktree reference
2. The directory is deleted
3. The worktree is no longer tracked

## Pruning Worktrees

Over time, worktree references can become stale. Prune them:

```bash
git wt prune
```

This removes:
- Worktree directories that no longer exist
- Stale worktree configuration data

### When to Prune

- After manually deleting a worktree directory
- After moving worktree directories
- Periodically as maintenance

## Navigating to Worktrees

### Get Worktree Path

```bash
git wt goto feature-x
```

Output:

```
/home/you/worktrees/my-project/feature-x
```

### Change to Worktree Directory

```bash
cd $(git wt goto feature-x)
```

### Use in Scripts

```bash
# Build in a worktree
cd $(git wt goto feature-x) && make

# Run tests
cd $(git wt goto feature-x) && cargo test
```

## Common Workflows

### Feature Development

```bash
# Start a new feature
git wt add my-feature
cd $(git wt goto my-feature)

# Work on the feature
git checkout -b my-feature
# ... make changes ...
git commit -m "Add new feature"

# Return to main repository
cd -

# Clean up when done
git wt remove my-feature
```

### Hotfix Workflow

```bash
# Create hotfix from production
git wt add hotfix origin/main
cd $(git wt goto hotfix)

# Apply fix
git checkout -b hotfix/urgent-fix
# ... make changes ...
git commit -m "Fix critical bug"

# Return to main repository
cd -
```

### Parallel Development

```bash
# Create multiple worktrees for parallel work
git wt add feature-a
git wt add feature-b
git wt add bugfix

# Work on each independently
cd $(git wt goto feature-a) && # ... work ...
cd $(git wt goto feature-b) && # ... work ...
cd $(git wt goto bugfix) && # ... work ...

# List all active worktrees
git wt list
```

## Error Handling

### "Path already exists"

The worktree directory already exists. Either:
- Use a different name
- Remove the existing directory manually
- Check if the worktree was already created

### "Not inside a git repository"

You must be in a Git repository to use git-wt. Navigate to a repository first:

```bash
cd /path/to/your/repo
git wt add feature-x
```

### "Worktree path does not exist"

The worktree may have been manually deleted. Use `git wt prune` to clean up stale references.

## Next Steps

- Learn about [worktree configuration](configure-worktrees.md)
- See the [CLI reference](../reference/cli.md)
- Understand the [architecture](../explanation/architecture.md)
