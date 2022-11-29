use proc_macro::{TokenStream};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};
use syn::parse::{Parse};
use syn::parse::ParseStream;
use syn::{Expr, ItemEnum, PathArguments};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::__private::ToTokens;
use convert_case::{Case, Casing};
use syn::spanned::Spanned;

//Option<Vec<(ValueTag,ValueType)>>
static mut VALUE_TAGS   :Option<Vec<(String,String)>>= None;

//BTreeMap<OpName,Vec<ValueTypes>>
static mut BINARY_OPS   :Option<BTreeMap<String,BTreeSet<String>>> = None;

//BTreeMap<OpName,Vec<ValueTypes>>
static mut UNARY_OPS    :Option<BTreeMap<String,BTreeSet<String>>> = None;

/// ```rust
/// #[define_val]
/// pub enum Value{
///     Integer(Integer),
///     Float(Float),
///     Bool(Bool),
/// }
/// ```

#[proc_macro_attribute]
pub fn def_val(attr:TokenStream,item:TokenStream) -> TokenStream{
    struct ValueDefine{
        enums:Vec<(String,String)>,
    }
    impl Parse for ValueDefine{
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let enums = Vec::new();
            let parse:ItemEnum = input.parse()?;
            // check defines
            for x in parse.variants{
                //check if the Enum value just has one field
                if x.fields.iter().count()!=1
                enums.push((
                    x.ident.to_string(),
                    x.fields.iter().last().unwrap_or_else(||{syn::Error::new(x.fields.span(),"f")})
                    ));
            }
        }
    }
}
/// ```rust
/// #[define_binary_op(OpAdd,op_add)]
/// pub trait OpAdd<T>{
///     fn op_add(&self)
/// }
/// ```
#[proc_macro_attribute]
pub fn def_binary_op(attr:TokenStream,item:TokenStream) -> TokenStream{

}

#[proc_macro_attribute]
pub fn def_unary_op(attr:TokenStream,item:TokenStream) -> TokenStream{

}

#[proc_macro_attribute]
pub fn impl_op(attr:TokenStream,item:TokenStream) -> TokenStream{

}


macro_rules! def_binary_op_trait {
    ($trait_name:ident,$fn_name:ident->$ret_type:ident) => {paste!{
        #[def_binary_op($trait_name,$fn_name)]
        pub trait $trait_name<T> {
            fn $fn_name(&self,_other:&T) -> $ret_type;
        }
        #[inline(always)]
        pub fn $fn_name(a:&Value,b:&Value) -> Value {
            return match_2_values!((a,b),{a.unbox_const().$fn_name(b.unbox_const())});
        }
    }};
}

macro_rules! def_unary_op_trait {
    ($trait_name:ident,$fn_name:ident,ReturnType) => {paste!{
        #[op_define]
        pub trait $trait_name {
            #[inline(always)]
            fn $fn_name(&self)->Value{
                unimplemented!()
            }
        }
        #[inline(always)]
        pub fn $fn_name<T:$trait_name>(a:&Value) -> Value{
            return match_1_value!(a,{a.unbox_const().$fn_name()});
        }
    }}
}

/// ```rust
/// #[impl_op_default]
/// impl OpAdd<Integer> for Bool{
///     fn op_add(&self,other:Integer) -> Value{
///
///     }
/// }
/// ```
#[proc_macro]
pub fn impl_default(attr:TokenStream,item:TokenStream) -> TokenStream{

}

/// ```rust
/// match_2_values!(ValueType,(left,right),{
///     a.unbox().op_add(right)
/// })
/// ```

#[proc_macro]
fn match_2_values(input:TokenStream)->TokenStream{

}


#[proc_macro]
fn match_1_value(input:TokenStream) -> TokenStream{

}

