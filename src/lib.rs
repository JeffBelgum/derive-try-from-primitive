#![crate_type = "proc-macro"]

#![recursion_limit = "192"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(TryFromPrimitive, attributes(TryFromPrimitive))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();
    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let result = match ast.body {
        syn::Body::Enum(ref variants) => try_from_primitive_for_enum(&ast, &variants),
        syn::Body::Struct(_) => panic!("#[derive(TryFromPrimitive)] not supported for structs"),
    };
    result.to_string().parse().expect("Couldn't parse string to tokens")
}

fn try_from_primitive_for_enum(ast: &syn::MacroInput, variants: &[syn::Variant]) -> quote::Tokens {
    if variants.is_empty() {
        panic!("#[derive(TryFromPrimitive)] cannot be implemented for enums with zero variants");
    }

    let _impl = try_from_primitive(ast, variants);
    quote!(#_impl)
}

use syn::Lit::Int;
use syn::ConstExpr::Lit;

fn try_from_primitive(ast: &syn::MacroInput, variants: &[syn::Variant]) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let doc = format!("TryFrom for `{}`.", name);
    let lint_attrs = collect_parent_lint_attrs(&ast.attrs);
    let lint_attrs = quote![#(#lint_attrs),*];

    let repr = find_repr_attr(&ast.attrs);


    let mut discr = None;
    let match_arms = variants.iter().map(|v| {
        let v_name = &v.ident;
        if let Some(Lit(Int(d, _))) = v.discriminant {
            discr = Some(d)
        } else {
            discr = Some(discr.unwrap_or(0));
        }
        if let Some(ref ident) = repr {
            match ident.as_ref() {
                "u8" => {
                    let discr = discr.unwrap() as u8;
                    quote!(#discr => Some(#name::#v_name))
                }
                "u16" => {
                    let discr = discr.unwrap() as u16;
                    quote!(#discr => Some(#name::#v_name))
                }
                "u32" => {
                    let discr = discr.unwrap() as u32;
                    quote!(#discr => Some(#name::#v_name))
                }
                "u64" => {
                    let discr = discr.unwrap() as u64;
                    quote!(#discr => Some(#name::#v_name))
                }
                "usize" => {
                    let discr = discr.unwrap() as usize;
                    quote!(#discr => Some(#name::#v_name))
                }
                "i8" => {
                    let discr = discr.unwrap() as i8;
                    quote!(#discr => Some(#name::#v_name))
                }
                "i16" => {
                    let discr = discr.unwrap() as i16;
                    quote!(#discr => Some(#name::#v_name))
                }
                "i32" => {
                    let discr = discr.unwrap() as i32;
                    quote!(#discr => Some(#name::#v_name))
                }
                "i64" => {
                    let discr = discr.unwrap() as i64;
                    quote!(#discr => Some(#name::#v_name))
                }
                "isize" => {
                    let discr = discr.unwrap() as isize;
                    quote!(#discr => Some(#name::#v_name))
                }
                ty => {
                    panic!("#[derive(TryFromPrimitive)] does not support enum repr type {:?}",
                    ty);
                }
            }
        } else {
            let discr = discr.unwrap() as usize;
            quote!(#discr => Some(#name::#v_name))
        }
    });
    let match_arms = quote![#(#match_arms),*];


    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[doc = #doc]
            #lint_attrs
            pub fn try_from(n: #repr) -> Option<#name> {
                match n {
                    #match_arms,
                    _ => None
                }
            }
        }
    }
}

fn collect_parent_lint_attrs(attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
    fn is_lint(item: &syn::MetaItem) -> bool {
        if let syn::MetaItem::List(ref ident, _) = *item {
            match ident.as_ref() {
                "allow" | "deny" | "forbid" | "warn" => return true,
                _ => (),
            }
        }
        false
    }

    fn is_cfg_attr_lint(item: &syn::MetaItem) -> bool {
        if let syn::MetaItem::List(ref ident, ref items) = *item {
            if ident.as_ref() == "cfg_attr" && items.len() == 2 {
                if let syn::NestedMetaItem::MetaItem(ref item) = items[1] {
                    return is_lint(item);
                }
            }
        }
        false
    }

    attrs.iter().filter(|attr| {
        is_lint(&attr.value) || is_cfg_attr_lint(&attr.value)
    }).cloned().collect()
}

fn find_repr_attr(attrs: &[syn::Attribute]) -> Option<syn::Ident> {
    fn is_size_repr(item: &syn::MetaItem) -> bool {
        if let syn::MetaItem::List(ref ident, _) = *item {
            ident.as_ref() == "repr"
        } else {
            false
        }
    }

    let repr_attrs: Vec<_> = attrs.iter().filter(|attr| {
        is_size_repr(&attr.value)
    }).collect();

    if repr_attrs.is_empty() {
        None
    } else {
        let attr = repr_attrs[repr_attrs.len() - 1].clone().value;
        match attr {
            syn::MetaItem::List(_, ref nested) => {
                match nested[0] {
                    syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) => Some(ident.clone()),
                    _ => panic!("bug in #[derive(TryFromPrimitive)]. Please file an issue on https://github.com/jeffbelgum/derive_from_primitive")
                }
            }
            _ => panic!("bug in #[derive(TryFromPrimitive)]. Please file an issue on https://github.com/jeffbelgum/derive_from_primitive")
        }
    }
}
