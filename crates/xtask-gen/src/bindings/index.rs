use anyhow::Result;
use handlebars::Handlebars;
use heck::ToLowerCamelCase;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct NamespaceTemplate {
    key: String,
    namespace: &'static str,
}

#[derive(Default, Debug, Serialize)]
pub struct Template {
    namespaces: Vec<NamespaceTemplate>,
}

impl Template {
    pub fn add_namespace(&mut self, namespace: &'static str) {
        self.namespaces.push(NamespaceTemplate {
            key: namespace.to_lower_camel_case(),
            namespace,
        });
    }

    pub fn render(&self) -> Result<()> {
        let mut hb = Handlebars::new();
        hb.set_strict_mode(true);
        hb.register_escape_fn(handlebars::no_escape);
        hb.register_template_string("index", include_str!("index.ts.hbs"))?;
        let output = hb.render("index", self)?;

        let path = deskulpt_workspace::package_dir("deskulpt-bindings")
            .join("src")
            .join("index.ts");
        std::fs::write(&path, output)?;
        println!("✅ Generated: {}", path.display());

        Ok(())
    }
}
