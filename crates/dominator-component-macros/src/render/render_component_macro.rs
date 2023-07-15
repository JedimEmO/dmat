use crate::parse::{Component, SignalType};
use crate::render::render_utils::get_prop_signal_type_param;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use proc_macro2::{Punct, Spacing};
use quote::quote;

pub fn render_component_macro(cmp: &Component) -> TokenStream {
    let name: Ident = syn::parse_str(cmp.name.to_string().to_case(Case::Snake).as_str())
        .expect("failed to parse component name");
    let render_fn = syn::parse_str::<Ident>(format!("{}", cmp.render_fn).as_str())
        .expect("failed to parse render fn name");
    let props_name = syn::parse_str::<Ident>(format!("{}Props", cmp.name).as_str())
        .expect("failed to parse props name");
    let dollar = Punct::new('$', Spacing::Joint);
    let methods = quote!(#dollar methods);

    let docs = create_generated_macro_docs_section(cmp, &name);

    let out = quote! {
        #docs
        #[macro_export]
        macro_rules! #name {
            (#dollar(#methods:tt)*) => {{
                let default_props = #props_name::new();
                let applied_props = dominator::apply_methods!(default_props, #dollar(#methods)*);
                #render_fn (applied_props)
            }}
        }
    };

    out
}

fn create_generated_macro_docs_section(cmp: &Component, macro_name: &Ident) -> TokenStream {
    let mut doc_strings = vec![
        "This macro is generated by the `dominator-component-macros` crate.\n".to_string(),
        "All possible methods:\n".to_string(),
        "```rust".to_string(),
        format!("{}! ({{", macro_name),
    ];

    for prop in &cmp.props {
        let ty_ = &prop.type_;

        if let Some(signal) = &prop.is_signal {
            match signal {
                SignalType::Item => {
                    doc_strings.push(format!("    .{}(<{}>)", prop.name, quote! {#ty_}));
                    let ty_ = get_prop_signal_type_param(prop, signal, ty_, false);
                    doc_strings.push(format!("    .{}_signal(<{}>)", prop.name, quote! {#ty_}));
                }
                SignalType::Vec => {
                    doc_strings.push(format!(
                        "    .{}(<Into<Vec<{}>>>)",
                        prop.name,
                        quote! {#ty_}
                    ));
                    let ty_ = get_prop_signal_type_param(prop, signal, ty_, false);
                    doc_strings.push(format!(
                        "    .{}_signal_vec(<{}>)",
                        prop.name,
                        quote! {#ty_}
                    ));
                }
            }
        } else {
            doc_strings.push(format!("    .{}(<{}>)", prop.name, quote! {#ty_}));
        }
    }

    doc_strings.push("});".to_string());
    doc_strings.push("```".to_string());

    let doc_props = doc_strings
        .into_iter()
        .map(|s| {
            let _expr = syn::parse_str::<syn::Expr>(format!("\"{}\"", s).as_str())
                .expect("failed to parse doc expr");
            quote! {#[doc = #s]}
        })
        .collect::<Vec<_>>();

    quote! { #(#doc_props)* }
}
