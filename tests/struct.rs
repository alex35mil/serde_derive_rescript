use serde_derive_rescript::{DeserializeDto, SerializeDto};
use serde_json::{self as json, json};

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
struct BasicStruct {
    field_a: usize,
    field_b: String,
}

#[test]
fn test_struct_serialization() {
    let value = BasicStruct {
        field_a: 42,
        field_b: "hello".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(
        serialized,
        json!({
            "fieldA": 42,
            "fieldB": "hello"
        }),
    );
}

#[test]
fn test_struct_deserialization() {
    let value = json!({
        "fieldA": 42,
        "fieldB": "hello"
    });
    let deserialized: BasicStruct = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        BasicStruct {
            field_a: 42,
            field_b: "hello".to_string()
        },
    );
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
struct StructWithRenameAllRule {
    field_a: usize,
    field_b: String,
}

#[test]
fn test_struct_with_rename_all_rule_serialization() {
    let value = StructWithRenameAllRule {
        field_a: 42,
        field_b: "hello".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(
        serialized,
        json!({
            "field-a": 42,
            "field-b": "hello"
        }),
    );
}

#[test]
fn test_struct_with_rename_all_rule_deserialization() {
    let value = json!({
        "field-a": 42,
        "field-b": "hello"
    });
    let deserialized: StructWithRenameAllRule = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        StructWithRenameAllRule {
            field_a: 42,
            field_b: "hello".to_string(),
        },
    );
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
struct StructWithRenamedField {
    field_a: usize,
    #[serde(rename = "field-b")]
    field_b: String,
}

#[test]
fn test_struct_with_renamed_field_serialization() {
    let value = StructWithRenamedField {
        field_a: 42,
        field_b: "hello".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(
        serialized,
        json!({
            "fieldA": 42,
            "field-b": "hello"
        })
    );
}

#[test]
fn test_struct_with_renamed_field_deserialization() {
    let value = json!({
        "fieldA": 42,
        "field-b": "hello"
    });
    let deserialized: StructWithRenamedField = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        StructWithRenamedField {
            field_a: 42,
            field_b: "hello".to_string()
        }
    );
}

#[derive(SerializeDto, PartialEq, Debug)]
struct StructWithSkippedField {
    field_a: usize,
    #[serde(skip)]
    field_b: String,
}

#[test]
fn test_struct_with_skipped_field_serialization() {
    let value = StructWithSkippedField {
        field_a: 42,
        field_b: "hello".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "fieldA": 42 }));
}

#[derive(SerializeDto, PartialEq, Debug)]
struct StructWithSkippedIfField {
    field_a: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_b: Option<String>,
}

#[test]
fn test_struct_with_some_skipped_if_field_serialization() {
    let value = StructWithSkippedIfField {
        field_a: 42,
        field_b: Some("hello".to_string()),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "fieldA": 42, "fieldB": "hello" }));
}

#[test]
fn test_struct_with_none_skipped_if_field_serialization() {
    let value = StructWithSkippedIfField {
        field_a: 42,
        field_b: None,
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "fieldA": 42 }));
}
