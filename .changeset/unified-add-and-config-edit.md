---
"@t128n/git-wt": minor
---

Add unified worktree creation with automatic branch detection and config edit command

- `git wt add <name>` now creates branch from main (auto-detected from origin/HEAD) if not exists, uses existing branch otherwise
- Added `default_branch` config option to override auto-detected main branch
- Added `git wt config edit` command to open config in $EDITOR (falls back to vim/nano/vi on Unix, notepad on Windows)
- Improved error messages to show actual git errors instead of just exit codes
