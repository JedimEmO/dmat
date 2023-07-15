use crate::parse::{Component, Prop, SignalType};
use crate::render::render_doc_exprs;
use crate::render::render_utils::{
    compute_component_generics, get_prop_signal_always_type, get_prop_signal_type_param,
    new_prop_signal_name, prop_signal_name,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Type, TypeParam};

pub fn render_prop_impl(props_struct_name: &Ident, prop: &Prop, cmp: &Component) -> TokenStream {
    let generics = compute_component_generics(cmp, false, false);
    let generic_idents = generics
        .iter()
        .map(|g| g.ident.clone())
        .map(|i| {
            syn::parse_str(quote! {#i}.to_string().as_str()).expect("failed to parse generic ident")
        })
        .collect::<Vec<_>>();
    let prop_name = &prop.name;

    let mut changed_generics: Vec<TypeParam> = vec![];
    let mut out_rewrites = vec![];
    let mut ty_ = prop.type_.clone();
    let is_generic_type = prop.generics.is_some();

    let docs = render_doc_exprs(&prop.docs);

    let value_assign_expr = if let Some(_default) = &prop.default {
        quote! {v}
    } else {
        quote! {Some(v)}
    };

    if is_generic_type {
        let generic = prop.generics.clone().unwrap();

        let bounds = generic.param.bounds;
        let new_generic_param =
            syn::parse_str(format!("{}New:{}", generic.param.ident, quote! {#bounds}).as_str())
                .expect("failed to parse signal generic");
        changed_generics.push(new_generic_param);

        let new_type = syn::parse_str::<Type>(format!("{}New", generic.param.ident).as_str())
            .expect("failed to parse new generic param");
        let old_type = generic.param.ident.to_string();
        ty_ = new_type.clone();
        out_rewrites.push((old_type, new_type));
    }

    if prop.is_signal.is_some() {
        let param = get_prop_signal_type_param(prop, prop.is_signal.as_ref().unwrap(), &ty_, true);
        let prop_signal_always_type =
            get_prop_signal_always_type(prop.is_signal.as_ref().unwrap(), &ty_);

        let changed_generics_nosig = changed_generics.clone();
        changed_generics.push(param);

        let new_signal_name: Type = syn::parse_str(new_prop_signal_name(&prop.name).as_str())
            .expect("failed to parse new signal name");

        let old_name = prop_signal_name(&prop.name);

        let mut generic_idents_out =
            replace_generic(generic_idents.clone(), &old_name, new_signal_name.clone());
        let mut generic_idents_out_always =
            replace_generic(generic_idents.clone(), &old_name, prop_signal_always_type);

        for (old_type, new_type) in out_rewrites.iter() {
            generic_idents_out =
                replace_generic(generic_idents_out, &old_type.to_string(), new_type.clone());
            generic_idents_out_always = replace_generic(
                generic_idents_out_always,
                &old_type.to_string(),
                new_type.clone(),
            );
        }

        let rest_of_props = cmp.props.iter().filter(|p| p.name != prop.name).map(|p| {
            let name = &p.name;

            quote! {
                #name: self.#name,
            }
        });

        let props_signal_fn_name = match prop.is_signal.as_ref().unwrap() {
            SignalType::Item => syn::parse_str::<Ident>(format!("{}_signal", prop.name).as_str())
                .expect("failed to parse props signal fn name"),
            SignalType::Vec => {
                syn::parse_str::<Ident>(format!("{}_signal_vec", prop.name).as_str())
                    .expect("failed to parse props signal fn name")
            }
        };

        let signal_mod_ident = match prop.is_signal.as_ref().unwrap() {
            SignalType::Item => Ident::new("signal", prop.type_.span()),
            SignalType::Vec => Ident::new("signal_vec", prop.type_.span()),
        };

        let always_value_type = match prop.is_signal.as_ref().unwrap() {
            SignalType::Item => quote! {#ty_},
            SignalType::Vec => quote! {impl Into<Vec<#ty_>>},
        };

        quote! {
            impl<#(#generics),*> #props_struct_name<#(#generic_idents),*> {
                #docs
                pub fn #prop_name<#(#changed_generics_nosig),*>(mut self, v: #always_value_type) -> #props_struct_name<#(#generic_idents_out_always),*> {
                    self.#props_signal_fn_name(futures_signals::#signal_mod_ident::always(v.into()))
                }

                #docs
                pub fn #props_signal_fn_name<#(#changed_generics),*>(self, v: #new_signal_name) -> #props_struct_name<#(#generic_idents_out),*> {
                    #props_struct_name {
                        #prop_name: #value_assign_expr,
                        #(#rest_of_props)*
                    }
                }
            }
        }
    } else {
        let mut generic_idents_out = generic_idents.clone();

        for (old_type, new_type) in out_rewrites.iter() {
            generic_idents_out =
                replace_generic(generic_idents_out, &old_type.to_string(), new_type.clone());
        }

        let rest_of_props = cmp.props.iter().filter(|p| p.name != prop.name).map(|p| {
            let name = &p.name;

            quote! {
                #name: self.#name,
            }
        });

        quote! {
            impl<#(#generics),*> #props_struct_name<#(#generic_idents),*> {
                #docs
                pub fn #prop_name<#(#changed_generics),*>(mut self, v: #ty_) -> #props_struct_name<#(#generic_idents_out),*> {
                     #props_struct_name {
                        #prop_name: #value_assign_expr,
                        #(#rest_of_props)*
                    }
                }
            }
        }
    }
}

fn replace_generic(
    generic_idents: Vec<Type>,
    old_name: &impl ToString,
    new_name: Type,
) -> Vec<Type> {
    let old_name =
        syn::parse_str::<Type>(old_name.to_string().as_str()).expect("failed to parse old name");

    generic_idents
        .into_iter()
        .map(|ty| {
            if quote! {#ty}.to_string() == quote! {#old_name}.to_string() {
                new_name.clone()
            } else {
                ty
            }
        })
        .collect::<Vec<_>>()
}
