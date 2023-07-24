use anyhow::anyhow;
use proc_macro2::Ident;
use syn::{Data, DataStruct, DeriveInput};

pub struct StructInfo {
    pub name: Ident,
    pub updateables: Vec<Ident>,
}

pub fn parse_struct_info(input: DeriveInput) -> anyhow::Result<StructInfo> {
    let strct = if let Data::Struct(strct) = input.data {
        strct
    } else {
        return Err(anyhow!("expected a struct"))?;
    };

    let mutables = get_updateables_from_struct(strct);

    Ok(StructInfo {
        name: input.ident,
        updateables: mutables,
    })
}

fn get_updateables_from_struct(strct: DataStruct) -> Vec<Ident> {
    strct
        .fields
        .iter()
        .filter_map(|field| {
            if field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("updateable"))
            {
                field.ident.clone()
            } else {
                None
            }
        })
        .collect()
}
