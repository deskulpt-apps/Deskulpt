use anyhow::Result;
use handlebars::Handlebars;
use heck::ToPascalCase;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ModuleTemplate {
    module: &'static str,
    namespace: String,
}

#[derive(Default, Debug, Serialize)]
pub struct Template {
    modules: Vec<ModuleTemplate>,
}

impl Template {
    pub fn add_module(&mut self, module: &'static str) {
        self.modules.push(ModuleTemplate {
            module,
            namespace: module.to_pascal_case(),
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
