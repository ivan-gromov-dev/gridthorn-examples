# Examples AI workflow

Start with the requested scope, Git status, and applicable repository
instructions. Follow the engine's [AI workflow](../gridthorn-engine/docs/AI_WORKFLOW.md)
for context routing and verification. Run the affected example checks before
cleaning their output, and record the results.

## Build artifact cleanup

After completing required checks, remove generated build output and temporary
task files from this repository and the sibling engine repository. This includes
Cargo `target` directories (also nested or task-specific build directories),
Python `__pycache__` directories, generated test projects, disposable verification
logs, and rustfmt backup files. Retain diagnostics for unresolved failures until
they are resolved or handed off.

Resolve each absolute deletion target and verify that it stays inside the
intended repository before recursive deletion. Inspect ignored and untracked
paths; being ignored does not make a file disposable. Preserve sources, assets,
manifests, lockfiles, documentation, Git history, editor configuration, user saves
(including `tycoon_slice/harbor-save.toml`), and unrelated work. Do not delete
global Cargo caches or files outside the two repositories. Report removed paths
and reclaimed space when measured. Perform cleanup after verification so checks
do not recreate artifacts before handoff.
