use crate::model::{get_mutables_from_struct, MutableField};
use anyhow::anyhow;
use proc_macro2::Ident;
use syn::{Data, DataStruct, DeriveInput, Type};

pub struct EventSourcedStructInfo {
    pub name: Ident,
    pub mutables: Vec<MutableField>,
    pub event_sourced: Vec<EventSourcedField>,
}

pub struct EventSourcedField {
    pub ident: Ident,
    pub ty: Type,
}

pub fn parse_event_sourced_struct(input: DeriveInput) -> anyhow::Result<EventSourcedStructInfo> {
    let strct = if let Data::Struct(strct) = input.data {
        strct
    } else {
        return Err(anyhow!("expected a struct"))?;
    };

    let mutables = get_mutables_from_struct(&strct);
    let event_sourced = get_event_sourced_from_struct(&strct);

    Ok(EventSourcedStructInfo {
        name: input.ident,
        mutables,
        event_sourced,
    })
}

fn get_event_sourced_from_struct(strct: &DataStruct) -> Vec<EventSourcedField> {
    strct
        .fields
        .iter()
        .filter_map(|field| {
            if !field.attrs.iter().any(|attr| attr.path().is_ident("skip"))
                && field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("event_sourced"))
            {
                Some(EventSourcedField {
                    ident: field.ident.clone().unwrap(),
                    ty: field.ty.clone(),
                })
            } else {
                None
            }
        })
        .collect()
}
