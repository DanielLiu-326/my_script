use proc_macro::{TokenStream};
use syn::parse::{Parse};
use syn::parse::ParseStream;
use syn::{ExprMatch};
use syn::punctuated::Punctuated;
use syn::Ident;
use syn::Token;
use syn::token::Comma;
use syn::Arm;
use syn::__private::ToTokens;

#[proc_macro]
pub fn match_2_value(item:TokenStream)->TokenStream{
    struct Temp{
        token:proc_macro2::TokenStream
    }
    impl Parse for Temp{
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let input_expr =  input.parse::<syn::Expr>()?;
            input.parse::<Comma>()?;
            let result_expr =  input.parse::<syn::Expr>()?;
            input.parse::<Comma>()?;

            let idents= Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?;

            let mut expr_match:ExprMatch = ExprMatch{
                attrs: vec![],
                match_token: Default::default(),
                expr: Box::new(input_expr),
                brace_token: Default::default(),
                arms: vec![]
            };
            for x in idents.iter(){
                for y in idents.iter(){

                    expr_match.arms.push({
                        Arm{
                            body: Box::new(result_expr.clone()),
                            pat: syn::parse(format!("(Value::{}(a),Value::{}(b))",x,y).parse::<TokenStream>().unwrap()).unwrap(),
                            attrs: vec![],
                            guard: None,
                            fat_arrow_token: Default::default(),
                            comma: Some(Default::default())
                        }
                    });
                }
            }


            Ok(Self{
                token:expr_match.into_token_stream()
            })

        }
    }

    let temp:Temp = syn::parse(item).unwrap();
    proc_macro::TokenStream::from(temp.token)
}

