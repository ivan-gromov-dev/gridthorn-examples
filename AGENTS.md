# Gridthorn Examples Repository Instructions

- Store every example in its own top-level directory with its own `Cargo.toml`
  and focused source tree.
- Examples consume Gridthorn APIs as external game projects would. Do not use
  privileged access to subsystem internals.
- Keep every example runnable and testable through the repository Cargo
  workspace.
- Store tests with the domain they validate under `src/<domain>/test/`.
- Follow the engine repository comment policy: prefer declaration-level
  documentation and reserve comments inside blocks for genuinely tricky
  constraints.
- Example-only automation belongs to the example unless it is a reusable SDK
  capability.
