use serde::Serialize;

/// Serializable wrapper around [`anyhow::Error`].
///
/// This implements [`Serialize`] with the [`Debug`] representation of the
/// error. Any error that can be converted into an [`anyhow::Error`] can be
/// converted into this error type, meaning that error propagation with `?`
/// works in the same way as with [`anyhow::Error`].
#[derive(Debug)]
pub struct SerError(anyhow::Error);

impl<E> From<E> for SerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        SerError(err.into())
    }
}

impl Serialize for SerError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(format!("{:?}", self.0).as_str())
    }
}

impl specta::Type for SerError {
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

/// Result type with [`SerError`].
///
/// This is serializable as long as `T` is serializable.
pub type SerResult<T> = Result<T, SerError>;

#[doc(hidden)]
#[macro_export]
macro_rules! __ser_bail {
    ($msg:literal $(,)?) => {
        return Err(anyhow::anyhow!($msg).into())
    };
    ($err:expr $(,)?) => {{
        return Err(anyhow::anyhow!($err).into())
    }};
    ($fmt:expr, $($arg:tt)*) => {
        return Err(anyhow::anyhow!($fmt, $($arg)*).into())
    };
}

/// [`anyhow::bail!`] that returns early with a [`SerError`].
#[doc(inline)]
pub use __ser_bail as ser_bail;
