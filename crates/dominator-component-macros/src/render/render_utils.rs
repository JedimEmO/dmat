use crate::parse::{Component, Prop};
use proc_macro2::{Ident};
use quote::quote;
use syn::{Type, TypeParam};

pub fn new_prop_signal_name(prop_name: &Ident) -> String {
    format!("T{}SignalNew", prop_name)
}

pub fn compute_component_generics(
    cmp: &Component,
    include_defaults: bool,
    include_self_prefix: bool,
) -> Vec<TypeParam> {
    let mut generics = Vec::<TypeParam>::default();

    for prop in cmp.props.iter() {
        if let Some(ref prop_generics) = prop.generics {
            let mut param = prop_generics.param.clone();

            if !include_defaults {
                param.default = None;
            }

            generics.push(param);
        }

        if prop.is_signal {
            let ty_ = &prop.type_;
            let prop_type = if prop.generics.is_some() && include_self_prefix {
                quote! {Self::#ty_}.to_string()
            } else {
                quote! {#ty_}.to_string()
            };

            let param = match include_defaults {
                true => syn::parse_str(format!("T{}Signal: futures_signals::signal::Signal<Item={}> = futures_signals::signal::Always<{}>", prop.name, prop_type, prop_type).as_str()).expect("failed to parse signal generic"),
                false => syn::parse_str(format!("T{}Signal: futures_signals::signal::Signal<Item={}>", prop.name, prop_type).as_str()).expect("failed to parse signal generic"),
            };

            generics.push(param);
        }
    }

    generics
}

pub fn compute_prop_type_ident(prop: &Prop, include_self_prefix: bool) -> Type {
    if prop.is_signal {
        let prefix = if include_self_prefix { "Self::" } else { "" };
        syn::parse_str(format!("{}T{}Signal", prefix, prop.name).as_str())
            .expect("failed to parse signal generic")
    } else {
        let prefix = if prop.generics.is_some() && include_self_prefix {
            "Self::"
        } else {
            ""
        };

        let ty_ = prop.type_.clone();
        let ty_ = quote! {#ty_}.to_string();
        syn::parse_str(format!("{}{}", prefix, ty_).as_str()).expect("failed to parse prop type")
    }
}
