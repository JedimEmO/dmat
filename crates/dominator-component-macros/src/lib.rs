mod render;
mod parse;

use syn::{Ident, Type, TypeParam};
use parse::{Component, Prop, PropGenerics};
use render::render_props;

#[proc_macro]
pub fn component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut cmp: Component = syn::parse(input.into()).unwrap();

    let apply_prop = Prop {
        is_signal: false,
        name: Ident::new("apply", cmp.name.span()),
        generics: Some(PropGenerics { param: syn::parse_str::<TypeParam>("TApplyFn: FnOnce(dominator::DomBuilder<web_sys::HtmlElement>) -> dominator::DomBuilder<web_sys::HtmlElement> = fn(dominator::DomBuilder<web_sys::HtmlElement>)->dominator::DomBuilder<web_sys::HtmlElement>").expect("failed to parse type param") }),
        type_: syn::parse_str::<Type>("TApplyFn").expect("failed to parse type"),
    };

    cmp.props.push(apply_prop);

    render_props::render_props(&cmp).into()
}

