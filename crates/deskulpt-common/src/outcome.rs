use serde::{Deserialize, Serialize};

/// A result-like binary outcome.
///
/// This represents the outcome of an operation that can either succeed with a
/// value of type `T` or fail with an error message.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum Outcome<T> {
    Ok(T),
    Err(String),
}
