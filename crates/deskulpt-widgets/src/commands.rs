use serde::Serialize;
use tauri::{AppHandle, Runtime};

use crate::WidgetsExt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl specta::Type for Error {
    fn inline(
        type_map: &mut specta::TypeCollection,
        generics: specta::Generics,
    ) -> specta::datatype::DataType {
        <String as specta::Type>::inline(type_map, generics)
    }

    fn reference(
        type_map: &mut specta::TypeCollection,
        generics: &[specta::datatype::DataType],
    ) -> specta::datatype::reference::Reference {
        <String as specta::Type>::reference(type_map, generics)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// TODO
#[tauri::command]
#[specta::specta]
pub async fn bundle<R: Runtime>(
    _app_handle: AppHandle<R>,
    _ids: Option<Vec<String>>,
) -> Result<()> {
    todo!()
}

/// TODO
#[tauri::command]
#[specta::specta]
pub async fn rescan<R: Runtime>(app_handle: AppHandle<R>) -> Result<()> {
    app_handle.widgets().rescan()?;
    Ok(())
}
