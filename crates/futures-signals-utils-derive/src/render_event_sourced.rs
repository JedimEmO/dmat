use crate::event_sourced_model::EventSourcedStructInfo;
use crate::render::render_mutable_updater;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn render_event_sourced_derive(info: EventSourcedStructInfo) -> proc_macro::TokenStream {
    let struct_name = info.name.clone();

    let event_type = render_event_sourced_event_type(&info);

    let out = quote! {
        impl #struct_name {

        }

        #event_type
    };

    out.into()
}

fn render_event_sourced_event_type(info: &EventSourcedStructInfo) -> TokenStream {
    let event_type_name = syn::parse_str::<Ident>(format!("{}Event", info.name).as_str()).unwrap();
    let event_update_type_name =
        syn::parse_str::<Ident>(format!("{}EventUpdate", info.name).as_str()).unwrap();
    let struct_name = info.name.clone();

    let field_update_fields = info.mutables.iter().map(|f| {
        let field_name = f.ident.clone();
        let field_type = f.ty.clone();
        quote! { pub #field_name: Option<#field_type>, }
    });

    let field_updates_names = info.mutables.iter().map(|f| {
        let name = f.ident.clone();
        quote! { #name, }
    });

    let field_updates_updates = info.mutables.iter().map(|f| {
        let field_name = f.ident.clone();
        let updated = render_mutable_updater(f, false);

        quote! {
            if let Some(#field_name) = #field_name {
                #updated
            }
        }
    });

    let update_event = quote! {
        Update(#event_update_type_name)
    };

    let mut_signals = info.mutables.iter().map(|f| {
        let ident = f.ident.clone();
        let ty = f
            .template_argument()
            .expect("Mutable field must be a template argument");

        if f.is_mutable_vec() {
            let signal = Ident::new(format!("{}_signal_vec", ident).as_str(), ident.span());
            quote! {
                pub fn #signal(&self) -> impl futures_signals::signal_vec::SignalVec<Item=#ty> {
                    self.#ident.signal_vec_cloned()
                }
            }
        } else {
            let signal = Ident::new(format!("{}_signal", ident).as_str(), ident.span());
            let get_ident = Ident::new(format!("get_{}", ident).as_str(), ident.span());
            quote! {
                pub fn #get_ident(&self) -> #ty {
                    self.#ident.get_cloned()
                }

                pub fn #signal(&self) -> impl futures_signals::signal::Signal<Item=#ty> {
                    self.#ident.signal_cloned()
                }
            }
        }
    });

    quote! {
        #[derive(Default)]
        pub struct #event_update_type_name {
            #(#field_update_fields)*
        }

        pub enum #event_type_name {
            #update_event
        }

        impl futures_signals_utils::event_sourced::EventSourced for #struct_name {
            type Event = #event_type_name;

            fn apply_event(&self, event: Self::Event) -> () {
                match event {
                    #event_type_name::Update(#event_update_type_name { #(#field_updates_names)*}) => {
                        #(#field_updates_updates)*
                    }
                }
            }
        }

        impl #struct_name {
            #(#mut_signals)*
        }
    }
}
