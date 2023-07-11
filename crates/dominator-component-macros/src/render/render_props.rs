use crate::parse::{Component, Prop};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::quote;
use syn::{Type, TypeParam};

/// Renders the props builder struct along with all the impls of type changing prop setters
pub fn render_props(cmp: &Component) -> TokenStream {
    let props_struct_name = Ident::new(&format!("{}Props", cmp.name), cmp.name.span());

    let props_struct_ts = render_prop_builder_struct(props_struct_name.clone(), cmp);
    let props_impl_ts = cmp
        .props
        .iter()
        .map(|prop| render_prop_impl(&props_struct_name, prop, cmp));
    let macro_ = render_component_macro(cmp);

    let mut s = quote! {
        #props_struct_ts
        #(#props_impl_ts)*
    };

    s.extend(macro_);
    s
}

fn render_prop_builder_struct(props_struct_name: Ident, cmp: &Component) -> TokenStream {
    let generics = compute_component_generics(cmp, true, false);

    let props = cmp.props.iter().map(|prop| {
        let name = &prop.name;
        let type_ = compute_prop_type_ident(prop, false);

        let type_: Type = syn::parse_str(format!("Option<{}>", quote! {#type_}).as_str())
            .expect("failed to parse prop type");

        quote! {
            pub #name: #type_,
        }
    });

    let props_ctor = cmp.props.iter().map(|prop| {
        let name = &prop.name;

        quote! {
            #name: None,
        }
    });

    let generics_params_no_self = compute_component_generics(cmp, false, false);
    let generics_params = compute_component_generics(cmp, false, true);
    let generic_idents = generics_params
        .iter()
        .map(|g| g.ident.clone())
        .map(|i| {
            syn::parse_str::<Ident>(quote! {#i}.to_string().as_str())
                .expect("failed to parse generic ident")
        })
        .collect::<Vec<_>>();

    let trait_name = Ident::new(&format!("{}PropsTrait", cmp.name), cmp.name.span());

    let trait_types = generics_params.iter().map(|g| {
        let ident = &g.ident;
        let bounds = &g.bounds;

        quote! {
            type #ident: #bounds;
        }
    });

    let trait_type_impls = generics_params.iter().map(|g| {
        let ident = &g.ident;

        quote! {
            type #ident = #ident;
        }
    });

    let trait_field_getters = cmp.props.iter().map(|prop| {
        let name = &prop.name;
        let type_ = compute_prop_type_ident(prop, false);

        quote! {
            fn #name(&mut self) -> Option<#type_> {
                self.#name.take()
            }
        }
    });

    let trait_field_getter_decls = cmp.props.iter().map(|prop| {
        let name = &prop.name;
        let type_ = compute_prop_type_ident(prop, true);

        quote! {
            fn #name(&mut self) -> Option<#type_>;
        }
    });

    quote! {
        pub trait #trait_name {
            #(#trait_types)*
            #(#trait_field_getter_decls)*
        }

        #[derive(Default)]
        pub struct #props_struct_name<#(#generics,)* > {
            #(#props)*
        }

        impl<#(#generics_params_no_self),*> #trait_name for #props_struct_name<#(#generic_idents,)* > {
            #(#trait_type_impls)*

            #(#trait_field_getters)*
        }

        impl #props_struct_name {
            pub fn new() -> Self {
                Self {
                    #(#props_ctor)*
                }
            }
        }
    }
}

fn render_prop_impl(props_struct_name: &Ident, prop: &Prop, cmp: &Component) -> TokenStream {
    let generics = compute_component_generics(cmp, false, false);
    let generic_idents = generics
        .iter()
        .map(|g| g.ident.clone())
        .map(|i| {
            syn::parse_str(quote! {#i}.to_string().as_str()).expect("failed to parse generic ident")
        })
        .collect::<Vec<_>>();
    let prop_name = &prop.name;

    let mut changed_generics: Vec<TypeParam> = vec![];
    let mut out_rewrites = vec![];
    let mut ty_ = prop.type_.clone();
    let is_generic_type = prop.generics.is_some();

    if is_generic_type {
        let generic = prop.generics.clone().unwrap();

        let bounds = generic.param.bounds;
        let new_generic_param =
            syn::parse_str(format!("{}New:{}", generic.param.ident, quote! {#bounds}).as_str())
                .expect("failed to parse signal generic");
        changed_generics.push(new_generic_param);

        let new_type = syn::parse_str::<Type>(format!("{}New", generic.param.ident).as_str())
            .expect("failed to parse new generic param");
        let old_type = generic.param.ident.to_string();
        ty_ = new_type.clone();
        out_rewrites.push((old_type, new_type));
    }

    if prop.is_signal {
        let prop_type = quote! {#ty_}.to_string();

        let param = syn::parse_str(
            format!(
                "{}: futures_signals::signal::Signal<Item={}>",
                new_prop_signal_name(&prop.name),
                prop_type
            )
            .as_str(),
        )
        .expect("failed to parse signal generic");

        let changed_generics_nosig = changed_generics.clone();
        changed_generics.push(param);

        let new_signal_name: Type = syn::parse_str(new_prop_signal_name(&prop.name).as_str())
            .expect("failed to parse new signal name");

        let old_name = format!("T{}Signal", prop.name);

        let prop_signal_always_type = syn::parse_str::<Type>(
            format!("futures_signals::signal::Always<{}>", quote! {#ty_}).as_str(),
        )
        .expect("failed to parse prop signal always type");
        let mut generic_idents_out =
            replace_generic(generic_idents.clone(), &old_name, new_signal_name.clone());
        let mut generic_idents_out_always =
            replace_generic(generic_idents.clone(), &old_name, prop_signal_always_type);

        for (old_type, new_type) in out_rewrites.iter() {
            generic_idents_out =
                replace_generic(generic_idents_out, &old_type.to_string(), new_type.clone());
            generic_idents_out_always = replace_generic(
                generic_idents_out_always,
                &old_type.to_string(),
                new_type.clone(),
            );
        }

        let rest_of_props = cmp.props.iter().filter(|p| p.name != prop.name).map(|p| {
            let name = &p.name;

            quote! {
                #name: self.#name,
            }
        });

        let props_signal_fn_name =
            syn::parse_str::<Ident>(format!("{}_signal", prop.name).as_str())
                .expect("failed to parse props signal fn name");

        quote! {
            impl<#(#generics),*> #props_struct_name<#(#generic_idents),*> {
                pub fn #prop_name<#(#changed_generics_nosig),*>(mut self, v: #ty_) -> #props_struct_name<#(#generic_idents_out_always),*> {
                    self.#props_signal_fn_name(futures_signals::signal::always(v))
                }

                pub fn #props_signal_fn_name<#(#changed_generics),*>(self, v: #new_signal_name) -> #props_struct_name<#(#generic_idents_out),*> {
                    #props_struct_name {
                        #prop_name: Some(v),
                        #(#rest_of_props)*
                    }
                }
            }
        }
    } else {
        let mut generic_idents_out = generic_idents.clone();

        for (old_type, new_type) in out_rewrites.iter() {
            generic_idents_out =
                replace_generic(generic_idents_out, &old_type.to_string(), new_type.clone());
        }

        let rest_of_props = cmp.props.iter().filter(|p| p.name != prop.name).map(|p| {
            let name = &p.name;

            quote! {
                #name: self.#name,
            }
        });

        quote! {
            impl<#(#generics),*> #props_struct_name<#(#generic_idents),*> {
                pub fn #prop_name<#(#changed_generics),*>(mut self, v: #ty_) -> #props_struct_name<#(#generic_idents_out),*> {
                     #props_struct_name {
                        #prop_name: Some(v),
                        #(#rest_of_props)*
                    }
                }
            }
        }
    }
}

fn render_component_macro(cmp: &Component) -> TokenStream {
    let name: Ident = syn::parse_str(cmp.name.to_string().to_case(Case::Snake).as_str())
        .expect("failed to parse component name");
    let render_fn = syn::parse_str::<Ident>(format!("{}", cmp.render_fn).as_str())
        .expect("failed to parse render fn name");
    let props_name = syn::parse_str::<Ident>(format!("{}Props", cmp.name).as_str())
        .expect("failed to parse props name");
    let dollar = Punct::new('$', Spacing::Joint);
    let methods = quote!(#dollar methods);

    let out = quote! {
        #[macro_export]
        macro_rules! #name {
            (#dollar(#methods:tt)*) => {{
                let default_props = #props_name::new();
                let applied_props = dominator::apply_methods!(default_props, #dollar(#methods)*);
                #render_fn (applied_props)
            }}
        }
    };

    out
}

fn replace_generic(
    generic_idents: Vec<Type>,
    old_name: &impl ToString,
    new_name: Type,
) -> Vec<Type> {
    let old_name =
        syn::parse_str::<Type>(old_name.to_string().as_str()).expect("failed to parse old name");

    generic_idents
        .into_iter()
        .map(|ty| {
            if quote! {#ty}.to_string() == quote! {#old_name}.to_string() {
                new_name.clone()
            } else {
                ty
            }
        })
        .collect::<Vec<_>>()
}

fn new_prop_signal_name(prop_name: &Ident) -> String {
    format!("T{}SignalNew", prop_name)
}

fn compute_component_generics(
    cmp: &Component,
    include_defaults: bool,
    include_self_prefix: bool,
) -> Vec<TypeParam> {
    let mut generics = Vec::<TypeParam>::default();

    for prop in cmp.props.iter() {
        if let Some(ref prop_generics) = prop.generics {
            let mut param = prop_generics.param.clone();

            if !include_defaults {
                param.default = None;
            }

            generics.push(param);
        }

        if prop.is_signal {
            let ty_ = &prop.type_;
            let prop_type = if prop.generics.is_some() && include_self_prefix {
                quote! {Self::#ty_}.to_string()
            } else {
                quote! {#ty_}.to_string()
            };

            let param = match include_defaults {
                true => syn::parse_str(format!("T{}Signal: futures_signals::signal::Signal<Item={}> = futures_signals::signal::Always<{}>", prop.name, prop_type, prop_type).as_str()).expect("failed to parse signal generic"),
                false => syn::parse_str(format!("T{}Signal: futures_signals::signal::Signal<Item={}>", prop.name, prop_type).as_str()).expect("failed to parse signal generic"),
            };

            generics.push(param);
        }
    }

    generics
}

fn compute_prop_type_ident(prop: &Prop, include_self_prefix: bool) -> Type {
    if prop.is_signal {
        let prefix = if include_self_prefix { "Self::" } else { "" };
        syn::parse_str(format!("{}T{}Signal", prefix, prop.name).as_str())
            .expect("failed to parse signal generic")
    } else {
        let prefix = if prop.generics.is_some() && include_self_prefix {
            "Self::"
        } else {
            ""
        };

        let ty_ = prop.type_.clone();
        let ty_ = quote! {#ty_}.to_string();
        syn::parse_str(format!("{}{}", prefix, ty_).as_str()).expect("failed to parse prop type")
    }
}
