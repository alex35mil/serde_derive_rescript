//! Utilities for handling enums with ReScript-style serialization.

use crate::internals::ast::{Style, Variant};
use crate::internals::attr;

/// Default tag field name for internally tagged struct variants in enums - matches ReSript one.
pub const DEFAULT_TAG: &str = "TAG";

/// Determines if an enum is "mixed" - containing both unit and struct variants.
///
/// Mixed enums receive special serialization/deserialization treatment:
/// - Unit variants are serialized as strings
/// - Struct variants are serialized with internal tagging
pub fn is_mixed_enum(variants: &[Variant]) -> bool {
    let has_unit = variants.iter().any(|v| matches!(v.style, Style::Unit));
    let has_struct = variants.iter().any(|v| matches!(v.style, Style::Struct));
    has_unit && has_struct
}

/// Gets the effective tag name for enum, respecting explicit `#[serde(tag = "...")]`
/// attributes but defaulting to `DEFAULT_TAG` if none is specified.
pub fn get_effective_tag(cattrs: &attr::Container) -> String {
    match cattrs.tag() {
        attr::TagType::Internal { tag } => tag.clone(),
        _ => DEFAULT_TAG.to_string(),
    }
}
