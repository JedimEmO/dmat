pub mod parse_field;

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Token, Type, TypeParam};

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
        self.param.ident == other.param.ident
    }
}

#[derive(Clone)]
pub enum SignalType {
    Item,
    Vec,
}

#[derive(Clone)]
pub struct Prop {
    pub is_signal: Option<SignalType>,
    pub name: Ident,
    pub generics: Option<PropGenerics>,
    pub type_: Type,
    pub default: Option<syn::Expr>,
}

pub struct Component {
    pub name: Ident,
    pub render_fn: Ident,
    pub props: Punctuated<Prop, Token![,]>,
}

impl Parse for AttributeArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let param = input.parse::<Ident>()?;
        let eq = input.parse::<Token![=]>()?;
        let fn_name = input.parse::<Ident>()?;

        Ok(AttributeArgument { param, eq, fn_name })
    }
}
