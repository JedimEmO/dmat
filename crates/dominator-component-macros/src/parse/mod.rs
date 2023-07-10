use syn::parse::{Parse, ParseStream};
use syn::{braced, bracketed, Token, Type, TypeParam};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Lt};

pub struct AttributeArgument {
    pub param: Ident,
    pub eq: Token![=],
    pub fn_name: Ident,
}

#[derive(Clone)]
pub struct PropGenerics {
    pub param: TypeParam,
}

impl PartialEq for PropGenerics {
    fn eq(&self, other: &Self) -> bool {
        other.param.ident.to_string() == self.param.ident.to_string()
    }
}

pub struct Attribute {
    pub pound: Token![#],
    pub bracket: Bracket,
    pub content: Ident,
}

#[derive(Clone)]
pub struct Prop {
    pub is_signal: bool,
    pub name: Ident,
    pub generics: Option<PropGenerics>,
    pub type_: Type,
}

pub struct Component {
    pub name: Ident,
    pub render_fn: Ident,
    pub props: Punctuated<Prop, Token![,]>,
}

pub enum ComponentProp {
    Name(Ident),
    RenderFn(Ident),
    Props(Punctuated<Prop, Token![,]>),
}

impl Parse for AttributeArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let param = input.parse::<Ident>()?;
        let eq = input.parse::<Token![=]>()?;
        let fn_name = input.parse::<Ident>()?;

        Ok(AttributeArgument {
            param,
            eq,
            fn_name,
        })
    }
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
                let _braces = braced!(props in input);
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

        let name = component_props.iter().find_map(|prop| {
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
