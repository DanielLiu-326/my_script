use proc_macro::{TokenStream};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};
use syn::parse::{Parse};
use syn::parse::ParseStream;
use syn::{Expr, PathArguments};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::__private::ToTokens;
use convert_case::{Case, Casing};
/// Value Type
///     Bool
///     Integer
///     Float
///     Inline String

/// Register Type
///     InlineMutRef
///     InlineConstRef
///     RefCount
///
///
///


// the value name
static mut VALUE_NAME   :Option<String> = None;
// all tokens in enum value;
static mut VALUE_TOKENS :Option<Vec<String>> = None;
//(type,implemented op traits)
static mut VALUE_TYPES  :Option<BTreeSet<String>> = None;
//<op,(left,right)>
static mut BINARY_OPS   :Option<BTreeMap<String,BTreeSet<(String, String)>>> = None;
//<op,left>
static mut UNARY_OPS    :Option<BTreeMap<String,BTreeSet<String>>> = None;

#[proc_macro_attribute]
pub fn define_val(_attr: TokenStream, item: TokenStream) -> TokenStream{unsafe{

    let item_enum = syn::parse::<syn::ItemEnum>(item.clone()).unwrap();

    VALUE_NAME = Some(item_enum.ident.to_string());

    VALUE_TOKENS = Some(
        item_enum.variants.iter().map(|a| {
            a.ident.to_string()
        }).collect()
    );

    VALUE_TYPES = Some(item_enum.variants.iter().map(|a|{
        a.fields.iter().next().unwrap().ty.to_token_stream().to_string()
    }).collect());

    item
}}
#[proc_macro_attribute]
pub fn op_define(_attr: TokenStream, item: TokenStream) -> TokenStream {unsafe {

    let trait_define = syn::parse::<syn::ItemTrait>(item.clone()).unwrap();

    let op_name = trait_define.ident.to_string();

    if trait_define.generics.params.is_empty(){
        //unary op
        UNARY_OPS = UNARY_OPS.take().map_or(Some({
            let mut ret:BTreeMap<String,BTreeSet<String>> = Default::default();
            ret.insert(op_name.clone(),Default::default());
            ret
        }), |mut ops|{
            ops.insert(op_name,BTreeSet::default());
            Some(ops)
        });
    }else{
        //binary op
        BINARY_OPS = BINARY_OPS.take().map_or(Some({
            let mut ret:BTreeMap<String,BTreeSet<(String,String)>> = Default::default();
            ret.insert(op_name.clone(),Default::default());
            ret
        }), |mut ops|{
            ops.insert(op_name,BTreeSet::default());
            Some(ops)
        });
    }

    item
}}

///#[impl_op(binary)] or #[impl_op(unary)]
#[proc_macro_attribute]
pub fn impl_op(_attr: TokenStream, item: TokenStream) -> TokenStream {unsafe{

    //impl OpAdd for Integer
    let impl_block = syn::parse::<syn::ItemImpl>(item.clone()).unwrap();

    let left = (*impl_block.self_ty).clone().to_token_stream().to_string();

    let trait_name = impl_block.trait_.as_ref().unwrap().1.segments
        .last().unwrap().ident
        .to_token_stream().to_string();

    let right = &impl_block.trait_.as_ref().unwrap().1.segments.last().unwrap().arguments;

    if right.is_empty(){

        //left operator
        UNARY_OPS.as_mut().unwrap()
            .get_mut(&trait_name).unwrap()
            .insert(left);
    }else{
        //unary operator
        let right = match right{
            PathArguments::AngleBracketed(ref a) =>{Some(a)},
            _ =>{None},
        }.unwrap().args.last().unwrap().to_token_stream().to_string();

        BINARY_OPS.as_mut().unwrap()
            .get_mut(&trait_name).unwrap()
            .insert((left, right));
    }

    item
}}


#[proc_macro]
pub fn impl_default(items: TokenStream) -> TokenStream {unsafe{
    // impl_default!{
    //      Add => {
    //          ...
    //     }
    // }
    println!("impl_default");
    #[derive(Clone)]
    pub struct ImplDefault{
        trait_name:String,
        block:syn::Block,
    }
    impl syn::parse::Parse for ImplDefault{
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let trait_name = input.parse::<syn::Ident>()?.to_string();
            input.parse::<syn::token::FatArrow>()?;
            let block = input.parse()?;
            Ok(Self{
                trait_name,
                block,
            })
        }
    }

    pub struct ImplDefaults(Vec<ImplDefault>);
    impl syn::parse::Parse for ImplDefaults{
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let impl_defaults = Punctuated::<ImplDefault,Comma>::parse_separated_nonempty(input)?;
            Ok(Self(
                impl_defaults.iter().map(|x|{x.clone()}).collect()
            ))
        }
    }

    let mut ret = String::new();
    let impl_defaults = syn::parse::<ImplDefaults>(items).unwrap();

    for impl_default in impl_defaults.0.iter() {
        for left in VALUE_TYPES.as_ref().unwrap().iter() {
            if UNARY_OPS.as_ref().unwrap().contains_key(&impl_default.trait_name){
                //iterate right operatee value
                if UNARY_OPS.as_ref().unwrap().get(&impl_default.trait_name).unwrap()
                    .get(&left.clone()).is_none() {
                    ret += format!("impl {} for {}{{", impl_default.trait_name, left).as_str();
                    ret += format!("fn {}(&self) -> Value{}",
                                   impl_default.trait_name.to_case(Case::Snake),
                                   impl_default.block.to_token_stream().to_string(),
                    ).as_str();
                    ret += "}";
                }
            }else if BINARY_OPS.as_ref().unwrap().contains_key(&impl_default.trait_name) {
                //iterate left operatee value tag
                for right in VALUE_TYPES.as_ref().unwrap().iter() {
                    //iterate right operatee value tag
                    if BINARY_OPS.as_ref().unwrap().get(&impl_default.trait_name).unwrap()
                        .get(&(left.clone(), right.clone())).is_none() {
                        ret += format!("impl {}<{}> for {}{{", impl_default.trait_name, right, left).as_str();
                        ret += format!("fn {}(&self,other:&{}) -> Value{}",
                                       impl_default.trait_name.to_case(Case::Snake),
                                       right,
                                       impl_default.block.to_token_stream().to_string(),
                        ).as_str();
                        ret += "}";
                    }
                }
            }
        }

    }
    return ret.parse().unwrap();
}}

struct MatchArgument {
    expr:String,
    block:String,
}
impl Parse for MatchArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<Expr>()?;
        input.parse::<Comma>()?;
        let block = input.parse::<syn::Block>()?;
        Ok(Self{
            expr: expr.to_token_stream().to_string(),
            block: block.to_token_stream().to_string(),
        })
    }
}

/// match_1_value!(a,{
///     ...
/// })
#[proc_macro]
pub fn match_1_value(item:TokenStream)->TokenStream{unsafe{

    let arguments = syn::parse::<MatchArgument>(item).unwrap();

    let mut ret = String::new();
    ret += format!("match {} {{",arguments.expr).as_str();
    for x in VALUE_TOKENS.as_ref().unwrap(){
        ret += format!("Value::{}(a) => {},",x,arguments.block).as_str()
    }
    ret+="}";

    return ret.parse().unwrap();
}}

/// match_2_value!((a,b),{
///     ...
/// })
#[proc_macro]
pub fn match_2_values(item:TokenStream) ->TokenStream{unsafe{

    let arguments = syn::parse::<MatchArgument>(item).unwrap();

    let mut ret = String::new();
    ret += format!("match {} {{",arguments.expr).as_str();
    for x in VALUE_TOKENS.as_ref().unwrap(){
        for y in VALUE_TOKENS.as_ref().unwrap(){
            ret += format!("(Value::{}(a),Value::{}(b)) => {},",x,y,arguments.block).as_str()
        }
    }
    ret+="}";

    return ret.parse().unwrap();
}}
