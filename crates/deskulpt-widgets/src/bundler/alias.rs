//! Deskulpt alias plugin for rolldown.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use rolldown::plugin::{
    HookResolveIdArgs, HookResolveIdOutput, HookResolveIdReturn, HookUsage, Plugin, PluginContext,
    PluginContextResolveOptions,
};

/// Deskulpt alias plugin.
///
/// This is a simplified version of the rolldown built-in alias plugin since we
/// only need a subset of its functionalities.
///
/// Based on the given alias mapping, this plugin will replace the original
/// imports with the aliased imports. Note that the aliased imports need to be
/// either resolvable or externalized to avoid bundling errors.
#[derive(Debug)]
pub struct AliasPlugin(
    /// The alias mapping from original imports to aliased imports.
    pub HashMap<String, String>,
);

impl Plugin for AliasPlugin {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("deskulpt:alias")
    }

    async fn resolve_id(
        &self,
        ctx: &PluginContext,
        args: &HookResolveIdArgs<'_>,
    ) -> HookResolveIdReturn {
        let importee = args.specifier;
        let update_id = match self.0.get(importee) {
            Some(alias) => alias,
            None => return Ok(None),
        };

        let resolved_id = ctx
            .resolve(
                update_id,
                args.importer,
                Some(PluginContextResolveOptions {
                    import_kind: args.kind,
                    is_entry: args.is_entry,
                    skip_self: true,
                    custom: Arc::clone(&args.custom),
                }),
            )
            .await??;

        Ok(Some(HookResolveIdOutput {
            id: resolved_id.id,
            ..Default::default()
        }))
    }

    fn register_hook_usage(&self) -> HookUsage {
        HookUsage::ResolveId
    }
}
