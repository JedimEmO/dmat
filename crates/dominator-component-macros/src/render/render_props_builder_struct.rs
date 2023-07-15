use crate::parse::{Component, SignalType};
use crate::render::render_utils::{compute_component_generics, compute_prop_type_ident};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Type;

pub fn render_prop_builder_struct(props_struct_name: Ident, cmp: &Component) -> TokenStream {
    let generics = compute_component_generics(cmp, true, false);

    let props = cmp.props.iter().map(|prop| {
        let name = &prop.name;
        let type_ = compute_prop_type_ident(prop, false);

        let type_: Type = if let Some(_default) = &prop.default {
            type_
        } else {
            syn::parse_str::<Type>(format!("Option<{}>", quote! {#type_}).as_str())
                .expect("failed to parse prop type")
        };

        quote! {
            pub #name: #type_,
        }
    });

    let props_ctor = cmp.props.iter().map(|prop| {
        let name = &prop.name;

        let init_val = if prop.default.is_some() {
            let default = prop.default.as_ref().unwrap();

            if let Some(sig) = &prop.is_signal {
                match sig {
                    SignalType::Item => quote! {futures_signals::signal::always(#default)},
                    SignalType::Vec => quote! {futures_signals::signal_vec::always(#default)},
                }
            } else {
                quote! {#default}
            }
        } else {
            quote! {None}
        };

        quote! {
            #name: #init_val,
        }
    });

    let generics_params_no_self = compute_component_generics(cmp, false, false);
    let generics_params = compute_component_generics(cmp, false, true);
    let generic_idents = generics_params
        .iter()
        .map(|g| g.ident.clone())
        .map(|i| {
            syn::parse_str::<Ident>(quote! {#i}.to_string().as_str())
                .expect("failed to parse generic ident")
        })
        .collect::<Vec<_>>();

    let trait_name = Ident::new(&format!("{}PropsTrait", cmp.name), cmp.name.span());

    let trait_types = generics_params.iter().map(|g| {
        let ident = &g.ident;
        let bounds = &g.bounds;

        quote! {
            type #ident: #bounds;
        }
    });

    let trait_type_impls = generics_params.iter().map(|g| {
        let ident = &g.ident;

        quote! {
            type #ident = #ident;
        }
    });

    let unpack_trait_params_selfed = generics_params
        .iter()
        .map(|g| {
            syn::parse_str::<Type>(format!("Self::{}", g.ident).as_str())
                .expect("failed to parse generic ident")
        })
        .collect::<Vec<_>>();

    let unpack_trait_params = generics_params
        .iter()
        .map(|g| g.ident.clone())
        .collect::<Vec<_>>();

    let docs = cmp.docs.iter().map(|doc| {
        quote! {
            #[doc = #doc]
        }
    });

    quote! {
        pub trait #trait_name {
            #(#trait_types)*

            fn take(self) -> #props_struct_name<#(#unpack_trait_params_selfed,)* >;
        }

        #(#docs)*
        pub struct #props_struct_name<#(#generics,)* > {
            #(#props)*
        }

        impl<#(#generics_params_no_self),*> #trait_name for #props_struct_name<#(#generic_idents,)* > {
            #(#trait_type_impls)*

            fn take(self) -> #props_struct_name<#(#unpack_trait_params,)* > {
                self
            }
        }

        impl #props_struct_name {
            pub fn new() -> Self {
                Self {
                    #(#props_ctor)*
                }
            }
        }
    }
}
