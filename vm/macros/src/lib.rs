extern crate core;

use proc_macro::{ TokenStream};
use convert_case::{Case, Casing};
use syn::token::{Colon, Comma, FatArrow};
use syn::{Block, braced, Token, ItemEnum, Expr, Ident, PatTuple, Pat, PatType};
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
        let count = x.fields.iter().count();
        let field = x.fields.iter().last();
        if let (1,Some(field)) = (count,field){
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


struct MatchReg {
    expr:Expr,
    fat_arrow:FatArrow,
    var_name:Ident,
    block:Block,
}
impl Parse for MatchReg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        let fat_arrow = input.parse()?;
        let var_name = input.parse()?;
        let block = input.parse()?;

        Ok(Self{
            expr,
            fat_arrow,
            var_name,
            block,
        })
    }
}
/// ```rust
/// use macros::match_reg;
/// match_reg!(a => b{b.unbox_const()})
/// ```
#[proc_macro]
pub fn match_reg(input:TokenStream) ->TokenStream{unsafe{
    let input: MatchReg = syn::parse(input).unwrap();
    let match_input = input.expr.to_token_stream().to_string();
    let var_name = input.var_name.to_string();
    let body = input.block.to_token_stream().to_string();

    let match_body = REG_ENUMS
        .iter().map(|(enum_variant,_)|{
            format!("{}::{}({}) => {{ let __variant__ = \"{}\";{} }},",
                REG_ENUM_TYPE,
                enum_variant,
                var_name,
                enum_variant,
                body
            )
        }).reduce(|res,now| {
            res + &now
        }).unwrap();

    let code = format!("match {} {{ {} }}",match_input,match_body);

    code .parse().unwrap()
}}

struct OpBody {
    pat:PatTuple,
    fat_arrow:FatArrow,
    expr:Expr,
}
impl OpBody{
    pub fn to_match_branch(&self,op_name:String,mutibility:bool)->String{unsafe{
        let calc_expr = self.expr.to_token_stream().to_string();
        let elems = &self.pat.elems;
        let first = elems.first().unwrap().to_token_stream().to_string();
        let second = elems.last().unwrap().to_token_stream().to_string();

        let left_type = if mutibility {
            REF_MUT_VAL_TYPE.as_str()
        } else {
            REF_VAL_TYPE.as_str()
        };

        let mut left_pat = String::from("left");
        let mut right_pat = String::from("right");

        if first != "_" {
            let variant = &(*VAL_ENUMS
                .iter()
                .find(|(_,type_name)|{
                    type_name == &first
                })
                .expect(format!("Type {} is not a Value",first).as_str())
            ).0;

            left_pat = format!("{}::{}(left)", left_type, variant);
        }

        if second != "_" {
            let variant = &(*VAL_ENUMS
                .iter()
                .find(|(_,type_name)|{
                    type_name == &second
                })
                .expect(format!("Type {} is not a Value",second).as_str())
            ).0;

            right_pat = format!("{}::{}(right)", left_type, variant);
        }

        format!("({},{}) => {},", left_pat, right_pat, calc_expr)
    }}
}

fn parse_pat_tuple(input:&ParseStream) ->syn::Result<PatTuple>{
    match input.parse::<Pat>()?{
        Pat::Tuple(tup) => {
            Ok(tup)
        },
        _ => {
            Err(syn::Error::new(
                input.span(),
                "Must be a tuple pattern!")
            )
        }
    }
}
impl Parse for OpBody {

    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat = parse_pat_tuple(&input)?;
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

struct ImplOp {
    mutbility:Option<syn::token::Mut>,
    ident:syn::Ident,
    fat_arrow:FatArrow,
    brace: syn::token::Brace,
    bodies:Punctuated<OpBody,Token![,]>,
}
impl Parse for ImplOp {
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

struct ImplOps {
    ops:Punctuated<ImplOp,Token![,]>,
}
impl Parse for ImplOps {
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
    let input: ImplOps = syn::parse(input).unwrap();
    let mut code = String::new();

    for y in input.ops {
        let op_name = y.ident.to_string();
        let mutibility = y.mutbility.is_some();

        let left_unbox_fn = match mutibility{
            true =>"unbox_mut",
            false=>"unbox_const"
        };
        let right_unbox_fn = "unbox_const";

        let match_branchs = y.bodies.iter().map(|body|{
            body.to_match_branch(op_name.clone(),mutibility)
        }).reduce(|accu,now|{accu+&now}).unwrap();

        let match_expr = format!("match (left.{}().unwrap() , right.{}().unwrap()){{
            {}
         }}", left_unbox_fn, right_unbox_fn, match_branchs);

        let op_impl_fn = format!("
            #[inline(always)]
            pub fn {}_impl(left:&{},right:&{})->Result<{}> {{
                const __op_name__:&'static str = \"{}\";
                {}
            }}",
            op_name.to_string().to_case(Case::Snake),
            REG_ENUM_TYPE,
            REG_ENUM_TYPE,
            VAL_ENUM_TYPE,
            op_name,
            match_expr
        );

        code += &op_impl_fn;
    }

    println!("{}",code.to_string());

    code.parse().unwrap()
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
    let args_left = args.left.to_token_stream().to_string();
    let args_right = args.right.to_token_stream().to_string();

    let code = format!("{}({},{})", fn_name,args_left,args_right);

    code.parse().unwrap()
}

//
// /// ```rust
// /// use macros::impl_unary_ops;
// /// impl_unary_ops!{
// ///     OpNeg => {
// ///         (Integer) => {
// ///             Value::Integer(~right)
// ///         }
// ///     }
// /// }
// /// ```
//
// #[proc_macro]
// pub fn impl_unary_ops(input:TokenStream) -> TokenStream{
//     let ops:ImplOps = syn::parse(input).unwrap();
//     let code = String::new();
//     for op in ops.ops{
//         let mutability = op.mutbility.is_some();
//         for arm in op.bodies{
//
//         }
//     }
//
// }

struct MatchValueArgument{
    expr:Expr,
    fat_arrow:FatArrow,
    ident:Ident,
    block:Block,
}
impl Parse for MatchValueArgument{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        let fat_arrow = input.parse()?;
        let ident= input.parse()?;
        let block= input.parse()?;

        Ok(Self{
            expr,
            fat_arrow,
            ident,
            block,
        })
    }
}

/// ```rust
/// match_value!(expr => left{
///
/// })
/// ```
#[proc_macro]
pub fn match_value(input:TokenStream)->TokenStream{unsafe{
    let mut code = String::new();

    let MatchValueArgument {
        expr,
        ident,
        block,
        ..
    } = syn::parse(input).unwrap();

    let arg_value = expr.to_token_stream().to_string();
    let arg_var = ident.to_token_stream().to_string();
    let arg_body = block.to_token_stream().to_string();

    let match_body = VAL_ENUMS
        .iter()
        .map(|(now,_)|{
            format!("{}::{}({}) => {},",VAL_ENUM_TYPE,now,arg_var,arg_body)
        }).reduce(|accu,now|{
            accu + &now
        }).unwrap();

    format!("match {} {{ {} }}", arg_value,match_body).parse().unwrap()

}}