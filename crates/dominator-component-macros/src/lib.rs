use std::default::Default;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{AngleBracketedGenericArguments, braced, bracketed, GenericParam, Generics, Ident, Path, PathArguments, Token, TraitBound, TraitBoundModifier, Type, TypeParam, TypeParamBound, TypePath};
use syn::PathArguments::AngleBracketed;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Bracket, Comma, Gt, Lt};

#[proc_macro]
pub fn component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut cmp: Component = syn::parse(input.into()).unwrap();


    let apply_prop = Prop {
        is_signal: false,
        name: Ident::new("apply", cmp.name.span()),
        generics: Some(PropGenerics { param: syn::parse_str::<TypeParam>("TApplyFn: FnOnce(dominator::DomBuilder<web_sys::HtmlElement>) -> dominator::DomBuilder<web_sys::HtmlElement> = fn(dominator::DomBuilder<web_sys::HtmlElement>)->dominator::DomBuilder<web_sys::HtmlElement>").expect("failed to parse type param")}),
        type_: syn::parse_str::<Type>("TApplyFn").expect("failed to parse type"),
    };

    cmp.props.push(apply_prop);

    render_props(&cmp).into()
}

fn render_props(cmp: &Component) -> TokenStream {
    let props_struct_name = Ident::new(&format!("{}Props", cmp.name), cmp.name.span());
    let generics = compute_component_generics(cmp, true);

    let props = cmp.props.iter().map(|prop| {
        let name = &prop.name;
        let type_ = compute_prop_type_ident(&prop);

        let type_: Type = syn::parse_str(format!("Option<{}>", quote! {#type_}.to_string()).as_str()).expect("failed to parse prop type");

        quote! {
            pub #name: #type_,
        }
    });

    let props_ctor = cmp.props.iter().map(|prop| {
        let name = &prop.name;
        let type_ = compute_prop_type_ident(&prop);

        quote! {
            #name: None,
        }
    });

    let props_struct_ts = quote! {
        #[derive(Default)]
        pub struct #props_struct_name<#(#generics,)* > {
            #(#props)*
        }

        impl #props_struct_name {
            pub fn new() -> Self {
                Self {
                    #(#props_ctor)*
                }
            }
        }
    };

    let props_impl_ts = cmp.props.iter().map(|prop| render_prop_impl(&props_struct_name, prop, cmp));

    let s = quote! {
        #props_struct_ts
        #(#props_impl_ts)*
    };

    s.into()
}

fn compute_component_generics(cmp: &Component, include_defaults: bool) -> Vec<TypeParam> {
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
            let prop_type = quote! {#ty_}.to_string();

            let param = match include_defaults {
                true => syn::parse_str(format!("T{}Signal: Signal<Item={}> = futures_signals::signal::Always<{}>", prop.name.to_string(), prop_type, prop_type).as_str()).expect("failed to parse signal generic"),
                false => syn::parse_str(format!("T{}Signal: Signal<Item={}>", prop.name.to_string(), prop_type).as_str()).expect("failed to parse signal generic"),
            };

            generics.push(param);
        }
    }


    generics
}

fn compute_prop_type_ident(prop: &Prop) -> Type {
    if prop.is_signal {
        syn::parse_str(format!("T{}Signal", prop.name).as_str()).expect("failed to parse signal generic")
    } else {
        prop.type_.clone()
    }
}

fn new_prop_signal_name(prop_name: &Ident) -> String {
    format!("T{}SignalNew", prop_name.to_string())
}

fn render_prop_impl(props_struct_name: &Ident, prop: &Prop, cmp: &Component) -> TokenStream {
    let generics = compute_component_generics(cmp, false);
    let generic_idents = generics.iter()
        .map(|g| g.ident.clone())
        .map(|i| syn::parse_str(quote! {#i}.to_string().as_str()).expect("failed to parse generic ident"))
        .collect::<Vec<_>>();
    let prop_name = &prop.name;

    let mut changed_generics: Vec<TypeParam> = vec![];
    let mut out_rewrites = vec![];
    let mut ty_ = prop.type_.clone();

    if prop.generics.is_some() {
        let generic = prop.generics.clone().unwrap();

        let bounds = generic.param.bounds;
        let new_generic_param = syn::parse_str(format!("{}New:{}", generic.param.ident.to_string(), quote! {#bounds}.to_string()).as_str()).expect("failed to parse signal generic");
        changed_generics.push(new_generic_param);

        let new_type = syn::parse_str::<Type>(format!("{}New", generic.param.ident.to_string()).as_str()).expect("failed to parse new generic param");
        let old_type = generic.param.ident.to_string();
        ty_ = new_type.clone();
        out_rewrites.push((old_type, new_type));
    }

    if prop.is_signal {
        let prop_type = quote! {#ty_}.to_string();

        let param = syn::parse_str(format!("{}: Signal<Item={}>", new_prop_signal_name(&prop.name), prop_type).as_str()).expect("failed to parse signal generic");

        let changed_generics_nosig = changed_generics.clone();
        changed_generics.push(param);

        let new_signal_name: Type = syn::parse_str(new_prop_signal_name(&prop.name).as_str()).expect("failed to parse new signal name");

        let old_name = format!("T{}Signal", prop.name.to_string());

        let prop_signal_always_type = syn::parse_str::<Type>(format!("futures_signals::signal::Always<{}>", quote! {#ty_}.to_string()).as_str()).expect("failed to parse prop signal always type");
        let mut generic_idents_out = replace_generic(generic_idents.clone(), &old_name, new_signal_name.clone());
        let mut generic_idents_out_always = replace_generic(generic_idents.clone(), &old_name, prop_signal_always_type);

        for (old_type, new_type) in out_rewrites.iter() {
            generic_idents_out = replace_generic(generic_idents_out, &old_type.to_string(), new_type.clone());
            generic_idents_out_always = replace_generic(generic_idents_out_always, &old_type.to_string(), new_type.clone());
        }

        let rest_of_props = cmp.props.iter().filter(|p| p.name != prop.name).map(|p| {
            let name = &p.name;

            quote! {
                #name: self.#name,
            }
        });

        let props_signal_fn_name = syn::parse_str::<Ident>(format!("{}_signal", prop.name).as_str()).expect("failed to parse props signal fn name");

        quote! {
            impl<#(#generics),*> #props_struct_name<#(#generic_idents),*> {
                pub fn #prop_name<#(#changed_generics_nosig),*>(mut self, v: #ty_) -> #props_struct_name<#(#generic_idents_out_always),*> {
                    self.#props_signal_fn_name(futures_signals::signal::always(v))
                }

                pub fn #props_signal_fn_name<#(#changed_generics),*>(self, v: #new_signal_name) -> #props_struct_name<#(#generic_idents_out),*> {
                    #props_struct_name {
                        #prop_name: Some(v),
                        #(#rest_of_props)*
                    }
                }
            }
        }
    } else {
        let mut generic_idents_out = generic_idents.clone();

        for (old_type, new_type) in out_rewrites.iter() {
            generic_idents_out = replace_generic(generic_idents_out, &old_type.to_string(), new_type.clone());
        }

        let rest_of_props = cmp.props.iter().filter(|p| p.name != prop.name).map(|p| {
            let name = &p.name;

            quote! {
                #name: self.#name,
            }
        });

        quote! {
            impl<#(#generics),*> #props_struct_name<#(#generic_idents),*> {
                pub fn #prop_name<#(#changed_generics),*>(mut self, v: #ty_) -> #props_struct_name<#(#generic_idents_out),*> {
                     #props_struct_name {
                        #prop_name: Some(v),
                        #(#rest_of_props)*
                    }
                }
            }
        }
    }
}

fn replace_generic(generic_idents: Vec<Type>, old_name: &impl ToString, new_name: Type) -> Vec<Type> {
    let old_name = syn::parse_str::<Type>(old_name.to_string().as_str()).expect("failed to parse old name");

    generic_idents.into_iter()
        .map(|ty| {
            if quote! {#ty}.to_string() == quote! {#old_name}.to_string() {
                new_name.clone()
            } else {
                ty
            }
        }).collect::<Vec<_>>()
}

#[derive(Clone)]
struct PropGenerics {
    pub param: TypeParam,
}

impl Parse for PropGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![<] = input.parse()?;

        let param = input.parse::<TypeParam>()?;

        let _: Token![>] = input.parse()?;

        Ok(PropGenerics {
            param: param,
        })
    }
}

struct Attribute {
    pound: Token![#],
    bracket: Bracket,
    content: Ident,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pound = input.parse()?;
        let content;
        let bracket = bracketed!(content in input);
        let content: Ident = content.parse()?;
        Ok(Attribute {
            pound,
            bracket,
            content,
        })
    }
}

#[derive(Clone)]
struct Prop {
    is_signal: bool,
    name: Ident,
    generics: Option<PropGenerics>,
    type_: Type,
}

impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (is_signal, name) = if input.peek(Token![#]) {
            let _: Attribute = input.parse()?;
            let name: Ident = input.parse()?;
            (true, name)
        } else {
            (false, input.parse()?)
        };

        let generics = if input.peek(Lt) {
            Some(input.parse::<PropGenerics>()?)
        } else {
            None
        };

        let _: Token![:] = input.parse()?;

        let type_: Type = input.parse()?;

        Ok(Prop {
            is_signal,
            name,
            generics,
            type_,
        })
    }
}

struct Component {
    name: Ident,
    render_fn: Ident,
    props: Punctuated<Prop, Token![,]>,
}

enum ComponentProp {
    Name(Ident),
    RenderFn(Ident),
    Props(Punctuated<Prop, Token![,]>),
}

impl Parse for ComponentProp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prop_name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;

        match prop_name.to_string().as_str() {
            "name" => {
                let component_name: Ident = input.parse()?;
                Ok(ComponentProp::Name(component_name))
            }
            "render_fn" => {
                let render_fn: Ident = input.parse()?;
                Ok(ComponentProp::RenderFn(render_fn))
            }
            "props" => {
                let props;
                let braces = braced!(props in input);
                let props = props.parse_terminated(Prop::parse, Token![,])?;

                Ok(ComponentProp::Props(props))
            }
            _ => Err(syn::Error::new(prop_name.span(), "expected name"))
        }
    }
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_props = input.parse_terminated(ComponentProp::parse, Token![,])?;

        let mut name = component_props.iter().find_map(|prop| {
            match prop {
                ComponentProp::Name(name) => Some(name.clone()),
                _ => None,
            }
        }).ok_or(syn::Error::new(input.span(), "expected name"))?;

        let render_fn = component_props.iter().find_map(|prop| {
            match prop {
                ComponentProp::RenderFn(render_fn) => Some(render_fn.clone()),
                _ => None,
            }
        }).ok_or(syn::Error::new(input.span(), "expected render_fn"))?;

        let props = component_props.into_iter().find_map(|prop| {
            match prop {
                ComponentProp::Props(props) => Some(props.clone()),
                _ => None,
            }
        }).ok_or(syn::Error::new(input.span(), "expected props"))?;

        Ok(Self {
            name,
            render_fn,
            props,
        })
    }
}

