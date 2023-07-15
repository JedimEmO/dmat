use crate::get_type_generic_param_use;
use crate::parse::{docs_from_attrs, Prop, PropGenerics, SignalType};
use syn::Field;

pub fn parse_field(field: &Field, struct_generics: &Vec<PropGenerics>) -> Prop {
    let is_signal = field.attrs.iter().any(|a| a.path().is_ident("signal"));
    let is_signal_vec = field.attrs.iter().any(|a| a.path().is_ident("signal_vec"));

    let default = field
        .attrs
        .iter()
        .find(|a| a.path().is_ident("default"))
        .map(|a| {
            a.parse_args::<syn::Expr>()
                .expect("failed to parse default value")
        });

    if is_signal && is_signal_vec {
        panic!("field cannot be both signal and signal_vec");
    }

    // Extract generics from field, if any, and make sure they are matched exactly once to the structs generics
    let field_generics = get_type_generic_param_use(&field.ty, struct_generics);

    if field_generics.len() > 1 {
        panic!("field must have at most one generic param");
    }

    let generics = field_generics.first().map(|generic| {
        if struct_generics.iter().filter(|g| g == &generic).count() != 1 {
            panic!("field generic param must match exactly one struct generic param");
        }

        generic.clone()
    });

    let field_docs = docs_from_attrs(field.attrs.iter());

    Prop {
        is_signal: if is_signal {
            Some(SignalType::Item)
        } else if is_signal_vec {
            Some(SignalType::Vec)
        } else {
            None
        },
        name: field.ident.clone().expect("field must have name"),
        generics,
        type_: field.ty.clone(),
        default,
        docs: field_docs,
    }
}
