use proc_macro2::{Punct, Spacing};
use convert_case::{Case, Casing};
use crate::parse::{Component};
use proc_macro2::{Ident, TokenStream};
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

    let out = quote! {
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