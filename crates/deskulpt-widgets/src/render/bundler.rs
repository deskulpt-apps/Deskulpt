//! Rolldown-based bundler for Deskulpt widgets.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Result, anyhow, bail};
use either::Either;
use rolldown::{
    BundlerOptions, BundlerTransformOptions, JsxOptions, OutputFormat, Platform, RawMinifyOptions,
};
use rolldown_common::Output;

use crate::render::alias_plugin::AliasPlugin;

/// A default Deskulpt dependency provided by the Deskulpt runtime.
struct DefaultDependency {
    /// The module name of the dependency.
    name: &'static str,
    /// The URL to load the dependency from at runtime.
    url: &'static str,
}

/// The Deskulpt widget bundler.
///
/// Under the hood it wraps a [`rolldown::Bundler`] but is pre-configured to
/// suit Deskulpt widgets' needs.
pub struct Bundler(rolldown::Bundler);

impl Bundler {
    /// The default dependencies provided by the Deskulpt runtime.
    ///
    /// These dependencies are externalized during bundling and aliased to URLs
    /// resolvable at runtime. Note that the URLs may contain the following
    /// placeholders that must be replaced in the frontend before importing:
    ///
    /// - `__DESKULPT_BASE_URL__`: The base URL where the Deskulpt canvas is
    ///   served from.
    /// - `__DESKULPT_APIS_BLOB_URL__`: The URL of the blob containing the
    ///   generated Deskulpt APIs.
    const DEFAULT_DEPENDENCIES: &[DefaultDependency] = &[
        DefaultDependency {
            name: "@deskulpt-test/emotion/jsx-runtime",
            url: "__DESKULPT_BASE_URL__/gen/jsx-runtime.js",
        },
        DefaultDependency {
            name: "@deskulpt-test/raw-apis",
            url: "__DESKULPT_BASE_URL__/gen/raw-apis.js",
        },
        DefaultDependency {
            name: "@deskulpt-test/react",
            url: "__DESKULPT_BASE_URL__/gen/react.js",
        },
        DefaultDependency {
            name: "@deskulpt-test/ui",
            url: "__DESKULPT_BASE_URL__/gen/ui.js",
        },
        DefaultDependency {
            name: "@deskulpt-test/apis",
            url: "__DESKULPT_APIS_BLOB_URL__",
        },
    ];

    /// Create a new [`Bundler`] instance.
    ///
    /// This takes the root directory of the widget and the entry file path
    /// relative to the root directory. The returned bundler is configured with
    /// the following features:
    ///
    /// - Minified ESM output for browser target.
    /// - TypeScript supported by rolldown out of the box.
    /// - JSX transform with `@deskulpt-test/emotion` automatic runtime. This
    ///   resolves to `@deskulpt-test/emotion/jsx-runtime` which is listed in
    ///   [`Self::DEFAULT_DEPENDENCIES`].
    /// - Alias [`Self::DEFAULT_DEPENDENCIES`] to their resolvable URLs with
    ///   [`AliasPlugin`], so widget code can import them by module names.
    /// - Externalize the aliased URLs of [`Self::DEFAULT_DEPENDENCIES`], so the
    ///   bundler will not try to resolve them at bundle time (which will fail).
    pub fn new(root: PathBuf, entry: String) -> Result<Self> {
        let bundler_options = BundlerOptions {
            input: Some(vec![entry.into()]),
            cwd: Some(root),
            format: Some(OutputFormat::Esm),
            platform: Some(Platform::Browser),
            minify: Some(RawMinifyOptions::Bool(true)),
            transform: Some(BundlerTransformOptions {
                jsx: Some(Either::Right(JsxOptions {
                    runtime: Some("automatic".to_string()),
                    import_source: Some("@deskulpt-test/emotion".to_string()),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            external: Some(
                Self::DEFAULT_DEPENDENCIES
                    .iter()
                    .map(|dep| dep.url.to_string())
                    .collect::<Vec<_>>()
                    .into(),
            ),
            ..Default::default()
        };

        let alias_plugin = AliasPlugin(
            Self::DEFAULT_DEPENDENCIES
                .iter()
                .map(|dep| (dep.name.to_string(), dep.url.to_string()))
                .collect(),
        );

        let inner = rolldown::Bundler::with_plugins(bundler_options, vec![Arc::new(alias_plugin)])?;
        Ok(Self(inner))
    }

    /// Bundle the widget into a single output code string.
    pub async fn bundle(&mut self) -> Result<String> {
        let result = self.0.generate().await.map_err(|e| {
            anyhow!(
                e.into_vec()
                    .iter()
                    .map(|diagnostic| diagnostic.to_diagnostic().to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        })?;

        // We have supplied a single entry file, so we expect a single output
        // bundle; this can be broken if widget code contains e.g. dynamic
        // imports, which we do not allow
        if result.assets.len() != 1 {
            bail!(
                "Expected 1 bundled output, found {}; ensure that widget code does not contain \
                 e.g. dynamic imports that may result in extra chunks",
                result.assets.len()
            );
        }

        let output = &result.assets[0];
        let code = match output {
            Output::Asset(asset) => asset.source.clone().try_into_string()?,
            Output::Chunk(chunk) => chunk.code.clone(),
        };
        Ok(code)
    }
}
