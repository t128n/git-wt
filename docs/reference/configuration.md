# Configuration Reference

Complete reference for git-wt configuration.

## Configuration File

git-wt reads configuration from:

```
~/.config/git-wt/config.json
```

## Schema

```json
{
    "base_worktree_path": "<path>",
    "naming": "<string>"
}
```

## Options

### base_worktree_path

**Type:** `string` (path)
**Default:** `~/worktrees`

The root directory where worktrees are created.

```json
{
    "base_worktree_path": "/home/you/worktrees"
}
```

On Windows:

```json
{
    "base_worktree_path": "C:\\worktrees"
}
```

### naming

**Type:** `string`
**Default:** `"structured"`

How worktree directories are named.

```json
{
    "naming": "structured"
}
```

**Allowed values:**

- `"structured"` - `<base>/<repo>/<worktree-name>`
- `"flat"` - `<base>/<repo>-<worktree-name>`

## Examples

### Minimal Configuration

```json
{
    "base_worktree_path": "/home/you/worktrees"
}
```

### Structured Naming (Default)

```json
{
    "base_worktree_path": "/home/you/worktrees",
    "naming": "structured"
}
```

Result:

```
/home/you/worktrees/
├── project-a/
│   ├── feature-1/
│   └── feature-2/
└── project-b/
    └── feature-x/
```

### Flat Naming

```json
{
    "base_worktree_path": "/home/you/worktrees",
    "naming": "flat"
}
```

Result:

```
/home/you/worktrees/
├── project-a-feature-1/
├── project-a-feature-2/
└── project-b-feature-x/
```

### Complete Example

```json
{
    "base_worktree_path": "/home/you/worktrees",
    "naming": "structured"
}
```

## Path Resolution

### Structured Naming

```
<base_worktree_path>/<repo-name>/<worktree-name>
```

Example:
- Base: `/home/you/worktrees`
- Repository: `my-project` (at `/home/you/repos/my-project`)
- Worktree: `feature-x`
- Result: `/home/you/worktrees/my-project/feature-x`

### Flat Naming

```
<base_worktree_path>/<repo-name>-<worktree-name>
```

Example:
- Base: `/home/you/worktrees`
- Repository: `my-project`
- Worktree: `feature-x`
- Result: `/home/you/worktrees/my-project-feature-x`

## Naming Behavior

### Repository Name Extraction

The repository name is extracted from the repository root path:

- `/home/you/repos/my-project` → `my-project`
- `/home/you/work/src/awesome-lib` → `awesome-lib`
- `C:\Users\you\dev\project` → `project`

### Worktree Name

The worktree name is the `<NAME>` argument passed to `git-wt add`:

```bash
git-wt add feature-x  # Name: feature-x
git-wt add hotfix     # Name: hotfix
```

## Loading Behavior

1. If `~/.config/git-wt/config.json` does not exist, defaults are used
2. If the file exists but cannot be parsed, a warning is printed and defaults are used
3. Missing fields use their default values

## Verification

Validate your configuration by creating a worktree:

```bash
cd /path/to/your/repo
git-wt add test-worktree

# Check where it was created
git-wt goto test-worktree
```

## Troubleshooting

### Worktree created in wrong location

- Check that `base_worktree_path` is set correctly
- Verify the path exists and is writable
- Ensure you're running git-wt from a Git repository

### Configuration not loaded

- Verify the file is at `~/.config/git-wt/config.json`
- Check for JSON syntax errors:
  ```bash
  # Validate JSON
  python3 -c "import json; json.load(open('$HOME/.config/git-wt/config.json'))"
  ```
- Ensure the file is readable

### Permission denied

- Check that you have write permissions to `base_worktree_path`
- Create the directory manually if needed:
  ```bash
  mkdir -p ~/worktrees
  ```

### Path already exists

If you get "Path already exists" when adding a worktree:

1. Check if the worktree was already created:
   ```bash
   git wt list
   ```

2. Remove the existing worktree first:
   ```bash
   git wt remove <name>
   ```

3. Or use a different name:
   ```bash
   git wt add <new-name>
   ```

## Platform Notes

### Windows

Use forward slashes or escaped backslashes:

```json
{
    "base_worktree_path": "C:/worktrees"
}
```

Or:

```json
{
    "base_worktree_path": "C:\\worktrees"
}
```

### Linux/macOS

Standard paths work as expected:

```json
{
    "base_worktree_path": "/home/you/worktrees"
}
```

Or using `~`:

```json
{
    "base_worktree_path": "~/worktrees"
}
```

## Next Steps

- Learn about [worktree management](../how-to/manage-worktrees.md)
- See the [CLI reference](cli.md)
- Understand the [architecture](../explanation/architecture.md)
