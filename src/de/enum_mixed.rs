//! Deserialization for ReScript-style mixed enums (unit + struct variants).
//!
//! Mixed enums are deserialized as:
//! - Unit variants: plain strings like `"VariantName"`
//! - Struct variants: objects with TAG field like `{ "TAG": "VariantName", "field": value }`

use crate::de::struct_;
use crate::de::{field_i, Parameters, StructForm};
use crate::fragment::{Fragment, Match};
use crate::internals::ast::{Style, Variant};
use crate::internals::attr;
use crate::private;
use crate::rescript;
use quote::quote;

/// Generates `Deserialize::deserialize` body for a mixed enum (unit + struct variants)
pub(super) fn deserialize(
    params: &Parameters,
    variants: &[Variant],
    cattrs: &attr::Container,
) -> Fragment {
    let this_type = &params.this_type;
    let this_value = &params.this_value;
    let (de_impl_generics, de_ty_generics, ty_generics, where_clause) =
        params.generics_with_de_lifetime();
    let delife = params.borrowed.de_lifetime();

    let expecting = format!("mixed enum {}", params.type_name());
    let expecting = cattrs.expecting().unwrap_or(&expecting);

    let tag = rescript::get_effective_tag(cattrs);

    // Unit variants - deserialize from strings
    let unit_variants: Vec<_> = variants
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.attrs.skip_deserializing() && matches!(v.style, Style::Unit))
        .collect();

    let unit_variant_arms: Vec<_> = unit_variants
        .iter()
        .map(|(_, variant)| {
            let variant_name = variant.attrs.name().deserialize_name();
            let variant_ident = &variant.ident;
            quote! {
                #variant_name => _serde::#private::Ok(#this_value::#variant_ident),
            }
        })
        .collect();

    let unit_variant_names: Vec<_> = unit_variants
        .iter()
        .map(|(_, v)| v.attrs.name().deserialize_name())
        .collect();

    // Struct variants
    let struct_variants: Vec<_> = variants
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.attrs.skip_deserializing() && matches!(v.style, Style::Struct))
        .collect();

    // Generate variant enum for struct variants
    let struct_variant_field_idents: Vec<_> =
        struct_variants.iter().map(|(i, _)| field_i(*i)).collect();

    let struct_variant_names: Vec<_> = struct_variants
        .iter()
        .map(|(_, v)| v.attrs.name().deserialize_name())
        .collect();

    // Build variant enum match arms
    let variant_match_arms: Vec<_> = struct_variants
        .iter()
        .map(|(i, v)| {
            let name = v.attrs.name().deserialize_name();
            let ident = field_i(*i);
            quote! { #name => _serde::#private::Ok(__Field::#ident) }
        })
        .collect();

    // Generate variant enum for struct variants (used to deserialize the TAG value)
    let field_enum = if struct_variants.is_empty() {
        quote! {}
    } else {
        quote! {
            #[doc(hidden)]
            enum __Field {
                #(#struct_variant_field_idents,)*
            }

            impl<#delife> _serde::Deserialize<#delife> for __Field {
                fn deserialize<__D>(__deserializer: __D) -> _serde::#private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<#delife>,
                {
                    struct __FieldVisitor;

                    impl<#delife> _serde::de::Visitor<#delife> for __FieldVisitor {
                        type Value = __Field;

                        fn expecting(&self, __f: &mut _serde::#private::Formatter) -> _serde::#private::fmt::Result {
                            _serde::#private::Formatter::write_str(__f, "variant identifier")
                        }

                        fn visit_str<__E>(self, __value: &str) -> _serde::#private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #(#variant_match_arms,)*
                                _ => _serde::#private::Err(_serde::de::Error::unknown_variant(__value, &[#(#struct_variant_names),*])),
                            }
                        }
                    }

                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
        }
    };

    // Generate match arms for struct variants using struct_::deserialize
    let struct_variant_arms: Vec<_> = struct_variants
        .iter()
        .map(|(i, variant)| {
            let variant_field = field_i(*i);
            let variant_ident = &variant.ident;

            let block = Match(struct_::deserialize(
                params,
                &variant.fields,
                cattrs,
                StructForm::InternallyTagged(variant_ident),
            ));

            quote! {
                __Field::#variant_field => #block
            }
        })
        .collect();

    // For unit variants from string
    let visit_str_body = if unit_variants.is_empty() {
        quote! {
            _serde::#private::Err(_serde::de::Error::custom("expected object with TAG field"))
        }
    } else {
        quote! {
            match __value {
                #(#unit_variant_arms)*
                _ => {
                    let __expected = &[#(#unit_variant_names),*];
                    _serde::#private::Err(_serde::de::Error::unknown_variant(__value, __expected))
                }
            }
        }
    };

    // For struct variants from map: use TaggedContentVisitor pattern
    let visit_map_body = if struct_variants.is_empty() {
        quote! {
            _serde::#private::Err(_serde::de::Error::custom("expected string for unit variant"))
        }
    } else {
        quote! {
            // Collect entries and extract TAG field (similar to TaggedContentVisitor)
            let mut __tag: _serde::#private::Option<__Field> = _serde::#private::None;
            let mut __vec = _serde::#private::Vec::<(
                _serde::#private::de::Content<#delife>,
                _serde::#private::de::Content<#delife>,
            )>::new();

            while let _serde::#private::Some(__key) = _serde::de::MapAccess::next_key_seed(&mut __map, _serde::#private::de::ContentVisitor::new())? {
                // Check if this key matches the tag field
                let __is_tag = match &__key {
                    _serde::#private::de::Content::String(__s) => __s == #tag,
                    _serde::#private::de::Content::Str(__s) => *__s == #tag,
                    _ => false,
                };
                if __is_tag {
                    if __tag.is_some() {
                        return _serde::#private::Err(_serde::de::Error::duplicate_field(#tag));
                    }
                    __tag = _serde::#private::Some(_serde::de::MapAccess::next_value(&mut __map)?);
                } else {
                    let __v = _serde::de::MapAccess::next_value_seed(&mut __map, _serde::#private::de::ContentVisitor::new())?;
                    __vec.push((__key, __v));
                }
            }

            let __tag = match __tag {
                _serde::#private::Some(__t) => __t,
                _serde::#private::None => return _serde::#private::Err(_serde::de::Error::missing_field(#tag)),
            };

            // Create ContentDeserializer from the remaining fields
            let __content = _serde::#private::de::Content::Map(__vec);
            let __deserializer = _serde::#private::de::ContentDeserializer::<__A::Error>::new(__content);

            match __tag {
                #(#struct_variant_arms)*
            }
        }
    };

    quote_block! {
        #field_enum

        #[doc(hidden)]
        struct __Visitor #de_impl_generics #where_clause {
            marker: _serde::#private::PhantomData<#this_type #ty_generics>,
            lifetime: _serde::#private::PhantomData<&#delife ()>,
        }

        impl #de_impl_generics _serde::de::Visitor<#delife> for __Visitor #de_ty_generics #where_clause {
            type Value = #this_type #ty_generics;

            fn expecting(&self, __formatter: &mut _serde::#private::Formatter) -> _serde::#private::fmt::Result {
                _serde::#private::Formatter::write_str(__formatter, #expecting)
            }

            fn visit_str<__E>(self, __value: &str) -> _serde::#private::Result<Self::Value, __E>
            where
                __E: _serde::de::Error,
            {
                #visit_str_body
            }

            fn visit_map<__A>(self, mut __map: __A) -> _serde::#private::Result<Self::Value, __A::Error>
            where
                __A: _serde::de::MapAccess<#delife>,
            {
                #visit_map_body
            }
        }

        _serde::Deserializer::deserialize_any(
            __deserializer,
            __Visitor {
                marker: _serde::#private::PhantomData::<#this_type #ty_generics>,
                lifetime: _serde::#private::PhantomData,
            },
        )
    }
}
