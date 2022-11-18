use proc_macro::{TokenStream};
use std::fmt::format;
use std::ops::Add;
use syn::parse::{Parse};
use syn::parse::ParseStream;
use syn::{Expr, ExprMatch, parse, Stmt};
use syn::__private::quote::__private::ext::RepToTokensExt;
use syn::punctuated::Punctuated;
use syn::Ident;
use syn::Token;
use syn::token::Comma;
use syn::Arm;
use syn::__private::ToTokens;

#[proc_macro]
pub fn match_2_value(item:TokenStream)->TokenStream{
    struct Params{
        stmt:Stmt,
        enum_values:Punctuated<Ident,Comma>,
    }
    impl Parse for Params{
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let stmt = input.parse()?;
            input.parse::<Comma>();

            let enum_values = Punctuated::<Ident,Comma>::parse_separated_nonempty(input)?;
            Ok(Self{
                stmt: stmt,
                enum_values,
            })
        }
    }


    let params:Params = syn::parse(item).unwrap();

    let mut ret = String::new();
    ret+="match ($a,$b){";
    for x in &params.enum_values {
        for y in &params.enum_values {
            ret += format!("(Value::{}($a),Value::{}($b))=>{{{}}},",x,y,params.stmt.clone().into_token_stream()).as_str();
        }
    }
    ret+="};";

    println!("{}",ret);
    println!("dbg");
    println!("dbg");
    println!("dbg");
    println!("dbg");
    println!("dbg");
    println!("dbg");
    println!("dbg");
    ret.pop();

    return ret.parse().unwrap();
}

