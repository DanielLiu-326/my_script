extern crate core;

use proc_macro::{TokenStream};
use syn::token::FatArrow;
use syn::{Block, braced, Token, TypeTuple};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

static mut VAL_ENUM_TYPE : String = String::new();

static mut VAL_ENUMS     : Vec<(String,String)> = Vec::new();

static mut REG_ENUM_TYPE : String = String::new();

static mut REG_ENUMS     : Vec<(String,String)> = Vec::new();

static mut REF_VAL_TYPE  :String = String::new();

static mut REF_MUT_VAL_TYPE:String = String::new();

//unbox the reference into the type that can be operatee.

#[proc_macro_attribute]
pub fn val_enum_def(_attr:TokenStream,input:TokenStream) -> TokenStream{unsafe{
    let mut enum_item = syn::parse::<ItemEnum>(input).unwrap();

    VAL_ENUM_TYPE = enum_item.ident.to_string();

    for x in enum_item.variants{
        if let (1,Some(field)) = (x.fields.iter().count(),x.fields.iter().last()){
            VAL_ENUMS.push(field.ty.to_token_stream().to_string())
        }else{
            Err(syn::Error::new(x.fields.span(),"Value enum must have one field")).unwrap()
        }
    }

    REF_VAL_TYPE = "Ref".to_string() + VAL_ENUM_TYPE;
    REF_MUT_VAL_TYPE = "RefMut".to_string() + VAL_ENUM_TYPE;

    let mut ret_code = input.to_string();

    ret_code+=format!("pub enum {}<'a>{{ {} }}",
                      REF_VAL_TYPE,
                      VAL_ENUMS.iter().map(|a|{
                          format!("{}(&'a {}),",(*a).0,(*a).1)
                      }).count()
    );

    ret_code+=format!("pub enum {}<'a>{{ {} }}",
                      REF_MUT_VAL_TYPE,
                      VAL_ENUMS.iter().map(|a|{
                          format!("{}(&'a mut {}),",(*a).0,(*a).1)
                      }).reduce(|res,x|{res+=x}).unwrap()
    );

    ret_code.parse().unwrap()

}}

#[proc_macro_attribute]
pub fn reg_enum_def(_attr:TokenStream,input:TokenStream) -> TokenStream{unsafe{
    let mut enum_item = syn::parse::<ItemEnum>(input).unwrap();

    REG_ENUM_TYPE = enum_item.ident.to_string();

    for x in enum_item.variants{
        if let (1,Some(field)) = (x.fields.iter().count(),x.fields.iter().last()){
            REG_ENUMS.push(field.ty.to_token_stream().to_string())
        }else{
            Err(syn::Error::new(x.fields.span(),"Value enum must have one field")).unwrap()
        }
    }

    let mut impl_code = format!(r#"
    impl {} {{
        pub fn unbox_const(&self)->{}<'_>{{
            match self{{
                {}
            }}
        }}
    "#,
        REG_ENUM_TYPE,
        REF_VAL_TYPE,
        VAL_ENUMS.iter().map(|a|{
            format!("Self::{}(a) => {}::{}(a.unbox_const()),",(*a).0,REF_VAL_TYPE,(*a).0)
        }).reduce(|a,n|{a+=n}).unwrap()
    );

    impl_code += format!(r#"
        pub fn unbox_const(&self)->{}<'_>{{
            match self{{
                {}
            }}
        }}
    "#,
        REF_MUT_VAL_TYPE,
        VAL_ENUMS.iter().map(|a|{
            format!("Self::{}(a) => {}::{}(a.unbox_mut()),",(*a).0,REF_MUT_VAL_TYPE,(*a).0)
        }).reduce(|a,n|{a+=n}).unwrap()
    );

    impl_code += "}}";

    impl_code.parse().unwrap()
}}

// impl_binary_ops!{
//     Integer OpAdd Integer => {
//         Value::Integer(left + right)
//     }
//
//     _ OpAdd _ =>{
//
//     }
// }
///
/// impl_binary_op!{
///     OpAdd => {
///         (Integer,Integer) => {
///             Value::Integer(left+right)
///         },
///     }
/// }


struct BinaryOpBody{
    tuple:TypeTuple,
    fat_arrow:FatArrow,
    block:Block,
}
impl Parse for BinaryOpBody{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tuple = input.parse()?;
        let fat_arrow = input.parse()?;
        let block = input.parse()?;
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

#[proc_macro]
pub fn impl_binary_ops(input:TokenStream) -> TokenStream{
    let input:ImplBinaryOps = syn::parse(input).unwrap();
    for x in input.ops{

    }
}
//
// static mut VAL_COMM_TYPE:Option<String> = None;
//
// //Option<Vec<(ValueTag,ValueType)>>
// static mut VAL_ENUMS    :Option<Vec<(String, String)>>= None;
//
//
// static mut REG_COMM_TYPE:Option<String> = None;
//
// static mut REG_ENUMS    :Option<Vec<(String,String)>> = None;
//
// //BTreeMap<OpName,Vec<ValueTypes>>
// static mut BINARY_OPS   :Option<BTreeMap<String,(String,BTreeSet<String>)>> = None;
//
// //BTreeMap<OpName,Vec<ValueTypes>>
// static mut UNARY_OPS    :Option<BTreeMap<String,BTreeSet<String>>> = None;
//
// pub trait RegTy{
//     type Output;
//     //unbox the reference into the type that can be operatee.
//     fn unbox_const(&self) -> Option<&Self::Output>{
//         None
//     }
//     fn unbox_mut(&self) -> Option<&mut Self::Output>{
//         None
//     }
// }
//
// //pub enum RefValue
// //pub enum RefMutValue
//
// #[proc_macro]
// pub fn val_def(input:TokenStream) -> TokenStream{unsafe{
//     let enum_def:ItemEnum = syn::parse(input);
//     let value_enums = Vec::new();
//     let mut code = String::new();
//
//     VAL_COMM_TYPE = Some(enum_def.ident.to_string());
//
//     for x in enum_def.variants{
//         if let (1,Some(field)) = (x.fields.iter().count(),x.fields.iter().last()){
//             value_enums.push((x.ident.to_string(),field.to_token_stream().to_string()));
//         }else{
//             Err(syn::Error::new(
//                 x.fields.span(),
//                 "Value enum must have only one field in each variant!")
//             ).unwrap();
//         }
//     }
//
//     VAL_ENUMS = Some(value_enums.clone());
//
//     let mut variants_code = String::new();
//     for x in value_enums{
//         variants_code += format!("{}(&'a {}),", x.0, x.1);
//     }
//
//     code += format!(r#"pub enum Ref{}<'a>{{
//         {}
//     }}"#, VAL_COMM_TYPE.unwrap(), variants_code.as_str());
//
//     let mut variants_code = String::new();
//     for x in value_enums{
//         variants_code += format!("{}(&'a mut {}),", x.0, x.1);
//     }
//
//     code += format!(r#"pub enum RefMut{}<'a>{{
//         {}
//     }}"#, VAL_COMM_TYPE.unwrap(), variants_code.as_str());
//
//     code.parse().unwrap()
// }}
//
// #[proc_macro_attribute]
// pub fn reg_common_def(input:TokenStream) -> TokenStream{unsafe{
//     let enum_def:ItemEnum = syn::parse(input);
//     let reg_enums = Vec::new();
//
//     REG_COMM_TYPE = Some(enum_def.ident.to_string());
//
//     for x in enum_def.variants{
//         if let (1,Some(field)) = (x.fields.iter().count(),x.fields.iter().last()){
//             reg_enums.push((x.ident.to_string(), field.to_token_stream().to_string()));
//         }else{
//             Err(syn::Error::new(
//                 x.fields.span(),
//                 "Reg enum must have only one field in each variant!")
//             ).unwrap();
//         }
//     }
//
//     REG_ENUMS = Some(reg_enums.clone());
//
//     let mut get_ref_val = format!("pub fn get_ref_val(a:&{}) -> Option<{}>{{",enum_def.ident.to_string(),);
//     get_ref_val+="match a {RegType::Integer(a)=>{RefValue::Integer(a.unbox_const())}}";
//     for x in reg_enums{
//
//     }
//     get_ref_val += "}";
//
//
// }}
//
//
//
// impl_binary_ops!{
//     Integer OpAdd Integer => {
//         Value::Integer(left + right)
//     }
//
//     _ OpAdd _ =>{
//
//     }
// }
//
// // |
// // |
// // V
// /// fn op_add_impl(left:&RegVal,right:&RegVal) -> Value{
// ///     match(left.unbox_const_ref(),right.unbox_const_ref()){
// ///         (RefConstValue::Integer(left),RefConstValue::Integer(right)) =>{
// ///             Value::Integer(left + right)
// ///         }
// ///         _ => {
// ///
// ///         }
// ///     }
// /// }
// ///
// fn op_add(left:&RegCommon,Right:&RegCommon)->Value{
//     call_op!(left.unbox_const(),OpAdd,right.unbox_const());
// }
//
// call_op!(left,OpAdd,right);
//
// ///
// /// match(left,right){
// ///     (Value::Integer(left),Value::Integer(right)){
// ///         left
// ///     }
// /// }
// ///
