use deskulpt_common::outcome::Outcome;
use serde_json;

#[test]
fn test_outcome_from_result_ok() {
    let result: Result<String, &str> = Ok("success".to_string());
    let outcome: Outcome<String> = result.into();

    match outcome {
        Outcome::Ok(value) => assert_eq!(value, "success"),
        Outcome::Err(_) => panic!("Expected Ok variant"),
    }
}

#[test]
fn test_outcome_from_result_err() {
    let result: Result<String, &str> = Err("error message");
    let outcome: Outcome<String> = result.into();

    match outcome {
        Outcome::Ok(_) => panic!("Expected Err variant"),
        Outcome::Err(msg) => assert!(msg.contains("error message")),
    }
}

#[test]
fn test_outcome_serialization_ok() {
    let outcome = Outcome::<String>::Ok("test".to_string());
    let json = serde_json::to_string(&outcome).unwrap();

    assert!(json.contains("\"type\":\"ok\""));
    assert!(json.contains("\"content\":\"test\""));
}

#[test]
fn test_outcome_serialization_err() {
    let outcome = Outcome::<String>::Err("error".to_string());
    let json = serde_json::to_string(&outcome).unwrap();

    assert!(json.contains("\"type\":\"err\""));
    assert!(json.contains("\"content\":\"error\""));
}

#[test]
fn test_outcome_deserialization_ok() {
    let json = r#"{"type":"ok","content":"test"}"#;
    let outcome: Outcome<String> = serde_json::from_str(json).unwrap();

    match outcome {
        Outcome::Ok(value) => assert_eq!(value, "test"),
        Outcome::Err(_) => panic!("Expected Ok variant"),
    }
}

#[test]
fn test_outcome_deserialization_err() {
    let json = r#"{"type":"err","content":"error message"}"#;
    let outcome: Outcome<String> = serde_json::from_str(json).unwrap();

    match outcome {
        Outcome::Ok(_) => panic!("Expected Err variant"),
        Outcome::Err(msg) => assert_eq!(msg, "error message"),
    }
}

#[test]
fn test_outcome_with_integer() {
    let result: Result<i32, &str> = Ok(42);
    let outcome: Outcome<i32> = result.into();

    match outcome {
        Outcome::Ok(value) => assert_eq!(value, 42),
        Outcome::Err(_) => panic!("Expected Ok variant"),
    }
}

#[test]
fn test_outcome_from_anyhow_error() {
    use anyhow::anyhow;
    let result: Result<String, anyhow::Error> = Err(anyhow!("anyhow error"));
    let outcome: Outcome<String> = result.into();

    match outcome {
        Outcome::Ok(_) => panic!("Expected Err variant"),
        Outcome::Err(msg) => assert!(msg.contains("anyhow error")),
    }
}
