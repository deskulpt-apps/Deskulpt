use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use deskulpt_common::bindings::{Bindings, BindingsBuilder};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use heck::ToLowerCamelCase;
use regex::Regex;
use serde::Serialize;
use specta::datatype::{DataType, Function, FunctionResultVariant};
use specta::TypeCollection;
use specta_typescript::{datatype, export_named_datatype, js_doc, Typescript};

/// Similar to [`export_named_datatype`] but for [`DataType`].
fn export_datatype(ts: &Typescript, typ: &DataType, tcl: &TypeCollection) -> Result<String> {
    Ok(datatype(
        ts,
        &FunctionResultVariant::Value(typ.clone()),
        tcl,
    )?)
}

/// Template data for an event.
#[derive(Serialize)]
struct EventTemplate {
    key: String,
    name: String,
    ty: String,
}

impl EventTemplate {
    fn from(ts: &Typescript, tcl: &TypeCollection, name: &str, ty: &DataType) -> Result<Self> {
        Ok(Self {
            key: name.to_lower_camel_case(),
            name: name.to_string(),
            ty: export_datatype(ts, ty, tcl)?,
        })
    }
}

/// Template data for a command argument.
#[derive(Serialize)]
struct CommandArgTemplate {
    name: String,
    ty: String,
}

impl CommandArgTemplate {
    fn from(ts: &Typescript, tcl: &TypeCollection, name: &str, ty: &DataType) -> Result<Self> {
        Ok(Self {
            name: name.to_lower_camel_case(),
            ty: export_datatype(ts, ty, tcl)?,
        })
    }
}

/// Template data for a command.
#[derive(Serialize)]
struct CommandTemplate {
    key: String,
    name: String,
    plugin_name: String,
    args: Vec<CommandArgTemplate>,
    ret_ty: String,
    doc: String,
}

impl CommandTemplate {
    fn from(
        ts: &Typescript,
        tcl: &TypeCollection,
        plugin_name: &str,
        function: &Function,
    ) -> Result<Self> {
        Ok(Self {
            key: function.name().to_lower_camel_case(),
            name: function.name().to_string(),
            plugin_name: plugin_name.to_string(),
            args: function
                .args()
                .map(|(name, ty)| CommandArgTemplate::from(ts, tcl, name, ty))
                .collect::<Result<Vec<_>>>()?,
            ret_ty: match function.result() {
                Some(FunctionResultVariant::Value(t))
                | Some(FunctionResultVariant::Result(t, _)) => export_datatype(ts, t, tcl)?,
                None => "void".to_string(),
            },
            doc: {
                let mut builder = js_doc::Builder::default();
                if let Some(d) = &function.deprecated() {
                    builder.push_deprecated(d);
                }
                if !function.docs().is_empty() {
                    builder.extend(function.docs().split("\n"));
                }
                builder.build()
            },
        })
    }
}

/// Full template data.
#[derive(Serialize)]
struct Template {
    types: Vec<String>,
    events: Vec<EventTemplate>,
    commands: BTreeMap<String, Vec<CommandTemplate>>,
}

impl Template {
    fn from(ts: Typescript, bindings: &Bindings) -> Result<Self> {
        Ok(Self {
            types: bindings
                .types
                .into_iter()
                .map(|(_, ndt)| Ok(export_named_datatype(&ts, ndt, &bindings.types)?))
                .collect::<Result<Vec<_>>>()?,
            events: bindings
                .events
                .iter()
                .map(|(name, ty)| EventTemplate::from(&ts, &bindings.types, name, ty))
                .collect::<Result<Vec<_>>>()?,
            commands: bindings
                .commands
                .iter()
                .map(|(plugin_name, functions)| {
                    let fns = functions
                        .iter()
                        .map(|function| {
                            CommandTemplate::from(&ts, &bindings.types, plugin_name, function)
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let key = plugin_name
                        .strip_prefix("deskulpt-")
                        .map(ToLowerCamelCase::to_lower_camel_case)
                        .ok_or_else(|| {
                            anyhow!("Plugin name must start with 'deskulpt-'; got '{plugin_name}'")
                        })?;
                    Ok((key, fns))
                })
                .collect::<Result<BTreeMap<_, _>>>()?,
        })
    }
}

/// Handlebars helper to indent (multi-line) text by a given number of spaces.
fn handlebars_indent_helper(
    h: &Helper<'_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    let text = h
        .param(0)
        .and_then(|p| p.value().as_str())
        .unwrap_or_default();
    let spaces = h
        .param(1)
        .and_then(|p| p.value().as_u64())
        .unwrap_or_default();

    let pad = " ".repeat(spaces as usize);
    for (i, line) in text.lines().enumerate() {
        if i > 0 {
            out.write("\n")?;
        }
        out.write(&pad)?;
        out.write(line)?;
    }

    Ok(())
}

/// Entry point for the `cargo gen bindings` command.
pub fn run() -> Result<()> {
    let mut builder = BindingsBuilder::default();
    deskulpt_core::configure_bindings_builder(&mut builder);
    builder.typ::<deskulpt_common::window::DeskulptWindow>();
    let bindings = builder.build();

    let mut hb = Handlebars::new();
    hb.register_escape_fn(handlebars::no_escape);
    hb.register_helper("indent", Box::new(handlebars_indent_helper));
    hb.register_template_string("bindings", include_str!("bindings.ts.hbs"))?;

    let data = Template::from(Typescript::new(), &bindings)?;
    let output = hb.render("bindings", &data)?;

    // TODO: Remove when specta > 2.0.0-rc.22
    let re = Regex::new(r"Partial\s*<\s*(\{\s*\[\s*key\s+in\s+string\s*\][^}]*\})\s*>").unwrap();
    let output = re.replace_all(&output, "$1").to_string();

    let path = deskulpt_workspace::package_dir("deskulpt-bindings").join("src/index.ts");
    std::fs::write(&path, output)?;
    println!("✅ Generated: {}", path.display());

    Ok(())
}
