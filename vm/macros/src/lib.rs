use proc_macro::{TokenStream};
use std::collections::{BTreeMap, BTreeSet};
use std::mem::Discriminant;
use std::ops::{Deref, DerefMut};
use syn::parse::{Parse};
use syn::parse::ParseStream;
use syn::{Expr, FnArg, ItemEnum, ItemTrait, PathArguments, TraitItem};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::__private::ToTokens;
use convert_case::{Case, Casing, Converter};
use syn::spanned::Spanned;

static mut VALUE_TYPE   :Option<String> = None;

//Option<Vec<(ValueTag,ValueType)>>
static mut VALUE_TAGS   :Option<Vec<(String,String)>>= None;

//BTreeMap<OpName,Vec<ValueTypes>>
static mut BINARY_OPS   :Option<BTreeMap<String,(String,BTreeSet<String>)>> = None;

//BTreeMap<OpName,Vec<ValueTypes>>
static mut UNARY_OPS    :Option<BTreeMap<String,BTreeSet<String>>> = None;

struct ValueDefine{
    type_name:String,
    enums:Vec<(String,String)>,
}
impl Parse for ValueDefine{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut enums = Vec::new();
        let parsed:ItemEnum = input.parse()?;
        // check defines
        for x in parsed.variants{
            //check if the Enum value just has one field
            if x.fields.iter().count()!=1{
                return Err(syn::Error::new(x.fields.span(),"Value mast have one field"));
            }
            //insert into values
            enums.push((
                x.ident.to_string(),
                x.fields.iter().last().unwrap().to_token_stream().to_string(),
            ));
        }

        Self{
            type_name:parsed.ident.to_string(),
            enums,
        }
    }
}

pub struct OpInfo{
    fn_name:String,
    self_mut:bool,
    other_mut:bool,
    implemented:BTreeMap<String>,
}
impl Parse for OpInfo{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_trait  :ItemTrait = syn::parse(input)?;
        let trait_name = item_trait.ident.to_string();
        let fn_name = trait_name.to_case(Case::Snake);

        item_trait.items.iter().find(|&&x| if let TraitItem::Method(fn_def) = x{
            fn_def.sig.ident.to_string() == fn_name
        }).map_or(Err(syn::Error::new(item_trait.span(),format!("cant find function {}"))),);

    }
}

/// ```rust
/// #[define_val]
/// pub enum Value{
///     Integer(Integer),
///     Float(Float),
///     Bool(Bool),
/// }
/// ```
#[proc_macro_attribute]
pub fn def_val(attr:TokenStream,item:TokenStream) -> TokenStream{unsafe{
    let ValueDefine{type_name,enums } = syn::parse(item).unwrap();

    VALUE_TYPE = Some(type_name);
    VALUE_TAGS = Some(enums);

    item
}}


/// ```rust
/// #[define_binary_op(op_add)]
/// pub trait OpAdd<T>{
///     fn op_add(&self,other:&T) -> Value;
/// }
/// ```
#[proc_macro_attribute]
pub fn def_binary_op(attr:TokenStream,item:TokenStream) -> TokenStream{unsafe{
    if attrs.count() != 1{
        panic!("def_binary_op attribute must have one attribute");
    }

    let attrs       :Punctuated<syn::Ident,Token![,]> = syn::parse(attr).unwrap();
    let item_trait  :ItemTrait = syn::parse(item).unwrap();
    let trait_name = item_trait.ident.to_string();
    let fn_name =attrs.first().unwrap();

    item_trait.items.iter().find(|&&x| if let TraitItem::Method(a) = x{
        a.sig.inputs.iter().count() == 2 &&
        if let syn::FnArg::Receiver(a_) = a.sig.inputs.first().unwrap(){
            true
        }else{
            false
        } &&

    });

    if BINARY_OPS.is_none(){
        BINARY_OPS = Some(Default::default());
    }

    BINARY_OPS.unwrap().insert(trait_name,(fn_name,Default::default()));
}}

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

