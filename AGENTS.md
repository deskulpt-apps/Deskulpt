# Deskulpt AGENTS

## Project charter

- **Mission:** Deliver a first-class, cross-platform desktop customization environment where React widgets run natively on Windows, macOS, and Linux via a Rust/Tauri runtime.
- **What we ship:**
  - `deskulpt-core`: runtime services (logging, system shortcuts, process lifecycle).
  - `deskulpt-widgets`: widget renderer/executor bridging React and system APIs.
  - `deskulpt-*` crates: macros, plugin surfaces, workspace/build helpers, system adapters.
  - JS/TS packages inside `packages/` powering the Tauri frontend, widget gallery, docs tooling, and integration helpers.
- **Source of truth:** main branch on GitHub (`deskulpt-apps/Deskulpt`). All work funnels through PRs referencing Linear tickets when applicable.
- **Non-goals:** shipping OS-specific hacks that cannot be abstracted, experimental widgets without automated coverage, or changes that bypass the documented review + QA gates.

## Architecture orientation

- **Rust workspace (`crates/`):**
  - `deskulpt-core`: orchestrates state machines (see `states/logging.rs`, `shortcuts.rs`), handles tracing, interop with Tauri shell.
  - `deskulpt-widgets`: render graph + command dispatch (see `src/render.rs`, `commands.rs`, `lib.rs`).
  - `deskulpt-build` / `deskulpt-workspace`: build tooling, template/xtask helpers.
  - `deskulpt-plugin`, `deskulpt-plugin-fs`, `deskulpt-plugin-sys`: plugin API and host integrations (file system, OS services).
  - `deskulpt-macros` / `deskulpt-plugin-macros`: proc-macros, schema generation.
  - `deskulpt-settings`, `deskulpt-common`: configuration management and shared models.
  - `xtask-gen`: generators that scaffold code/docs; keep them current with any schema updates.
- **JavaScript/TypeScript workspaces (`packages/`, `docs/`):**
  - `packages/deskulpt`, `deskulpt-manager`, `ui`, `react`: front-end shell and component kit rendered inside the Tauri window.
  - `packages/widgets`: curated widget set for gallery/testing.
  - `packages/apis`, `deskulpt-bindings`, `deskulpt-utils`: bridge layer for invoking Rust commands via Specta bindings.
  - `docs/`: VitePress-based documentation + `whatsnew/` release notes.
- **Tooling stack:**
  - PNPM is mandatory (`package.json#scripts.preinstall` enforces `only-allow pnpm`).
  - Rust 2024 edition with nightly formatter (`cargo +nightly fmt`) and stable toolchain for builds/tests.
  - Tauri CLI (`pnpm tauri`) for bundling native shells; Oxlint for JS linting; Specta/serde for schema bridging.

## Workstreams & ownership

- **Runtime stability:** owned by Rust domain leads; prioritize tracing, error surfaces, and cross-platform parity. Coordinate changes touching `deskulpt-core` or `deskulpt-widgets`.
- **Widget & UI experience:** front-end squad maintains packages under `packages/`. Align on component APIs with runtime folks before changing bindings.
- **Docs & developer experience:** documentation squad owns `docs/`, `README.md`, and developer tooling in `packages/observability`, `packages/react`.
- **Release management:** maintainers handle changelog generation via `cliff.toml` + `git cliff`, tagging GitHub releases only after artifacts build on all OSes.

## Collaboration protocol

- Branch naming: `{type}/{scope}-{issue}` (`feat/widgets-grid-123`, `chore/logging-ci`).
- Keep PRs scoped; include Linear or GitHub issue links plus testing evidence (command output, screenshots).
- Architectural or UX shifts require an ADR/GitHub Discussion summary before merging.
- Never push directly to `main`; require at least one reviewer with domain knowledge.
- Capture course-specific updates (CS1060) both in Linear and the shared Google Drive folder referenced in `README.md`.

## Testing & quality gates

- **CI (`.github/workflows/ci.yaml`):** runs on pushes + PRs. Steps: formatting (`pnpm format:check`), lint (`pnpm lint:check`), Rust doc build, and `cargo test --workspace`. Keep CI green; coordinate if a failing job blocks unrelated work.
- **Local test pyramid:**
  - `pnpm test` – orchestrates JS + Rust suites (JS currently stubbed, but keep command working).
  - `pnpm test:rs` / `cargo test --workspace` – required before PRs touching Rust crates.
  - Targeted tests: use `cargo test -p deskulpt-core -- logging::state` etc. when iterating quickly.
  - JS/TS widgets: when UI suites exist, run `pnpm --filter <pkg> test` inside the relevant package.
- **Linters/static analysis:**
  - `pnpm lint` (or `pnpm lint:check`) – Oxlint for JS/TS, `cargo clippy` for Rust (`--fix` only when you intend to commit changes).
  - Formatting: `pnpm format` (`prettier`, `cargo +nightly fmt`); run before commits to avoid churn.
  - Type safety: `pnpm tsc -b` across packages; CI enforces via `pnpm build`.
- **Docs/build verifications:**
  - `pnpm docs:build` for doc contributions.
  - `pnpm build` (Vite) and `pnpm tauri build` before tagging releases or merging platform-specific changes.
- **When to update tests:** new features, bug fixes, or behavior changes affecting APIs/CLI/UI flows must add or modify coverage. Write regression tests before fixes when possible.
- **Do NOT modify existing tests unless:** explicitly requested by the maintainer/user or the asserted contract is obsolete. Any test change must cite the spec/update in your PR and include reviewer approval.
- **Additional guidance:**
  - Keep `tracing` logs enabled when debugging runtime issues (`RUST_LOG=deskulpt=debug cargo run`).
  - Use feature flags or cfg guards for OS-specific logic; avoid `#[cfg(target_os = "...")]` scattering without central documentation.

## Local environment setup

1. Install PNPM and run `pnpm install` at repo root (enforces pnpm via `preinstall`).
2. `rustup toolchain install stable nightly` and `rustup component add rustfmt clippy --toolchain nightly` to satisfy formatting/lint steps.
3. `pnpm husky install` if hooks are missing after cloning.
4. Build once with `pnpm tauri dev` (or `pnpm tauri build`) to verify front-end + Rust wiring.
5. Keep artifacts (`target/`, `dist/`, `node_modules/`) out of commits; use `.gitignore` defaults. Run `pnpm clean`/`cargo clean` if builds misbehave.

## Release & deployment playbook

- Cut release branches (`release/vX.Y.Z`) when the changelog (`git cliff --tag vX.Y.Z`) and binaries are ready.
- Ensure docs (`docs/whatsnew`, site) mention user-facing changes before tagging.
- Build signed artifacts per OS using Tauri CLI and attach them to GitHub Releases; verify auto-updates where applicable.
- After release, bump versions in `Cargo.toml`, `package.json`, and any Specta-generated bindings; keep workspace versions in sync.

## Decision & escalation routes

- **Technical decisions:** default to async discussions in GitHub Discussions or Linear. Summaries/decisions should be linked from PR descriptions.
- **Blocking issues:** page runtime maintainers via course communication channels or GitHub (@deskulpt-maintainers) when CI failures or regressions block progress.
- **Security/privacy:** report vulnerabilities privately to maintainers before opening issues. Avoid committing secrets; leverage `deskulpt-settings` for config injection.

## Appendix – quick command reference

- Bootstrap: `pnpm install`, `cargo check --workspace`.
- Development loops: `pnpm tauri dev` (UI shell) + `cargo watch -x 'test -p deskulpt-core logging::state'`.
- Tooling helpers: `pnpm docs:dev`, `pnpm build:packages`, `pnpm docs:rs`.
- Housekeeping: `pnpm format`, `pnpm lint`, `git cliff --output CHANGELOG.md`.

Keep this file updated whenever tooling, workflows, or project scope changes so contributors always have an authoritative guide.
