#![crate_type = "proc-macro"]
#![recursion_limit = "192"]

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, DeriveInput},
};

#[proc_macro_derive(TryFromPrimitive, attributes(TryFromPrimitive))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let result = match &input.data {
        syn::Data::Enum(data) => {
            try_from_primitive_for_enum(&input, data.variants.iter().cloned().collect())
        }
        syn::Data::Struct(_) => panic!("#[derive(TryFromPrimitive)] not supported for structs"),
        syn::Data::Union(_) => panic!("#[derive(TryFromPrimitive)] not supported for unions"),
    };
    result
        .to_string()
        .parse()
        .expect("Couldn't parse string to tokens")
}

fn try_from_primitive_for_enum(
    ast: &syn::DeriveInput,
    variants: Vec<syn::Variant>,
) -> proc_macro2::TokenStream {
    if variants.is_empty() {
        panic!("#[derive(TryFromPrimitive)] cannot be implemented for enums with zero variants");
    }

    let _impl = try_from_primitive(ast, variants);
    quote!(#_impl)
}

fn try_from_primitive(
    ast: &syn::DeriveInput,
    variants: Vec<syn::Variant>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let doc = format!(
        "Generated impl [TryFrom](std::convert::TryFrom) for `{}`.",
        name
    );
    let lint_attrs = collect_parent_lint_attrs(&ast.attrs);
    let lint_attrs = quote![#(#lint_attrs),*];
    let repr = find_repr_attr(&ast.attrs);

    let mut discr = None;
    let match_arms = variants.iter().map(|v| {
        let v_name = &v.ident;
        if let Some((_, syn::Expr::Lit(l))) = &v.discriminant {
            match &l.lit {
                syn::Lit::Int(int_lit) => {
                    match int_lit.base10_parse::<u64>() {
                        Ok(d) => discr = Some(d),
                        Err(e) => panic!("Could not parse Enum variant in #[derive(TryFromPrimitive)]. Reason: {}. If you believe this is a bug, please file an issue on https://github.com/jeffbelgum/derive_from_primitive", e),
                    }
                }
                _ => panic!("Enum discriminant must be an integer literal"),
            }
        } else {
            discr = Some(discr.map(|d| d + 1).unwrap_or(0));
        }
        if let Some(ref ident) = repr {
            match &*ident.to_string() {
                "u8" => {
                    let discr = discr.unwrap() as u8;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "u16" => {
                    let discr = discr.unwrap() as u16;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "u32" => {
                    let discr = discr.unwrap() as u32;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "u64" => {
                    let discr = discr.unwrap() as u64;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "usize" => {
                    let discr = discr.unwrap() as usize;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "i8" => {
                    let discr = discr.unwrap() as i8;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "i16" => {
                    let discr = discr.unwrap() as i16;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "i32" => {
                    let discr = discr.unwrap() as i32;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "i64" => {
                    let discr = discr.unwrap() as i64;
                    quote!(#discr => Ok(#name::#v_name))
                }
                "isize" => {
                    let discr = discr.unwrap() as isize;
                    quote!(#discr => Ok(#name::#v_name))
                }
                ty => {
                    panic!("#[derive(TryFromPrimitive)] does not support enum repr type {:?}",
                    ty);
                }
            }
        } else {
            let discr = discr.unwrap() as usize;
            quote!(#discr => Ok(#name::#v_name))
        }
    });
    let match_arms = quote![#(#match_arms),*];

    quote! {
        impl #impl_generics core::convert::TryFrom<#repr> for #name #ty_generics #where_clause {
            type Error = #repr;

            #[doc = #doc]
            #lint_attrs
            fn try_from(n: #repr) -> core::result::Result<Self, Self::Error> {
                match n {
                    #match_arms,
                    _ => Err(n)
                }
            }
        }
    }
}

fn collect_parent_lint_attrs(attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
    fn is_lint(item: &syn::Meta) -> bool {
        if let syn::Meta::List(ref l) = *item {
            let path = &l.path;
            path.is_ident("allow")
                || path.is_ident("deny")
                || path.is_ident("forbid")
                || path.is_ident("warn")
        } else {
            false
        }
    }

    fn is_cfg_attr_lint(item: &syn::Meta) -> bool {
        if let syn::Meta::List(ref l) = *item {
            if l.path.is_ident("cfg_attr") && l.nested.len() == 2 {
                if let syn::NestedMeta::Meta(ref item) = l.nested[1] {
                    return is_lint(item);
                }
            }
        }
        false
    }

    attrs
        .iter()
        .filter_map(|a| a.parse_meta().ok().map(|m| (m, a)))
        .filter(|&(ref m, _)| is_lint(m) || is_cfg_attr_lint(m))
        .map(|p| p.1)
        .cloned()
        .collect()
}

fn find_repr_attr(attrs: &[syn::Attribute]) -> Option<syn::Ident> {
    fn is_repr(item: &syn::Meta) -> bool {
        if let syn::Meta::List(ref l) = item {
            l.path.is_ident("repr")
        } else {
            false
        }
    }

    let reprs: Vec<syn::Meta> = attrs
        .iter()
        .flat_map(|attr| {
            attr.parse_meta()
                .map(|meta| if is_repr(&meta) { Some(meta) } else { None })
                .unwrap_or(None)
        })
        .collect();

    if reprs.is_empty() {
        return None;
    } else {
        let attr = reprs[reprs.len() - 1].clone();
        match attr {
            syn::Meta::List(ref l) => {
                match l.nested.first().unwrap() {
                    syn::NestedMeta::Meta(syn::Meta::Path(ref p)) => {
                        return p.get_ident().cloned();
                    }
                    _ => panic!("bug in #[derive(TryFromPrimitive)]. Please file an issue on https://github.com/jeffbelgum/derive_from_primitive"),
                }
            }
            _ => panic!("bug in #[derive(TryFromPrimitive)]. Please file an issue on https://github.com/jeffbelgum/derive_from_primitive"),
        }
    }
}
