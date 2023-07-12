mod parse;
mod render;

use crate::parse::AttributeArgument;
use crate::render::render_props;
use parse::{Component, Prop, PropGenerics};
use proc_macro::TokenStream;
use syn::{GenericArgument, Ident, PathArguments, Type, TypeParam};

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    let struct_ = syn::parse::<syn::ItemStruct>(input).expect("failed to parse struct");
    let arg = syn::parse::<AttributeArgument>(args).expect("failed to parse attribute args");

    let fields = match struct_.fields {
        syn::Fields::Named(fields) => fields.named,
        _ => panic!("struct must have named fields"),
    };

    let struct_generics = struct_
        .generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(type_param) => PropGenerics {
                param: type_param.clone(),
            },
            _ => panic!("struct must have only type params"),
        })
        .collect::<Vec<_>>();

    let props = fields.iter().map(|field| {
        let is_signal = field.attrs.iter().any(|a| a.path().is_ident("signal"));

        // Extract generics from field, if any, and make sure they are matched exactly once to the structs generics
        let field_generics = get_type_generic_param_use(&field.ty, &struct_generics);

        if field_generics.len() > 1 {
            panic!("field must have at most one generic param");
        }

        let generics = field_generics.first().map(|generic| {
            if struct_generics.iter().filter(|g| g == &generic).count() != 1 {
                panic!("field generic param must match exactly one struct generic param");
            }

            generic.clone()
        });

        Prop {
            is_signal,
            name: field.ident.clone().expect("field must have name"),
            generics,
            type_: field.ty.clone(),
        }
    });

    let mut cmp: Component = Component {
        name: struct_.ident,
        render_fn: arg.fn_name,
        props: props.collect(),
    };

    let apply_prop = Prop {
        is_signal: false,
        name: Ident::new("apply", cmp.name.span()),
        generics: Some(PropGenerics { param: syn::parse_str::<TypeParam>("TApplyFn: FnOnce(dominator::DomBuilder<web_sys::HtmlElement>) -> dominator::DomBuilder<web_sys::HtmlElement> = fn(dominator::DomBuilder<web_sys::HtmlElement>)->dominator::DomBuilder<web_sys::HtmlElement>").expect("failed to parse type param") }),
        type_: syn::parse_str::<Type>("TApplyFn").expect("failed to parse type"),
    };

    cmp.props.push(apply_prop);

    render_props(&cmp).into()
}

fn get_type_generic_param_use(
    type_: &Type,
    struct_generics: &Vec<PropGenerics>,
) -> Vec<PropGenerics> {
    let mut out = vec![];

    if let Type::Path(type_path) = &type_ {
        for segment in &type_path.path.segments {
            if let Some(generic) = struct_generics
                .iter()
                .find(|generic| segment.ident == generic.param.ident)
            {
                out.push(generic.clone());
            }

            if let PathArguments::AngleBracketed(angle_bracketed_arguments) = &segment.arguments {
                for argument in &angle_bracketed_arguments.args {
                    match &argument {
                        GenericArgument::Type(Type::Path(generic_type)) => {
                            for segment in generic_type.path.segments.iter() {
                                if let Some(generic) = struct_generics
                                    .iter()
                                    .find(|generic| segment.ident == generic.param.ident)
                                {
                                    out.push(generic.clone());
                                }
                            }
                        }
                        GenericArgument::Type(type_) => {
                            out.append(&mut get_type_generic_param_use(type_, struct_generics));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    out
}
