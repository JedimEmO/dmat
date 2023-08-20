use crate::event_sourced_model::EventSourcedStructInfo;
use crate::render::render_mutable_updater;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn render_event_sourced_derive(info: EventSourcedStructInfo) -> proc_macro::TokenStream {
    let event_type = render_event_sourced_event_type(&info);
    let impl_ = render_events_sourced_struct_impl(&info);
    let event_sourced_impl = render_events_sourced_impl(&info);

    let out = quote! {
        #event_type
        #impl_
        #event_sourced_impl
    };

    out.into()
}

fn render_events_sourced_impl(info: &EventSourcedStructInfo) -> TokenStream {
    let struct_name = info.name.clone();
    let event_type_name = event_type_name(info);
    let event_update_type_name = event_update_type_name(info);

    let non_vec_mutables = info.mutables.iter().filter(|f| !f.is_mutable_vec());
    let vec_mutables = info.mutables.iter().filter(|f| f.is_mutable_vec());

    let field_updates_names = non_vec_mutables.clone().map(|f| {
        let name = f.ident.clone();
        quote! { #name, }
    });

    let field_updates_updates = non_vec_mutables.clone().map(|f| {
        let field_name = f.ident.clone();
        let updated = render_mutable_updater(f, false);

        quote! {
            if let Some(#field_name) = #field_name {
                #updated
            }
        }
    });

    let vec_enum_event_updates = vec_mutables.clone().map(|f| {
        let ename = f.ident.to_string().to_case(Case::Pascal);
        let ident = syn::parse_str::<Ident>(format!("Update{}", ename).as_str()).unwrap();
        let field_ident = f.ident.clone();
        quote! { self::#event_type_name::#ident(diff) => {
            let mut s = self.#field_ident.lock_mut();
            futures_signals::signal_vec::MutableVecLockMut::apply_vec_diff(&mut s, diff);
        } }
    });

    let event_sourced_member_updates = info.event_sourced.iter().map(|f| {
        let field_ident = f.ident.clone();
        let ename = f.ident.to_string().to_case(Case::Pascal);
        let field_name = syn::parse_str::<Ident>(format!("Update{}", ename).as_str()).unwrap();

        if f.is_mutable_btree_map() {
            quote! {
                Self::Event::#field_name(update) => {
                    match update {
                        futures_signals_utils::event_sourced::MutableBTreeMapEvent::Insert{key, value} => {
                            let mut s = self.#field_ident.lock_mut();
                            s.insert_cloned(key, value);
                        }
                        futures_signals_utils::event_sourced::MutableBTreeMapEvent::Remove{key} => {
                            let mut s = self.#field_ident.lock_mut();
                            s.remove(&key);
                        }
                        futures_signals_utils::event_sourced::MutableBTreeMapEvent::Event{key, event} => {
                            self.#field_ident.lock_ref().get(&key).unwrap().apply_event(event);
                        }
                    }
                }
            }
        } else {
            quote! {
                Self::Event::#field_name(update) => {
                    let mut s = self.#field_ident.apply_event(update);
                }
            }
        }
    });

    quote! {
        impl futures_signals_utils::event_sourced::EventSourced for #struct_name {
            type Event = #event_type_name;

            fn apply_event(&self, event: Self::Event) -> () {
                match event {
                    #event_type_name::Update(#event_update_type_name { #(#field_updates_names)*}) => {
                        #(#field_updates_updates)*
                    }
                    #(#vec_enum_event_updates)*
                    #(#event_sourced_member_updates)*
                }
            }
        }
    }
}
fn render_events_sourced_struct_impl(info: &EventSourcedStructInfo) -> TokenStream {
    let struct_name = info.name.clone();
    let non_vec_mutables = info.mutables.iter().filter(|f| !f.is_mutable_vec());
    let vec_mutables = info.mutables.iter().filter(|f| f.is_mutable_vec());

    let mut_vec_signals = vec_mutables.clone().map(|f| {
        let ident = f.ident.clone();
        let ty = f
            .template_argument()
            .expect("Mutable field must be a template argument");

        let signal = Ident::new(format!("{}_signal_vec", ident).as_str(), ident.span());
        quote! {
            pub fn #signal(&self) -> impl futures_signals::signal_vec::SignalVec<Item=#ty> {
                self.#ident.signal_vec_cloned()
            }
        }
    });

    let vec_slice_getters = vec_mutables.clone().map(|f| {
        let ident = f.ident.clone();
        let ty = f
            .template_argument()
            .expect("Mutable field must be a template argument");

        let get_ident = Ident::new(format!("get_{}_lock_ref", ident).as_str(), ident.span());

        quote! {
            pub fn #get_ident(&self) -> futures_signals::signal_vec::MutableVecLockRef<#ty> {
                self.#ident.lock_ref()
            }
        }
    });

    let mut_signals = non_vec_mutables.map(|f| {
        let ident = f.ident.clone();
        let ty = f
            .template_argument()
            .expect("Mutable field must be a template argument");

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
    });

    quote! {
         impl #struct_name {
            #(#mut_signals)*
            #(#mut_vec_signals)*
            #(#vec_slice_getters)*
        }
    }
}
fn render_event_sourced_event_type(info: &EventSourcedStructInfo) -> TokenStream {
    let event_type_name = event_type_name(info);
    let event_update_type_name = event_update_type_name(info);

    let non_vec_mutables = info.mutables.iter().filter(|f| !f.is_mutable_vec());
    let vec_mutables = info.mutables.iter().filter(|f| f.is_mutable_vec());

    let vec_enum_events = vec_mutables.clone().map(|f| {
        let ename = f.ident.to_string().to_case(Case::Pascal);
        let ident = syn::parse_str::<Ident>(format!("Update{}", ename).as_str()).unwrap();
        let ty = f
            .template_argument()
            .expect("Mutable field must be a template argument");
        quote! { #ident(futures_signals::signal_vec::VecDiff<#ty>), }
    });

    let field_update_fields = non_vec_mutables.clone().map(|f| {
        let field_name = f.ident.clone();
        let ty = f.ty.clone();

        quote! { pub #field_name: Option<#ty>, }
    });

    let update_event = quote! {
        Update(#event_update_type_name)
    };

    let event_sourced_member_events = info.event_sourced.iter().map(|f| {
        let ename = f.ident.to_string().to_case(Case::Pascal);
        let field_name = syn::parse_str::<Ident>(format!("Update{}", ename).as_str()).unwrap();
        let field_type = f.ty.clone();

        if f.is_mutable_btree_map() {
            let (key_ty, value_ty) = f
                .get_btreemap_type_args()
                .expect("MutableBTreeMap field must have two template arguments");
            quote! { #field_name(MutableBTreeMapEvent<#key_ty, #value_ty>),}
        } else {
            quote! { #field_name(<#field_type as EventSourced>::Event), }
        }
    });

    quote! {
        #[derive(Default, Serialize, Deserialize)]
        pub struct #event_update_type_name {
            #(#field_update_fields)*
        }

        #[derive(Serialize, Deserialize)]
        pub enum #event_type_name {
            #update_event,
            #(#vec_enum_events)*
            #(#event_sourced_member_events)*
        }
    }
}

fn event_update_type_name(info: &EventSourcedStructInfo) -> Ident {
    let event_update_type_name =
        syn::parse_str::<Ident>(format!("{}EventUpdate", info.name).as_str()).unwrap();
    event_update_type_name
}

fn event_type_name(info: &EventSourcedStructInfo) -> Ident {
    let event_type_name = syn::parse_str::<Ident>(format!("{}Event", info.name).as_str()).unwrap();
    event_type_name
}
