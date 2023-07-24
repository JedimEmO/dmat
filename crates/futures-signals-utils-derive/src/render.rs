use crate::model::StructInfo;
use proc_macro2::Ident;
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
            fn update(&self, other: &Self) -> () {
                #mutable_updates
            }
        }
    };

    out
}

fn render_mutable_updater(mutables: Vec<Ident>) -> TokenStream {
    let mutables = mutables.iter().map(|ident| {
        quote! {
            self.#ident.update(&other.#ident);
        }
    });
    quote! {
        #(#mutables)*
    }
}
