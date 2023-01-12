use macros::mux;
use concept::types::*;
use concept::ast::*;
use crate::util::UncheckMut;
use super::errors::*;

// Register Type Definitions
pub trait RegisterType<T>:{
    fn load_variable(val:T) -> Self;

    fn unbox_const(&self) -> RefConstValue;

    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue>{
        return Err(MutabilityError::new().into())
    }
}


// Definitions Of Register Types
pub struct Inline<const MUTABLE:bool,T>(UncheckMut<T>);

impl<const MUTABLE:bool,T> Inline<MUTABLE,T> {
    pub fn new(val:T)->Self{
        Self(UncheckMut::new(val))
    }
}

impl<const MUTABLE:bool,T> RegisterType<T> for Inline<MUTABLE,T>{
    #[inline(always)]
    fn load_variable(val: T) -> Self {
        Self(UncheckMut::new(val))
    }

    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue> {
        if MUTABLE{
            Ok(self.0.get_mut().into())
        }else{
            Err(MutabilityError::new().into())
        }
    }
}
pub type RefNil       <const MUTABLE:bool> = Inline<MUTABLE,Nil>;
pub type InlineBool   <const MUTABLE:bool> = Inline<MUTABLE,Bool>;
pub type InlineInteger<const MUTABLE:bool> = Inline<MUTABLE,Integer>;
pub type InlineFloat  <const MUTABLE:bool> = Inline<MUTABLE,Float>;


#[mux]
pub enum RegType{
    InlineInteger(InlineInteger<true>),
    InlineFloat(InlineFloat<true>),
    InlineBool(InlineBool<true>),

    ConstInlineInteger(InlineInteger<false>),
    ConstInlineFloat(InlineFloat<false>),
    ConstInlineBool(InlineBool<false>),

    RefNil(RefNil<true>),
    ConstRefNil(RefNil<false>),
}

impl Default for RegType{
    fn default() -> Self {
        Self::RefNil(RefNil::new(Nil::new()))
    }
}

impl RegType{
    #[inline(always)]
    pub fn unbox_const(&self) -> RefConstValue{
        reg_type_match!(self => reg,{
            reg.unbox_const().into()
        })
    }
    #[inline(always)]
    pub fn unbox_mut(&self) -> Result<RefMutValue>{
        reg_type_match!(self => reg,{
            Ok(reg.unbox_mut()?.into())
        })
    }
}

pub(crate) macro call_bin($op_name:literal,$left:expr,$right:expr) {
    ref_const_value_match!(($left).unbox_const() => left,{
        BinaryOp::<$op_name>::op_call(left,$right)
    })
}

pub(crate) macro call_unary($op_name:literal,$value:expr) {
    ref_const_value_match!(($value).unbox_const() => value,{
        UnaryOp::<$op_name>::op_call(value)
    })
}