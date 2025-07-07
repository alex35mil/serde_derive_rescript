use serde_derive_rescript::{DeserializeDto, SerializeDto};
use serde_json::{self as json, json};

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
enum BasicEnum {
    Unit,
    Named { named_field: String },
}

#[test]
fn test_enum_unit_member_serialization() {
    let value = BasicEnum::Unit;
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!("Unit"));
}

#[test]
fn test_enum_member_with_named_fields_serialization() {
    let value = BasicEnum::Named {
        named_field: "named".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "TAG": "Named", "namedField": "named" }));
}

#[test]
fn test_enum_unit_member_deserialization() {
    let value = json!("Unit");
    let deserialized: BasicEnum = json::from_value(value).unwrap();
    assert_eq!(deserialized, BasicEnum::Unit);
}

#[test]
fn test_enum_member_with_named_fields_deserialization() {
    let value = json!({ "TAG": "Named", "namedField": "named" });
    let deserialized: BasicEnum = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        BasicEnum::Named {
            named_field: "named".to_string()
        },
    );
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
enum EnumWithRenameAllRule {
    UnitMember,
    MemberWithNamedField { named_field: String },
}

#[test]
fn test_enum_with_rename_all_rule_serialization() {
    let value = EnumWithRenameAllRule::MemberWithNamedField {
        named_field: "named".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(
        serialized,
        json!({
            "TAG": "member-with-named-field",
            "namedField": "named"
        }),
    );
}

#[test]
fn test_enum_with_rename_all_rule_deserialization() {
    let value = json!({
        "TAG": "member-with-named-field",
        "namedField": "named"
    });
    let deserialized: EnumWithRenameAllRule = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        EnumWithRenameAllRule::MemberWithNamedField {
            named_field: "named".to_string(),
        },
    );
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
#[serde(rename_all_fields = "kebab-case")]
enum EnumWithRenameAllFieldsRule {
    UnitMember,
    MemberWithNamedField { named_field: String },
}

#[test]
fn test_enum_with_rename_all_fields_rule_serialization() {
    let value = EnumWithRenameAllFieldsRule::MemberWithNamedField {
        named_field: "named".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(
        serialized,
        json!({
            "TAG": "MemberWithNamedField",
            "named-field": "named"
        }),
    );
}

#[test]
fn test_enum_with_rename_all_fields_rule_deserialization() {
    let value = json!({
        "TAG": "MemberWithNamedField",
        "named-field": "named"
    });
    let deserialized: EnumWithRenameAllFieldsRule = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        EnumWithRenameAllFieldsRule::MemberWithNamedField {
            named_field: "named".to_string(),
        },
    );
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
enum EnumWithRenamedMember {
    #[serde(rename = "u")]
    Unit,
}

#[test]
fn test_enum_unit_memeber_with_renamed_member_serialization() {
    let value = EnumWithRenamedMember::Unit;
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!("u"));
}

#[test]
fn test_enum_unit_memeber_with_renamed_member_deserialization() {
    let value = json!("u");
    let deserialized: EnumWithRenamedMember = json::from_value(value).unwrap();
    assert_eq!(deserialized, EnumWithRenamedMember::Unit);
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
enum EnumWithRenamedField {
    Named {
        #[serde(rename = "f")]
        field: String,
    },
}

#[test]
fn test_enum_member_with_renamed_field_serialization() {
    let value = EnumWithRenamedField::Named {
        field: "named".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "TAG": "Named", "f": "named" }));
}

#[test]
fn test_enum_member_with_renamed_field_deserialization() {
    let value = json!({ "TAG": "Named", "f": "named" });
    let deserialized: EnumWithRenamedField = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        EnumWithRenamedField::Named {
            field: "named".to_string(),
        },
    );
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
#[serde(tag = "T")]
enum EnumWithCustomTag {
    Unit,
    Named { field: String },
}

#[test]
fn test_enum_unit_memeber_with_custom_tag_serialization() {
    let value = EnumWithCustomTag::Unit;
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!("Unit"));
}

#[test]
fn test_enum_unit_memeber_with_custom_tag_deserialization() {
    let value = json!("Unit");
    let deserialized: EnumWithCustomTag = json::from_value(value).unwrap();
    assert_eq!(deserialized, EnumWithCustomTag::Unit);
}

#[test]
fn test_enum_member_with_named_field_with_custom_tag_serialization() {
    let value = EnumWithCustomTag::Named {
        field: "named".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(
        serialized,
        json!({
            "T": "Named",
            "field": "named"
        }),
    );
}

#[test]
fn test_enum_member_with_custom_tag_deserialization() {
    let value = json!({ "T": "Named", "field": "named" });
    let deserialized: EnumWithCustomTag = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        EnumWithCustomTag::Named {
            field: "named".to_string(),
        },
    );
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
enum EnumWithSkippedMember {
    #[serde(skip)]
    Unit,
    #[allow(dead_code)]
    Named { field: String },
}

#[test]
fn test_enum_unit_memeber_with_skipped_member_serialization() {
    let value = EnumWithSkippedMember::Unit;
    let serialized = json::to_value(&value);
    assert!(serialized.is_err());
}

#[test]
fn test_enum_unit_memeber_with_skipped_member_deserialization() {
    let value = json!("Unit");
    let deserialized: Result<EnumWithSkippedMember, _> = json::from_value(value);
    assert!(deserialized.is_err());
}

#[derive(SerializeDto, PartialEq, Debug)]
enum EnumWithSkippedField {
    #[allow(dead_code)]
    Unit,
    Named {
        field: String,
        #[serde(skip)]
        _skipped: String,
    },
}

#[test]
fn test_enum_with_skipped_field_serialization() {
    let value = EnumWithSkippedField::Named {
        field: "named".to_string(),
        _skipped: "skipped".to_string(),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "TAG": "Named", "field": "named" }));
}

#[derive(SerializeDto, PartialEq, Debug)]
enum EnumWithSkippedIfField {
    Named {
        #[serde(skip_serializing_if = "Option::is_none")]
        field: Option<String>,
    },
}

#[test]
fn test_enum_with_some_skipped_if_field_serialization() {
    let value = EnumWithSkippedIfField::Named {
        field: Some("named".to_string()),
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "TAG": "Named", "field": "named" }));
}

#[test]
fn test_enum_with_none_skipped_if_field_serialization() {
    let value = EnumWithSkippedIfField::Named { field: None };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "TAG": "Named" }));
}

#[derive(SerializeDto, PartialEq, Debug)]
enum EnumWithUntaggedNamedMember {
    #[serde(untagged)]
    Named {
        #[serde(skip_serializing_if = "Option::is_none")]
        field_a: Option<usize>,
        #[serde(skip)]
        _field_b: usize,
    },
}

#[test]
fn test_enum_with_untagged_named_member_serialization() {
    let value = EnumWithUntaggedNamedMember::Named {
        field_a: Some(42),
        _field_b: 0,
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "fieldA": 42 }));
}

#[test]
fn test_enum_with_empty_untagged_named_member_serialization() {
    let value = EnumWithUntaggedNamedMember::Named {
        field_a: None,
        _field_b: 0,
    };
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({}));
}

#[derive(SerializeDto, PartialEq, Debug)]
enum EnumWithUnnamedMember {
    #[allow(dead_code)]
    Unit,
    Unnamed(#[serde(skip)] usize),
}

#[test]
fn test_enum_with_unnamed_member_serialization() {
    let value = EnumWithUnnamedMember::Unnamed(42);
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!("Unnamed"));
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
enum EnumWithUntaggedUnnamedMember {
    #[allow(dead_code)]
    Unit,
    #[serde(untagged)]
    Unnamed(ChildEnumForUntaggedUnnamedEnum),
}

#[derive(SerializeDto, DeserializeDto, PartialEq, Debug)]
enum ChildEnumForUntaggedUnnamedEnum {
    A,
    B { x: usize },
}

#[test]
fn test_enum_with_untagged_unnamed_member_serialization() {
    let value =
        EnumWithUntaggedUnnamedMember::Unnamed(ChildEnumForUntaggedUnnamedEnum::B { x: 42 });
    let serialized = json::to_value(&value).unwrap();
    assert_eq!(serialized, json!({ "TAG": "B", "x": 42 }));
}

#[test]
fn test_enum_with_untagged_unnamed_member_deserialization() {
    let value = json!({ "TAG": "B", "x": 42 });
    let deserialized: EnumWithUntaggedUnnamedMember = json::from_value(value).unwrap();
    assert_eq!(
        deserialized,
        EnumWithUntaggedUnnamedMember::Unnamed(ChildEnumForUntaggedUnnamedEnum::B { x: 42 })
    );
}
