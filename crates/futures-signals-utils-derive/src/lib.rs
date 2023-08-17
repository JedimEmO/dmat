mod event_sourced_model;
mod model;
mod render;
mod render_event_sourced;

use crate::event_sourced_model::parse_event_sourced_struct;
use crate::model::parse_struct_info;
use crate::render::render_derive;
use crate::render_event_sourced::render_event_sourced_derive;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(
    Updateable,
    attributes(skip, update_in_place_cloned, update_in_place_copied)
)]
pub fn updateable(input: TokenStream) -> TokenStream {
    let inp: DeriveInput = parse_macro_input!(input);

    let struct_info = parse_struct_info(inp).unwrap();

    render_derive(struct_info).into()
}

#[proc_macro_derive(
    EventSourced,
    attributes(skip, event_sourced, update_in_place_cloned, update_in_place_copied)
)]
pub fn event_sourced(input: TokenStream) -> TokenStream {
    let inp: DeriveInput = parse_macro_input!(input);

    let struct_info = parse_event_sourced_struct(inp).unwrap();

    render_event_sourced_derive(struct_info)
}
