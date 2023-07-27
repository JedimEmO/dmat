use anyhow::anyhow;
use proc_macro2::Ident;
use syn::{Data, DataStruct, DeriveInput, Type};

#[derive(PartialEq)]
pub enum MutableFieldFlag {
    UpdateInPlaceCloned,
    UpdateInPlaceCopied,
}

pub struct MutableField {
    pub ident: Ident,
    pub ty: Type,
    pub flags: Vec<MutableFieldFlag>,
}

pub struct StructInfo {
    pub name: Ident,
    pub updateables: Vec<MutableField>,
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

fn get_updateables_from_struct(strct: DataStruct) -> Vec<MutableField> {
    strct
        .fields
        .iter()
        .filter_map(|field| {
            if !field.attrs.iter().any(|attr| attr.path().is_ident("skip")) {
                let mut flags = Vec::new();
                let ty = field.ty.clone();

                if field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("update_in_place_cloned"))
                {
                    if let Type::Path(path) = &ty {
                        if let Some(segment) = path.path.segments.last() {
                            if segment.ident == "MutableVec" {
                                flags.push(MutableFieldFlag::UpdateInPlaceCloned);
                            } else {
                                panic!("update_in_place_cloned can only be used on MutableVec");
                            }
                        }
                    } else {
                        panic!("update_in_place_cloned can only be used on MutableVec");
                    }
                }

                if field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("update_in_place_copied"))
                {
                    if let Type::Path(path) = &ty {
                        if let Some(segment) = path.path.segments.last() {
                            if segment.ident == "MutableVec" {
                                flags.push(MutableFieldFlag::UpdateInPlaceCopied);
                            } else {
                                panic!("update_in_place_copied can only be used on MutableVec");
                            }
                        }
                    } else {
                        panic!("update_in_place_copied can only be used on MutableVec");
                    }
                }

                Some(MutableField {
                    ident: field.ident.clone().expect("field must have name"),
                    ty,
                    flags,
                })
            } else {
                None
            }
        })
        .collect()
}
