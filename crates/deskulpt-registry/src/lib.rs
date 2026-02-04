#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod index;
mod widget;

pub use crate::index::{Index, IndexFetcher};
pub use crate::widget::{WidgetFetcher, WidgetPreview, WidgetReference};
