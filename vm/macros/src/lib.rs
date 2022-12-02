extern crate core;

use proc_macro::{ TokenStream};
use convert_case::{Case, Casing};
use syn::token::{Comma, FatArrow};
use syn::{Block, braced, Token, ItemEnum, Expr, Ident, PatTuple, Pat};
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
        pub fn unbox_const(&self)->Result<{}<'_>>{{
            match self{{
                {}
            }}
        }}
    "#,
        REG_ENUM_TYPE,
        REF_VAL_TYPE,
        REG_ENUMS.iter().map(|a|{
            format!("Self::{}(a) => Ok(a.unbox_const()?.into()),",
                    (*a).0
            )
        }).reduce(|a,n|{a+&n}).unwrap()
    ).as_str();

    impl_code += format!(r#"
        #[inline(always)]
        pub fn unbox_mut(&self)->Result<{}<'_>>{{
            match self{{
                {}
            }}
        }}
    "#,
        REF_MUT_VAL_TYPE,
        REG_ENUMS.iter().map(|a|{
            format!("Self::{}(a) => Ok(a.unbox_mut()?.into()),",(*a).0)
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
            format!("{}::{}({}) => {{ let __variant__ = \"{}\";{} }},",
                    REG_ENUM_TYPE,
                    enum_variant,
                    input.var_name.to_string(),
                    enum_variant,
                    input.block.to_token_stream().to_string()
            )
        }).reduce(|res,now|{res + &now}).unwrap().as_str()
    );


    code .parse().unwrap()

}}

struct BinaryOpBody{
    pat:PatTuple,
    fat_arrow:FatArrow,
    expr:Expr,
}
impl Parse for BinaryOpBody{
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let pat = if let Pat::Tuple(tuple) = input.parse()?{
            if tuple.elems.iter().count()!=2{
                Err(syn::Error::new(
                    tuple.span(),
                    "Must be a two-element tuple pattern such as (LeftType,RightType)!"
                ))
            }else{
                Ok(tuple)
            }
        }else{
            Err(syn::Error::new(
                input.span(),
                "Must be a two-element tuple pattern such as (LeftType,RightType)!"
            ))
        }?;

        let fat_arrow = input.parse()?;
        let block = input.parse()?;
        if pat.elems.iter().count()!=2{
            Err(syn::Error::new(pat.span(),"Two Value types expected"))?;
        }
        Ok(Self{
            pat,
            fat_arrow,
            expr: block
        })
    }
}

struct ImplBinaryOp{
    mutbility:Option<syn::token::Mut>,
    ident:syn::Ident,
    fat_arrow:FatArrow,
    brace: syn::token::Brace,
    bodies:Punctuated<BinaryOpBody,Token![,]>,
}
impl Parse for ImplBinaryOp{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let mutbility = input.parse()?;
        let ident = input.parse()?;
        let fat_arrow = input.parse()?;
        let brace = braced!(content in input);
        let bodies = Punctuated::parse_terminated(&content)?;

        Ok(Self{
            mutbility,
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
///         (_,_) => {
///             Value::Nil(Nil())
///         }
///     },
/// }
/// ```
#[proc_macro]
pub fn impl_binary_ops(input:TokenStream) -> TokenStream{unsafe{
    let input:ImplBinaryOps = syn::parse(input).unwrap();
    let mut code = String::new();
    let mut mutibility = false;
    for y in input.ops {
        mutibility = y.mutbility.is_some();
        let (ref_val_type,unbox_fn_name) = if mutibility {
            (REF_MUT_VAL_TYPE.as_str(),"unbox_mut")
        }else{
            (REF_VAL_TYPE.as_str(),"unbox_const")
        };

        code += "#[inline(always)]";
        code += format!("pub fn {}_impl(left:&{},right:&{}) -> Result<{}> {{",
                        y.ident.to_string().to_case(Case::Snake),
                        REG_ENUM_TYPE,
                        REG_ENUM_TYPE,
                        VAL_ENUM_TYPE
        ).as_str();

        code += format!("match (left.{}().unwrap() , right.unbox_const().unwrap()){{",
                        unbox_fn_name
        ).as_str();

        for x in y.bodies{
            let elems = &x.pat.elems;
            let first = elems.first().unwrap().to_token_stream().to_string();
            let second = elems.last().unwrap().to_token_stream().to_string();

            // find enum variant by variable first
            let left_pat = if first!= "_" {
                format!("{}::{}(left)",ref_val_type, (
                        *VAL_ENUMS.iter().find(|(_,type_name)|{
                            type_name == &first
                        }).expect(format!("Type {} is not a Value",first).as_str())
                ).0)
            }else{
                "left".to_string()
            };

            // find enum variant by variable second
            let right_pat = if first!= "_" {
                format!("{}::{}(right)",REF_VAL_TYPE,(
                    *VAL_ENUMS.iter().find(|(_,type_name)|{
                        type_name == &second
                    }).expect(format!("Type {} is not a Value",second).as_str())
                ).0)
            }else{
                "right".to_string()
            };

            code += format!("({},{}) => {{ const __op_name__:&'static str = \"{}\"; {} }},",
                            left_pat, right_pat, y.ident.to_string(),
                            x.expr.to_token_stream().to_string()).as_str();
        }
        code += "}";
        code += "}";
    }

    println!("{}",code.to_string());

    return code.parse().unwrap();
}}



struct CallOpArg{
    left:Expr,
    comma1:Comma,
    op:Ident,
    comma2:Comma,
    right:Expr,
}
impl Parse for CallOpArg{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let left = input.parse()?;
        let comma1 = input.parse()?;
        let op = input.parse()?;
        let comma2 = input.parse()?;
        let right = input.parse()?;

        Ok(Self{
            left,
            comma1,
            op,
            right,
            comma2
        })
    }
}

/// ```rust
/// call_op!(left,OpAdd,right);
/// ```
#[proc_macro]
pub fn call_binary_op(input:TokenStream) -> TokenStream{
    let args:CallOpArg = syn::parse(input).unwrap();
    let fn_name = args.op.to_string().to_case(Case::Snake)+"_impl";
    let code = format!("{}({},{})",fn_name,args.left.to_token_stream().to_string(),args.right.to_token_stream().to_string());
    code.parse().unwrap()
}
