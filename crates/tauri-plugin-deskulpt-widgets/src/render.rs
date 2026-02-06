//! Bundling and rendering of Deskulpt widgets.

mod alias_plugin;
mod bundler;
mod worker;

pub use worker::{RenderWorkerHandle, RenderWorkerTask};
