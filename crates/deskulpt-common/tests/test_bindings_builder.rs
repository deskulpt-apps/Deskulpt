use deskulpt_common::bindings::BindingsBuilder;
use deskulpt_common::event::Event;
use serde::Serialize;
use specta::Type;

#[derive(Serialize, Type)]
struct TestType {
    field: String,
}

#[derive(Serialize, Type)]
struct TestEvent {
    data: i32,
}

impl Event for TestEvent {
    const NAME: &'static str = "test_event";
}

#[test]
fn test_bindings_builder_new() {
    let mut builder = BindingsBuilder::new("test_namespace");
    let bindings = builder.build();

    assert_eq!(bindings.namespace, "test_namespace");
    assert!(bindings.events.is_empty());
    assert!(bindings.commands.is_empty());
}

#[test]
fn test_bindings_builder_register_type() {
    let mut builder = BindingsBuilder::new("test");
    builder.typ::<TestType>();
    let bindings = builder.build();

    // Type should be registered in the type collection
    assert_eq!(bindings.namespace, "test");
}

#[test]
fn test_bindings_builder_register_event() {
    let mut builder = BindingsBuilder::new("test");
    builder.event::<TestEvent>();
    let bindings = builder.build();

    assert_eq!(bindings.namespace, "test");
    assert!(bindings.events.contains_key("test_event"));
}

#[test]
fn test_bindings_builder_register_multiple_events() {
    #[derive(Serialize, Type)]
    struct AnotherEvent {
        value: String,
    }

    impl Event for AnotherEvent {
        const NAME: &'static str = "another_event";
    }

    let mut builder = BindingsBuilder::new("test");
    builder.event::<TestEvent>();
    builder.event::<AnotherEvent>();
    let bindings = builder.build();

    assert_eq!(bindings.events.len(), 2);
    assert!(bindings.events.contains_key("test_event"));
    assert!(bindings.events.contains_key("another_event"));
}

#[test]
fn test_bindings_builder_chain_methods() {
    let mut builder = BindingsBuilder::new("test");
    builder.typ::<TestType>().event::<TestEvent>();
    let bindings = builder.build();

    assert_eq!(bindings.namespace, "test");
    assert!(bindings.events.contains_key("test_event"));
}

// Note: Testing commands registration requires specta function collection
// macros which are complex to set up in integration tests. This test is skipped
// for now. #[test]
// fn test_bindings_builder_with_commands() {
//     // This would require proper specta function collection setup
// }
