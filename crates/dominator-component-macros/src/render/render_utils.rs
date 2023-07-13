use crate::parse::{Component, Prop, SignalType};
use proc_macro2::Ident;
use quote::quote;
use syn::{Type, TypeParam};

pub fn new_prop_signal_name(prop_name: &Ident) -> String {
    format!("T{}SignalNew", prop_name)
}

pub fn prop_signal_name(prop_name: &Ident) -> String {
    format!("T{}Signal", prop_name)
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

        if prop.is_signal.is_some() {
            let ty_ = &prop.type_;

            let prop_type = if prop.generics.is_some() && include_self_prefix {
                syn::parse_str::<Type>(format!("Self::{}", quote! {#ty_}).as_str())
                    .expect("failed to parse prop type")
            } else {
                ty_.clone()
            };

            let prop_signal_type = get_prop_signal_type_param(
                prop,
                prop.is_signal.as_ref().unwrap(),
                &prop_type,
                false,
            );
            let prop_signal_always_type =
                get_prop_signal_always_type(prop.is_signal.as_ref().unwrap(), &prop_type);

            let param = match include_defaults {
                true => syn::parse_str(
                    format!(
                        "{} = {}",
                        quote! {#prop_signal_type},
                        quote! {#prop_signal_always_type}
                    )
                    .as_str(),
                )
                .expect("failed to parse prop signal type with default"),
                false => syn::parse_str(format!("{}", quote! {#prop_signal_type}).as_str())
                    .expect("failed to parse prop signal type"),
            };

            generics.push(param);
        }
    }

    generics
}

pub fn compute_prop_type_ident(prop: &Prop, include_self_prefix: bool) -> Type {
    if prop.is_signal.is_some() {
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

pub fn get_prop_signal_always_type(signal_type: &SignalType, prop_type: &Type) -> Type {
    match signal_type {
        SignalType::Item => syn::parse_str(
            format!("futures_signals::signal::Always<{}>", quote! {#prop_type}).as_str(),
        )
        .expect("failed to generate signal always"),

        SignalType::Vec => syn::parse_str(
            format!(
                "futures_signals::signal_vec::Always<{}>",
                quote! {#prop_type}
            )
            .as_str(),
        )
        .expect("failed to generate signal_vec always"),
    }
}

pub fn get_prop_signal_type_param(
    prop: &Prop,
    signal_type: &SignalType,
    prop_type: &Type,
    is_new: bool,
) -> TypeParam {
    let signal_name = if is_new {
        new_prop_signal_name(&prop.name)
    } else {
        prop_signal_name(&prop.name)
    };

    match signal_type {
        SignalType::Item => syn::parse_str(
            format!(
                "{}: futures_signals::signal::Signal<Item={}>",
                signal_name,
                quote! {#prop_type}
            )
            .as_str(),
        )
        .expect("failed to parse signal generic"),

        SignalType::Vec => syn::parse_str(
            format!(
                "{}: futures_signals::signal_vec::SignalVec<Item={}>",
                signal_name,
                quote! {#prop_type}
            )
            .as_str(),
        )
        .expect("failed to parse signal generic"),
    }
}
