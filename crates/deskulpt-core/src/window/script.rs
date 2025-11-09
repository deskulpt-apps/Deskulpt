//! Window initialization scripts.

use anyhow::Result;
use deskulpt_settings::Settings;
use serialize_to_javascript::{DefaultTemplate, Template, default_template};

/// Template for the manager window initialization script.
#[derive(Template)]
#[default_template("manager.js")]
pub struct ManagerInitJS<'a> {
    /// `window.__DESKULPT_INTERNALS__.initialSettings`
    initial_settings: &'a Settings,
}

/// Template for the canvas window initialization script.
#[derive(Template)]
#[default_template("canvas.js")]
pub struct CanvasInitJS<'a> {
    /// `window.__DESKULPT_INTERNALS__.apisWrapper`
    apis_wrapper: &'static str,
    /// `window.__DESKULPT_INTERNALS__.initialSettings`
    initial_settings: &'a Settings,
}

impl<'a> ManagerInitJS<'a> {
    /// Generate JavaScript code for initializing the manager window.
    pub fn generate(initial_settings: &'a Settings) -> Result<String> {
        let template = Self { initial_settings };
        let serialized = template.render_default(&Default::default())?;
        Ok(serialized.into_string())
    }
}

impl<'a> CanvasInitJS<'a> {
    /// Generate JavaScript code for initializing the canvas window.
    pub fn generate(initial_settings: &'a Settings) -> Result<String> {
        let template = Self {
            apis_wrapper: include_str!("../../gen/apis.wrapper.js"),
            initial_settings,
        };
        let serialized = template.render_default(&Default::default())?;
        Ok(serialized.into_string())
    }
}
