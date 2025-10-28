use anyhow::Result;

mod index;
mod module;

pub fn run() -> Result<()> {
    let all_bindings = vec![deskulpt_core::build_bindings()];

    let mut index = index::Template::default();
    for bindings in all_bindings {
        let template = module::Template::from(bindings)?;
        index.add_namespace(template.namespace);
        template.render()?;
    }
    index.render()?;

    Ok(())
}
