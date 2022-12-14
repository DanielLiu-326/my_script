#![feature(proc_macro_quote,proc_macro_diagnostic)]
use proc_macro::{TokenStream};
use std::collections::HashMap;
use std::env::var;
use syn::ItemEnum;
use syn::spanned::Spanned;

use quote::{quote, TokenStreamExt, ToTokens};

#[proc_macro_attribute]
pub fn mux(_attr:TokenStream,mut input:TokenStream) -> TokenStream{
    let item_enum:ItemEnum = syn::parse(input.clone()).unwrap();
    let enum_ty = item_enum.ident.clone();
    let mut var_map = HashMap::new();

    item_enum.variants.iter().for_each(|x|{
        let var_span = x.span().unwrap();
        let field_it = x.fields.iter();
        if let (1,Some(field)) = (field_it.clone().count(),field_it.last()){
            if let Some(origin) = var_map.insert(field.ty.clone(),x.ident.clone()) {
                var_span.error("mux enum should have one to one respondence with variant\n").emit();
            }
        }else{
            var_span.error("mux enum should have 1 field in each variant").emit();
        }
    });

    input.extend(var_map.iter().map(|(ty,var)|{
        quote!(
            impl From<#ty> for #enum_ty{
                #[inline(always)]
                fn from(val:#ty) -> Self {
                    Self::#var(val)
                }
            }
        ).into()
    }).reduce(|mut a:TokenStream,b:TokenStream|{
        a.extend(b);
        a
    }).unwrap());

    println!("{}",input.to_string());
    input
}
