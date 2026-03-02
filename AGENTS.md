# Deskulpt AGENTS

## Project charter

- **Mission:** Deliver a reliable, cross-platform desktop customization environment where React widgets render directly on Windows, macOS, and Linux through a Rust/Tauri runtime.
- **Scope:** Ship the runtime (`deskulpt-core`, `deskulpt-widgets`, plugin crates), Specta-powered bindings, and the Tauri front-end apps in `packages/`. Support widget authors with tooling, docs, and curated examples.
- **Stakeholders:** Deskulpt Apps maintainers (CS1060 DESKLPT), open-source contributors, and end users customizing desktops. Coordinate in GitHub Issues/Discussions, Linear, and the shared Google Drive folder referenced in `README.md`.
- **Non-goals:** OS-specific hacks that bypass abstractions, unchecked experimental widgets, or changes outside the documented review/QA funnel.

## Repository & architecture orientation

### Rust backend (`crates/`)

- `deskulpt/`: Thin Tauri binary that wires subsystems; keep heavy logic in libraries.
- `deskulpt-core/`: Runtime services (state machines like `states/logging.rs`, shortcuts, tracing, persistence).
- `deskulpt-widgets/`: Widget renderer/executor (`src/render.rs`, `commands.rs`, `lib.rs`) bridging React commands to OS primitives.
- `deskulpt-plugin`, `deskulpt-plugin-fs`, `deskulpt-plugin-sys`: Plugin API surface plus filesystem/system adapters.
- `deskulpt-macros`, `deskulpt-plugin-macros`: Proc macros for Specta/serde schemas and plugin ergonomics.
- `deskulpt-settings`, `deskulpt-common`: Shared configuration and domain models.
- `deskulpt-build`, `deskulpt-workspace`, `xtask-gen`: Build tooling, workspace automation, and code/doc generators.

### TypeScript/React frontend (`packages/`, `docs/`)

- `deskulpt-canvas`: Transparent desktop overlay rendering widgets.
- `deskulpt-manager`: Manager UI for widget lifecycle, preferences, and settings.
- `deskulpt-bindings`: Specta/TypeScript bindings for backend commands/events.
- `deskulpt`, `deskulpt-utils`, `deskulpt-react`, `deskulpt-ui`, `deskulpt-canvas`: Core UI kit and runtime helpers consumed across apps.
- `packages/widgets`: Curated widget gallery for demos/tests.
- `packages/apis`, `observability`, etc.: API schemas, telemetry hooks, and integration helpers.
- `docs/`: VitePress documentation + `whatsnew/` release notes. Keep docs aligned with shipped behavior.

## Generated artifacts & tooling contracts

- **Never edit** `crates/deskulpt-core/gen/`, root `gen/`, or `packages/deskulpt-bindings/src/` manually.
- Run `pnpm build:packages` after touching shared UI/API packages; it regenerates the `gen/` artifacts consumed by Rust crates.
- Run `cargo gen bindings` whenever backend types with `specta::Type` or `#[specta::specta]` commands change; commit regenerated bindings with the code change.
- Use `cliff.toml` + `git cliff` for changelog entries; keep release notes consistent with this template.

## Collaboration protocol

- Branch naming: `{type}/{scope}-{issue}` (`feat/widgets-grid-123`, `chore/logging-ci`).
- Every change flows through a PR referencing a Linear/GitHub issue. Include rationale, screenshots for UI, and the commands you ran.
- Require at least one domain reviewer before merging; never push directly to `main`.
- Architectural or UX shifts need an ADR or GitHub Discussion summary linked in the PR.
- Mirror course-specific updates (CS1060) in both Linear and the shared Drive folder.

## Development workflow

- Setup: `pnpm install`, `rustup toolchain install stable nightly`, `rustup component add rustfmt clippy --toolchain nightly`, `pnpm husky install`.
- Daily loop:
  - `pnpm tauri dev` to run the shell with live React widgets.
  - `cargo watch -x 'check --workspace'` or targeted `cargo test -p deskulpt-core -- logging::state` while iterating on Rust.
  - `pnpm --filter deskulpt-manager dev` (or other package-specific dev servers) for front-end-only work.
- Build commands: `pnpm build`, `cargo check`, `pnpm tauri build --debug --no-bundle`. Re-run `cargo gen bindings` before any front-end build when Specta-exposed types changed.

## Testing & quality instructions

- **CI plan:** `.github/workflows/ci.yaml` runs on every push/PR. It executes `pnpm format:check`, `pnpm lint:check`, Rust doc builds, and `cargo test --workspace`. Keep CI green; coordinate with maintainers if shared failures block you.
- **Local tests:**
  - `pnpm test` orchestrates JS + Rust suites (JS currently stubbed but must stay runnable).
  - `pnpm test:rs` / `cargo test --workspace` are mandatory before PRs touching Rust crates.
  - `pnpm --filter <pkg> test` for package-level JS suites as they land.
  - Use targeted Rust tests (`cargo test -p deskulpt-widgets render::`) for tight iteration loops.
- **Linters & static analysis:**
  - `pnpm lint` (`oxlint` + `cargo clippy --fix`). Use `pnpm lint:check` or `cargo clippy -- -D warnings` when you cannot modify files.
  - Formatting: `pnpm format` (Prettier + `cargo +nightly fmt`). Run before pushing to avoid CI churn.
  - Type checks: `pnpm tsc -b` or `pnpm build` to ensure bindings remain in sync.
- **Docs/build validation:** `pnpm docs:build` when editing docs; `pnpm docs:dev` for preview. Execute `pnpm tauri build` before tagging releases or merging OS-specific work.
- **Do NOT change existing tests unless:** explicitly requested by the user/maintainer or the asserted contract has legitimately changed. Document the rationale in the PR and secure reviewer approval before merging.
- **MANDATORY:** NEVER change existing tests unless explicitly requested by the user. Do not “fix” tests to get green builds; failing tests must be resolved by fixing the underlying code or by receiving explicit approval to update the assertions. If you believe a test must change, ask for permission first and clearly document why the change is required.

## Release & deployment playbook

- Create release branches (`release/vX.Y.Z`). Bump versions across `Cargo.toml`, `package.json`, and generated bindings.
- Generate changelog entries with `git cliff --tag vX.Y.Z` using `cliff.toml`.
- Build and smoke-test artifacts on Windows, macOS, and Linux via `pnpm tauri build`; attach signed binaries to GitHub Releases.
- Update `docs/whatsnew` and any homepage content before tagging the release.

## Common pitfalls & best practices

- Inspect surrounding modules before refactoring; keep logging/tracing conventions consistent.
- Centralize `cfg`/feature flags for OS-specific code and document them.
- Regenerate bindings/assets immediately after interface changes—stale files are a common CI failure.
- Break large changes into trackable subtasks to avoid scope creep.
- Use `RUST_LOG=deskulpt=debug` and tracing subscribers for diagnosing runtime issues.

## Quick reference

- Bootstrap: `pnpm install && cargo check --workspace`.
- Formatting/linting: `pnpm format`, `pnpm lint`.
- Testing: `pnpm test`, `pnpm test:rs`, `cargo test -p deskulpt-core -- logging::state`.
- Docs: `pnpm docs:dev`, `pnpm docs:build`.
- Release prep: `git cliff --output CHANGELOG.md`, `pnpm tauri build`.

Keep this document current; update it whenever tooling, workflows, or ownership changes so contributors always have an authoritative guide.
