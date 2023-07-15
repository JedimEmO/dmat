pub mod render_component_macro;
pub mod render_prop_impl;
pub mod render_props_builder_struct;
pub mod render_utils;

use crate::parse::Component;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Expr;

use crate::render::render_component_macro::render_component_macro;

use crate::render::render_prop_impl::render_prop_impl;
use crate::render::render_props_builder_struct::render_prop_builder_struct;

/// Renders the props builder struct along with all the impls of type changing prop setters
pub fn render_props(cmp: &Component) -> TokenStream {
    let props_struct_name = Ident::new(&format!("{}Props", cmp.name), cmp.name.span());

    let props_struct_ts = render_prop_builder_struct(props_struct_name.clone(), cmp);
    let props_impl_ts = cmp
        .props
        .iter()
        .map(|prop| render_prop_impl(&props_struct_name, prop, cmp));
    let macro_ = render_component_macro(cmp);

    let mut s = quote! {
        #props_struct_ts
        #(#props_impl_ts)*
    };

    s.extend(macro_);
    s
}

fn render_doc_exprs(doc_exprs: &Vec<Expr>) -> TokenStream {
    let mut s = TokenStream::new();

    for doc_expr in doc_exprs {
        s.extend(quote! {
            #[doc = #doc_expr]
        });
    }

    s
}
