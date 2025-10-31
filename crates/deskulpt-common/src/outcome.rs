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

impl<T, E: std::fmt::Debug> From<Result<T, E>> for Outcome<T> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => Outcome::Ok(value),
            Err(e) => Outcome::Err(format!("{e:?}")),
        }
    }
}
