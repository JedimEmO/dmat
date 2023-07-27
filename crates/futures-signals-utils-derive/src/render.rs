use crate::model::{MutableField, MutableFieldFlag, StructInfo};
use proc_macro2::TokenStream;
use quote::quote;

pub fn render_derive(struct_info: StructInfo) -> TokenStream {
    let StructInfo {
        name,
        updateables: mutables,
    } = struct_info;

    let mutable_updates = render_mutable_updater(mutables);

    let out = quote! {
        impl Updateable for #name {
            fn update(&self, other: Self) -> () {
                #mutable_updates
            }
        }
    };

    out
}

fn render_mutable_updater(mutables: Vec<MutableField>) -> TokenStream {
    let mutables = mutables.iter().map(|mutable_field| {
        let ident = mutable_field.ident.clone();

        if mutable_field
            .flags
            .iter()
            .any(|f| f == &MutableFieldFlag::UpdateInPlaceCloned)
        {
            quote! {
                futures_signals_utils::update_vec_direct_cloned(&self.#ident, other.#ident);
            }
        } else if mutable_field
            .flags
            .iter()
            .any(|f| f == &MutableFieldFlag::UpdateInPlaceCopied)
        {
            quote! {
                futures_signals_utils::update_vec_direct_copied(&self.#ident, other.#ident);
            }
        } else {
            quote! {
                self.#ident.update(other.#ident);
            }
        }
    });
    quote! {
        #(#mutables)*
    }
}
