extern crate core;

use proc_macro::{ TokenStream};
use convert_case::{Case, Casing};
use syn::token::{Comma, FatArrow};
use syn::{Block, braced, Token, TypeTuple, ItemEnum, Expr,Ident};
use syn::__private::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

static mut VAL_ENUM_TYPE : String = String::new();

static mut VAL_ENUMS     : Vec<(String,String)> = Vec::new();

static mut REG_ENUM_TYPE : String = String::new();

static mut REG_ENUMS     : Vec<(String,String)> = Vec::new();

static mut REF_VAL_TYPE  :String = String::new();

static mut REF_MUT_VAL_TYPE:String = String::new();

//unbox the reference into the type that can be operatee.

#[proc_macro_attribute]
pub fn val_enum_def(_attr:TokenStream,input:TokenStream) -> TokenStream{unsafe{
    let mut enum_item = syn::parse::<ItemEnum>(input.clone()).unwrap();

    VAL_ENUM_TYPE = enum_item.ident.to_string();

    for x in enum_item.variants{
        if let (1,Some(field)) = (x.fields.iter().count(),x.fields.iter().last()){
            VAL_ENUMS.push((x.ident.to_string(),field.ty.to_token_stream().to_string()))
        }else{
            Err(syn::Error::new(x.fields.span(),"Value enum must have one field")).unwrap()
        }
    }


    REF_VAL_TYPE = "Ref".to_string() + VAL_ENUM_TYPE.as_str();
    REF_MUT_VAL_TYPE = "RefMut".to_string() + VAL_ENUM_TYPE.as_str();

    let mut ret_code = input.to_string();

    ret_code+=format!("pub enum {}<'a>{{ {} }}",
                      REF_VAL_TYPE,
                      VAL_ENUMS.iter().map(|a|{
                          format!("{}(&'a {}),",(*a).0,(*a).1)
                      }).reduce(|res,now|{res+&now}).unwrap()
    ).as_str();

    for (enum_variant, type_name) in &VAL_ENUMS{
        ret_code += format!(r#"
            impl<'a> Into<{}<'a>> for &'a {}{{
                #[inline(always)]
                fn into(self) -> {}<'a> {{
                {}::{}(self)
            }}
        }}"#,REF_VAL_TYPE,type_name,REF_VAL_TYPE,REF_VAL_TYPE,enum_variant).as_str()
    }


    ret_code+=format!("pub enum {}<'a>{{ {} }}",
                      REF_MUT_VAL_TYPE,
                      VAL_ENUMS.iter().map(|a|{
                          format!("{}(&'a mut {}),",(*a).0,(*a).1)
                      }).reduce(|res,x|{res+&x}).unwrap()
    ).as_str();

    for (enum_variant, type_name) in &VAL_ENUMS{
        ret_code += format!(r#"
            impl<'a> Into<{}<'a>> for &'a mut {}{{
                #[inline(always)]
                fn into(self) -> {}<'a> {{
                {}::{}(self)
            }}
        }}"#,REF_MUT_VAL_TYPE,type_name,REF_MUT_VAL_TYPE,REF_MUT_VAL_TYPE,enum_variant).as_str()
    }

    println!("{}",ret_code);
    ret_code.parse().unwrap()

}}


#[proc_macro_attribute]
pub fn reg_enum_def(_attr:TokenStream,input:TokenStream) -> TokenStream{unsafe{
    let mut enum_item = syn::parse::<ItemEnum>(input.clone()).unwrap();

    REG_ENUM_TYPE = enum_item.ident.to_string();

    for x in enum_item.variants{
        if let (1,Some(field)) = (x.fields.iter().count(),x.fields.iter().last()){
            REG_ENUMS.push((x.ident.to_string(),field.ty.to_token_stream().to_string()))
        }else{
            Err(syn::Error::new(x.fields.span(),"Value enum must have one field")).unwrap()
        }
    }

    let mut impl_code = input.to_string();

    //impl Debug

    println!("{:?}",REG_ENUMS);
    impl_code += format!(r#"
    impl {} {{
        #[inline(always)]
        pub fn unbox_const(&self)->Option<{}<'_>>{{
            match self{{
                {}
            }}
        }}
    "#,
        REG_ENUM_TYPE,
        REF_VAL_TYPE,
        REG_ENUMS.iter().map(|a|{
            format!("Self::{}(a) => Some(a.unbox_const()?.into()),",
                    (*a).0
            )
        }).reduce(|a,n|{a+&n}).unwrap()
    ).as_str();

    impl_code += format!(r#"
        #[inline(always)]
        pub fn unbox_mut(&self)->Option<{}<'_>>{{
            match self{{
                {}
            }}
        }}
    "#,
        REF_MUT_VAL_TYPE,
        REG_ENUMS.iter().map(|a|{
            format!("Self::{}(a) => Some(a.unbox_mut()?.into()),",(*a).0)
        }).reduce(|a,n|{a+&n}).unwrap()
    ).as_str();

    impl_code += "}";

    println!("{}",impl_code);
    impl_code.parse().unwrap()
}}

/// ```rust
/// use macros::match_1_reg;
/// match_1_reg!(a => b,{b.unbox_const()})
/// ```
struct Match1Reg{
    expr:Expr,
    fat_arrow:FatArrow,
    var_name:Ident,
    comma:Comma,
    block:Block,
}
impl Parse for Match1Reg{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        let fat_arrow = input.parse()?;
        let var_name = input.parse()?;
        let comma = input.parse()?;
        let block = input.parse()?;

        Ok(Self{
            expr,
            fat_arrow,
            var_name,
            comma,
            block,
        })
    }
}
#[proc_macro]
pub fn match_1_reg(input:TokenStream)->TokenStream{unsafe{
    let input:Match1Reg = syn::parse(input).unwrap();

    let code = format!(r#"match {}{{
        {}
    }}"#,input.expr.to_token_stream().to_string(),
        REG_ENUMS.iter().map(|(enum_variant,_)|{
            format!("{}::{}({}) => {{ let __variant__ = \"{}\";{} }},", REG_ENUM_TYPE, enum_variant,input.var_name.to_string(),enum_variant,input.block.to_token_stream().to_string())}
        ).reduce(|res,now|{res + &now}).unwrap().as_str()
    );


    code .parse().unwrap()

}}




struct BinaryOpBody{
    tuple:TypeTuple,
    fat_arrow:FatArrow,
    block:Block,
}
impl Parse for BinaryOpBody{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tuple:TypeTuple = input.parse()?;
        let fat_arrow = input.parse()?;
        let block = input.parse()?;
        if tuple.elems.iter().count()!=2{
            Err(syn::Error::new(tuple.span(),"Two value types expected"))?;
        }
        Ok(Self{
            tuple,
            fat_arrow,
            block
        })
    }
}

struct ImplBinaryOp{
    ident:syn::Ident,
    fat_arrow:FatArrow,
    brace: syn::token::Brace,
    bodies:Punctuated<BinaryOpBody,Token![,]>,
}
impl Parse for ImplBinaryOp{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let ident = input.parse()?;
        let fat_arrow = input.parse()?;
        let brace = braced!(content in input);
        let bodies = Punctuated::parse_terminated(&content)?;

        Ok(Self{
            ident,
            fat_arrow,
            brace,
            bodies
        })
    }
}

struct ImplBinaryOps{
    ops:Punctuated<ImplBinaryOp,Token![,]>,
}
impl Parse for ImplBinaryOps{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ops = Punctuated::parse_terminated(input)?;
        Ok(Self{
            ops,
        })
    }
}


/// ```rust
/// impl_binary_op!{
///     OpAdd => {
///         (Integer,Integer) => {
///             Value::Integer(left+right)
///         },
///     }
/// }
/// ```
#[proc_macro]
pub fn impl_binary_ops(input:TokenStream) -> TokenStream{unsafe{
    let input:ImplBinaryOps = syn::parse(input).unwrap();
    let mut code = String::new();
    for y in input.ops {
        code += "#[inline(always)]";
        code += format!("pub fn {}(left:&{},right:&{}) -> {} {{",y.ident.to_string().to_case(Case::Snake),VAL_ENUM_TYPE,VAL_ENUM_TYPE,REG_ENUM_TYPE).as_str();
        code += format!("match (left , right){{").as_str();
        for x in y.bodies{
            let elems = &x.tuple.elems;
            let first = elems.first().unwrap().to_token_stream().to_string();
            let second = elems.last().unwrap().to_token_stream().to_string();

            // find enum variant by variable first
            let left_enum_variant = if first!= "_" {
                (
                    *VAL_ENUMS.iter().find(|(_,type_name)|{
                        type_name == &first
                    }).expect(format!("Type {} is not a Value",first).as_str())
                ).0.clone()
            }else{
                "_".to_string()
            };

            // find enum variant by variable second
            let right_enum_variant = if first!= "_" {
                (
                    *VAL_ENUMS.iter().find(|(_,type_name)|{
                        type_name == &second
                    }).expect(format!("Type {} is not a Value",second).as_str())
                ).0.clone()
            }else{
                "_".to_string()
            };

            code += format!("({}::{},{}::{}) => {},",
                            VAL_ENUM_TYPE ,left_enum_variant ,
                            VAL_ENUM_TYPE ,right_enum_variant ,
                            x.block.to_token_stream().to_string()
            ).as_str();
        }
        code += "}";
        code += "}";
    }
    return code.parse().unwrap();
}}


