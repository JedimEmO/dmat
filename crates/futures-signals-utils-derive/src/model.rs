use anyhow::anyhow;
use proc_macro2::Ident;
use syn::{Data, DataStruct, DeriveInput, GenericArgument, PathArguments, Type};

#[derive(Clone, PartialEq)]
pub enum MutableFieldFlag {
    UpdateInPlaceCloned,
    UpdateInPlaceCopied,
}

#[derive(Clone)]
pub struct MutableField {
    pub ident: Ident,
    pub ty: Type,
    pub flags: Vec<MutableFieldFlag>,
}

impl MutableField {
    pub fn is_mutable_vec(&self) -> bool {
        if let Type::Path(path) = &self.ty {
            if let Some(segment) = path.path.segments.first() {
                if segment.ident == "MutableVec" {
                    return true;
                }
            }
        }

        false
    }

    pub fn template_argument(&self) -> Option<Type> {
        if let Type::Path(path) = &self.ty {
            if let Some(segment) = path.path.segments.last() {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(ty)) = args.args.first() {
                        return Some(ty.clone());
                    }
                }
            }
        }

        None
    }
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

    let mutables = get_mutables_from_struct(&strct);

    Ok(StructInfo {
        name: input.ident,
        updateables: mutables,
    })
}

pub fn get_mutables_from_struct(strct: &DataStruct) -> Vec<MutableField> {
    strct
        .fields
        .iter()
        .filter_map(|field| {
            if !field.attrs.iter().any(|attr| attr.path().is_ident("skip"))
                && !field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("event_sourced"))
            {
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
