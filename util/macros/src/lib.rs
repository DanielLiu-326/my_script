#![feature(proc_macro_quote,proc_macro_diagnostic)]
use proc_macro::{TokenStream};
use std::collections::HashMap;
use std::env::var;
use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::ItemEnum;
use syn::spanned::Spanned;

use quote::{quote, TokenStreamExt};

#[proc_macro_attribute]
pub fn mux(_attr:TokenStream,mut input:TokenStream) -> TokenStream{
    let item_enum:ItemEnum = syn::parse(input.clone()).unwrap();
    let enum_ty = item_enum.ident.clone();
    let enum_ty_snake = enum_ty.to_string().to_case(Case::Snake);
    let enum_generics = item_enum.generics.clone();

    let mut var_map = HashMap::new();
    item_enum.variants.iter().for_each(|x|{
        let var_span = x.span().unwrap();
        let field_it = x.fields.iter();
        if let (1,Some(field)) = (field_it.clone().count(),field_it.last()){
            if let Some(origin) = var_map.insert(field.ty.clone(),x.ident.clone()) {
                var_span.error("mux enum should have one to one respondence between variant and field type\n").emit();
            }
        }else{
            var_span.error("mux enum should have 1 field in each variant").emit();
        }
    });

    let macro_name = syn::Ident::new(&(enum_ty_snake+"_match"),Span::call_site());
    let vars = var_map.values();
    let mut output:proc_macro2::TokenStream = input.into();

    output.extend(quote!(
        pub(crate) macro #macro_name($input:expr => $var:ident,$output:expr){{
            use #enum_ty :: *;
            match $input{
                #(#vars ($var) => {
                    $output
                })*
            }
        }}
    ));

    output.extend(var_map.iter().map(|(ty,var)|{ quote!(
        impl #enum_generics From<#ty> for #enum_ty #enum_generics{
            #[inline(always)]
            fn from(val:#ty) -> Self {
                Self::#var(val)
            }
        }
    )}).fold(quote!(),|mut input,now|{
        input.extend(now);
        input
    }));

    println!("{:#}",output.to_string());
    output.into()
}
