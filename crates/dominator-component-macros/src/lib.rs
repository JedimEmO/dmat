/*
usage example

component!(
    name: Button
    render_fn: button
    props: {
        signal label: String,
        click_handler<TClickHandler: FnMut() -> () = fn() -> ()>: Option<TClickHandler>,
        disabled: Option<bool>,
    }
);

fn button(props: ButtonProps) -> Dom {}

output:

pub enum ButtonLabelVariant {
    String(String),
    Dom(Option<Dom>),
}

pub struct ButtonProps<TLabelSignal: Signal<Item=ButtonLabelVariant> = Always<ButtonLabelVariant>, TClickHandler: FnMut()->() = fn() -> ()> {
    pub label: TLabelSignal,
    pub click_handler: Option<TClickHandler>,
    pub disabled: Option<bool>,
}

impl ButtonProps {
    pub fn new<T: Signal<Item=ButtonLabelVariant>(label: T) -> ButtonProps<T> {
        ButtonProps {
            label,
            click_handler: None,
            disabled: None,
        }
    }
}

impl<TLabelSignal: Signal<Item=ButtonLabelVariant> = Always<ButtonLabelVariant>, TClickHandler: FnMut()->() = fn() -> ()> ButtonProps<TLabelSignal, TClickHandler> {
    pub fn label(&self, v: ButtonLabelVariant) -> ButtonProps<Always<ButtonLabelVariant>, TClickHandler> {
        ButtonProps {
            label: always(v,
            click_handler: None,
            disabled: None,
        }
    }

    pub fn label_signal<T: Signal<Item=ButtonLabelVariant>>(self, v: T) -> ButtonProps<T, TClickHandler> {
        ButtonProps {
            label: v,
            click_handler: self.click_handler,
            disabled: self.disabled,
        }
    }

    pub fn click_handler<T: FnMut()->()>(self, v: T) -> ButtonProps<TLabelSignal, T> {
        ButtonProps {
            label: self.label,
            click_handler: Some(v),
            disabled: self.disabled,
        }
    }

    pub fn disabled(self, v: bool) -> ButtonProps<TLabelSignal, TClickHandler> {
        ButtonProps {
            label: self.label,
            click_handler: self.click_handler,
            disabled: Some(v),
        }
    }
}
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, GenericParam, Generics, Ident, Path, Token, Type, TypeParam};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Gt, Lt};


#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let cmp: Component = syn::parse(input).unwrap();

    let struct_name = Ident::new(&format!("{}Props", cmp.name), cmp.name.span());

    // Extract the generics from the props
    let mut generics = Vec::<TypeParam>::default();

    for prop in cmp.props.iter() {
        if let Some(ref prop_generics) = prop.generics {
            let param = prop_generics.param.clone();
            generics.push(param);
        }
    }

    let props = cmp.props.iter().map(|prop| {
        let name = &prop.name;
        let type_ = &prop.type_;

        quote! {
            pub #name: #type_,
        }
    });

    let props2 = props.clone();
    let props3 = props.clone();

    let s = quote! {
        pub struct #struct_name<#(#generics,)* > {
            #(#props)*
        }

        impl #struct_name {
            pub fn new() {
            }
        }
    };

    s.into()
}

#[derive(Clone)]
struct PropGenerics {
    pub param: TypeParam,
}

impl Parse for PropGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ : Token![<] = input.parse()?;

        let param = input.parse::<TypeParam>()?;

        let _ : Token![>] = input.parse()?;

        Ok(PropGenerics {
            param: param,
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

        let (is_signal, name) = if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            let _: Ident = input.parse()?;
            let _: Token![>] = input.parse()?;
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

