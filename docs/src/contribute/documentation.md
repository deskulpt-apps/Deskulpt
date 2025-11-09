# Documentation

## Documentation Website

This documentation website is built with [VitePress](https://vitepress.dev/). You can find the source code for the site under the `docs/` directory. To start a development server for the documentation website, run the following command (at project root):

```bash
pnpm docs:dev
```

Any changes to the documentation will automatically trigger reload. Most parts of the documentation are written purely in Markdown. Refer to the [VitePress](https://vitepress.dev/) documentation if you need to modify the structure or use advanced features.

If you want to build the documentation website into a static site instead of using the development server, you can use the following commands:

```bash
pnpm docs:build
pnpm docs:preview
```

Note that if you are only modifying the documentation contents without touching the setup, this step is not strictly necessary.

## Internal Backend Rustdoc

The Deskulpt backend is documented with [rustdoc](https://doc.rust-lang.org/rustdoc/) for developers' internal reference. Unlike the public docs hosted on [docs.rs](https://docs.rs/), the internal rustdoc includes private crate items and is meant specifically to be used by Deskulpt developers. It is built separately from the main documentation website and hosted under its `rustdoc/` subdirectory. To build the internal backend rustdoc locally, run the following command:

```bash
pnpm docs:rs
pnpm docs:rs --open # automatically open in your default browser
```

This command will print the path to the generated documentation, which you can open directly in our browser.
