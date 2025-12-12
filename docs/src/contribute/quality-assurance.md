# Quality Assurance

Before submitting a pull request, it is recommended to run several quality assurance checks to ensure that your changes are in good shape. These quality assurance checks are also enforced in the CI. We understand that some rules are opinionated and can be counter-productive in certain cases, but please try to follow them as much as possible for consistency of the Deskulpt codebase.

## Local CI

You can run the full suite of CI checks locally to verify your changes before pushing. This runs the same checks as the GitHub Actions pipeline (hygiene, auto-generated files, linting, build, and tests).

```bash
pnpm ci:local
```

This command runs:

- **Hygiene checks**: Formatting, unused dependencies (Node and Rust), and auto-generated files verification
- **Linting checks**: JavaScript/TypeScript and Rust linting
- **Build checks**: Package builds, Rust type checking, and Rust binary build

If you want to skip the build and test steps for faster feedback (only checking hygiene, auto-generated files, and linting), you can use:

```bash
pnpm ci:local:fast
```

This command runs hygiene checks and linting, but skips the build steps.

## Formatting

```bash
pnpm format
```

This will run [rustfmt](https://rust-lang.github.io/rustfmt/) on Rust sources and [prettier](https://prettier.io/) on JavaScript, TypeScript, Markdown, and other assets. Automatic fixes will be applied. To run them separately, you can also use the following commands:

```bash
pnpm format:rs
pnpm format:js
```

If you only want to check for formatting errors without applying automatic fixes, append `:check` to each of the commands above.

## Linting

```bash
pnpm lint
```

This will run [clippy](https://doc.rust-lang.org/clippy/) on Rust sources and [oxlint](https://oxc.rs/docs/guide/usage/linter) on JavaScript and TypeScript sources. Automatic fixes will be applied where possible. To run them separately, you can also use the following commands:

```bash
pnpm lint:rs
pnpm lint:js
```

If you only want to check for linting errors without applying automatic fixes, append `:check` to each of the commands above.

## Pre-Commit Hooks

Deskulpt uses [husky](https://typicode.github.io/husky/) to register some of the quality assurance checks as pre-commit hooks, so that they are automatically run on the files you commit every time. These pre-commit hooks are automatically registered when you run `pnpm install`.

There are circumstances where you may not want these hooks to run for certain commits. In such cases, you can use the [`-n` flag](https://git-scm.com/docs/git-commit#Documentation/git-commit.txt-code-ncode) to bypass the hooks:

```bash
git commit -n -m "YOUR COMMIT MESSAGE"
```

However, note that the CI pipeline still enforces all checks on the pull requests, so bypassing hooks locally should only be the exception.
