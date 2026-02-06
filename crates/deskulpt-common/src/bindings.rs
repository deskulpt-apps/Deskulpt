//! Common utilities for declaring Deskulpt bindings.

use std::collections::BTreeMap;

use specta::datatype::{DataType, Function};
use specta::{NamedType, Type, TypeCollection};

use crate::event::Event;

/// A collection of types, events, and commands to be exposed to the frontend.
///
/// This should never be constructed manually in bindings providers; instead,
/// configure the build script and use [`build_bindings`].
pub struct Bindings {
    /// The module these bindings belong to, which should be the plugin name.
    pub module: &'static str,
    /// The specta type collection.
    pub types: TypeCollection,
    /// The mapping from event names to their data types.
    pub events: BTreeMap<&'static str, DataType>,
    /// The collection of commands.
    pub commands: Vec<Function>,
}

/// Builder for a [`Bindings`] instance.
pub struct BindingsBuilder {
    module: &'static str,
    types: TypeCollection,
    events: BTreeMap<&'static str, DataType>,
    commands: Option<fn(&mut TypeCollection) -> Vec<Function>>,
}

impl BindingsBuilder {
    /// Create a new [`BindingsBuilder`] instance.
    pub fn new(module: &'static str) -> Self {
        Self {
            module,
            types: Default::default(),
            events: Default::default(),
            commands: Default::default(),
        }
    }

    /// Register a type in the collection.
    pub fn typ<T: NamedType>(&mut self) -> &mut Self {
        self.types.register::<T>();
        self
    }

    /// Register an event in the collection.
    pub fn event<T: Event + Type>(&mut self) -> &mut Self {
        let dt = T::reference(&mut self.types, &[]).inner;
        self.events.insert(T::NAME, dt);
        self
    }

    /// Register commands in the collection.
    ///
    /// The argument should be obtained via the [`collect_commands!`] macro.
    pub fn commands(&mut self, commands: fn(&mut TypeCollection) -> Vec<Function>) -> &mut Self {
        self.commands.replace(commands);
        self
    }

    /// Build the [`Bindings`] instance.
    pub fn build(&mut self) -> Bindings {
        let commands = match self.commands {
            Some(f) => f(&mut self.types),
            None => vec![],
        };

        Bindings {
            module: self.module,
            types: self.types.clone(),
            events: self.events.clone(),
            commands,
        }
    }
}

/// Used in [`BindingsBuilder::commands`].
///
/// <div style="display: none;">
#[doc(inline)]
pub use specta::function::collect_functions as collect_commands;

#[doc(hidden)]
#[macro_export]
macro_rules! __build_bindings {
    () => {
        include!(concat!(env!("OUT_DIR"), "/build_bindings.rs"));
    };
}

/// Create a function that builds [`Bindings`] for this crate.
///
/// The internals of the function are generated at build time, so one must
/// configure the build script correctly with `tauri-deskulpt-build`.
#[doc(inline)]
pub use __build_bindings as build_bindings;
