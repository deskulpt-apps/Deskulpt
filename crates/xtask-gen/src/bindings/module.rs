use anyhow::{Result, anyhow};
use deskulpt_common::bindings::Bindings;
use handlebars::Handlebars;
use heck::ToLowerCamelCase;
use regex::Regex;
use serde::Serialize;
use specta::TypeCollection;
use specta::datatype::{DataType, Function, FunctionResultVariant};
use specta_typescript::{Typescript, datatype, export_named_datatype, js_doc};

mod helpers {
    use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

    /// Handlebars helper to indent (multi-line) text by a given number of
    /// spaces.
    pub fn indent(
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
}

/// Similar to [`export_named_datatype`] but for [`DataType`].
fn export_datatype(ts: &Typescript, typ: &DataType, tcl: &TypeCollection) -> Result<String> {
    Ok(datatype(
        ts,
        &FunctionResultVariant::Value(typ.clone()),
        tcl,
    )?)
}

#[derive(Debug, Serialize)]
struct EventTemplate {
    key: String,
    name: String,
    ty: String,
}

impl EventTemplate {
    fn from(ts: &Typescript, tcl: &TypeCollection, name: &str, ty: &DataType) -> Result<Self> {
        Ok(Self {
            key: name
                .split("://")
                .nth(1)
                .ok_or_else(|| {
                    anyhow!("Deskulpt event name must contain '://' separator, but got: {name}")
                })?
                .to_lower_camel_case(),
            name: name.to_string(),
            ty: export_datatype(ts, ty, tcl)?,
        })
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
struct CommandTemplate {
    key: String,
    name: String,
    args: Vec<CommandArgTemplate>,
    ret_ty: String,
    doc: String,
}

impl CommandTemplate {
    fn from(ts: &Typescript, tcl: &TypeCollection, function: &Function) -> Result<Self> {
        Ok(Self {
            key: function.name().to_lower_camel_case(),
            name: function.name().to_string(),
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

#[derive(Debug, Serialize)]
pub struct Template {
    pub namespace: &'static str,
    types: Vec<String>,
    events: Vec<EventTemplate>,
    commands: Vec<CommandTemplate>,
}

impl Template {
    pub fn from(bindings: Bindings) -> Result<Self> {
        let ts = Typescript::new();

        Ok(Self {
            namespace: bindings.namespace,
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
                .map(|function| CommandTemplate::from(&ts, &bindings.types, function))
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn render(&self) -> Result<()> {
        let mut hb = Handlebars::new();
        hb.set_strict_mode(true);
        hb.register_escape_fn(handlebars::no_escape);
        hb.register_helper("indent", Box::new(helpers::indent));
        hb.register_template_string("module", include_str!("module.ts.hbs"))?;
        let output = hb.render("module", self)?;

        // TODO: Remove when specta > 2.0.0-rc.22
        let re =
            Regex::new(r"Partial\s*<\s*(\{\s*\[\s*key\s+in\s+string\s*\][^}]*\})\s*>").unwrap();
        let output = re.replace_all(&output, "$1").to_string();

        let path = deskulpt_workspace::package_dir("deskulpt-bindings")
            .join("src")
            .join(self.namespace)
            .with_extension("ts");
        std::fs::write(&path, output)?;
        println!("✅ Generated: {}", path.display());

        Ok(())
    }
}
