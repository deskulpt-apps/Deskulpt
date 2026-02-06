//! Common utilities for initializing internal Tauri plugins for Deskulpt.

#[doc(hidden)]
#[macro_export]
macro_rules! __init_builder {
    () => {
        include!(concat!(env!("OUT_DIR"), "/init_builder.rs"));
    };
}

/// Initialize a `tauri::plugin::Builder`.
///
/// The builder has its name automatically set to the crate name, and its
/// invoke handler set to the commands specified in the build script. It can be
/// further customized before calling the `.build()` method. One must configure
/// the build script correctly with `tauri-deskulpt-build`.
#[doc(inline)]
pub use __init_builder as init_builder;
