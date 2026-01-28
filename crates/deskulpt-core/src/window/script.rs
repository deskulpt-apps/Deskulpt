//! Window initialization scripts.

use anyhow::Result;
use deskulpt_settings::types::Settings;
use serialize_to_javascript::{DefaultTemplate, Template, default_template};

/// Template for Deskulpt portal initialization script.
#[derive(Template)]
#[default_template("portal.js")]
pub struct PortalInitJS<'a> {
    /// `window.__DESKULPT_INTERNALS__.initialSettings`
    initial_settings: &'a Settings,
}

/// Template for Deskulpt canvas initialization script.
#[derive(Template)]
#[default_template("canvas.js")]
pub struct CanvasInitJS<'a> {
    /// `window.__DESKULPT_INTERNALS__.apisWrapper`
    apis_wrapper: &'static str,
    /// `window.__DESKULPT_INTERNALS__.initialSettings`
    initial_settings: &'a Settings,
}

impl<'a> PortalInitJS<'a> {
    /// Generate JavaScript code for initializing Deskulpt portal.
    pub fn generate(initial_settings: &'a Settings) -> Result<String> {
        let template = Self { initial_settings };
        let serialized = template.render_default(&Default::default())?;
        Ok(serialized.into_string())
    }
}

impl<'a> CanvasInitJS<'a> {
    /// Generate JavaScript code for initializing Deskulpt canvas.
    pub fn generate(initial_settings: &'a Settings) -> Result<String> {
        let template = Self {
            apis_wrapper: include_str!("../../gen/apis.wrapper.js"),
            initial_settings,
        };
        let serialized = template.render_default(&Default::default())?;
        Ok(serialized.into_string())
    }
}
