mod model;
mod render;

use crate::model::parse_struct_info;
use crate::render::render_derive;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Updateable, attributes(skip))]
pub fn updateable(input: TokenStream) -> TokenStream {
    let inp: DeriveInput = parse_macro_input!(input);

    let struct_info = parse_struct_info(inp).unwrap();

    render_derive(struct_info).into()
}
