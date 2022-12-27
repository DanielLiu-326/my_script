use std::collections::hash_map::Values;
use macros::mux;
use paste::paste;
use crate::errors::*;
use crate::mem_collection::RefConst;
use crate::util::UncheckMut;

pub mod integer;
pub mod float;
pub mod bool;
pub mod nil;

pub use integer::*;
pub use float::*;
pub use self::bool::*;
pub use nil::*;

// ***************** operation defines *****************

pub trait BinaryOp<const op_name:&'static str>{
    #[inline(always)]
    fn op_call(&self,other:&RegType) ->Result<Value>{
        Err(UnsupportedOp::new(op_name).into())
    }
}

pub trait BinaryMutOp<const op_name:&'static str>{
    #[inline(always)]
    fn op_call(&mut self,other:&RegType) ->Result<Value>{
        Err(UnsupportedOp::new(op_name).into())
    }
}

pub trait UnaryOp<const op_name:&'static str>{
    #[inline(always)]
    fn op_call(&self) ->Result<Value>{
        Err(UnsupportedOp::new(op_name).into())
    }
}

// ***************** Value defines *****************

pub trait Val:
    BinaryOp<"op_or">+
    BinaryOp<"op_and">+
    BinaryOp<"op_bit_or">+
    BinaryOp<"op_bit_xor">+
    BinaryOp<"op_bit_and">+
    BinaryOp<"op_ne">+
    BinaryOp<"op_eq">+
    BinaryOp<"op_lt">+
    BinaryOp<"op_gt">+
    BinaryOp<"op_le">+
    BinaryOp<"op_ge">+
    BinaryOp<"op_l_mov">+
    BinaryOp<"op_r_mov">+
    BinaryOp<"op_add">+
    BinaryOp<"op_sub">+
    BinaryOp<"op_mul">+
    BinaryOp<"op_div">+
    BinaryOp<"op_mod">+
    BinaryOp<"op_fact">+
    BinaryMutOp<"op_assign">+
    UnaryOp<"op_bit_not">+
    UnaryOp<"op_not">+
    UnaryOp<"op_neg">+
    UnaryOp<"op_pos">
{
    fn load_variable(&self,mutable:bool) -> RegType;
}

#[mux]
#[derive(Debug)]
pub enum Value{
    Integer (Integer),
    Float   (Float),
    Bool    (Bool),
    Nil     (Nil),
}

#[mux]
#[derive(Debug)]
pub enum RefConstValue<'a>{
    Integer (&'a Integer),
    Float   (&'a Float),
    Bool    (&'a Bool),
    Nil     (&'a Nil),
}

#[mux]
#[derive(Debug)]
pub enum RefMutValue<'a>{
    Integer (&'a mut Integer),
    Float   (&'a mut Float),
    Bool    (&'a mut Bool),
    Nil     (&'a mut Nil),
}


impl Value{
    #[inline(always)]
    pub fn load_variable(&self,mutable:bool) -> RegType{
        value_match!(self,val,{
            val.load_variable(mutable)
        })
    }
}

impl<'a> RefConstValue<'a>{
    #[inline(always)]
    pub fn try_into_bool(&self)->Result<&bool>{
        match self{
            RefConstValue::Bool(ret) => {
                Ok(ret)
            },
            _ => {
                Err(TypeError::new("Bool","...").into())
            },
        }
    }
}


// ***************** RegType defines *****************
pub trait RegTy{
    #[inline(always)]
    fn unbox_mut(&self)->Result<RefMutValue>{
        Err(MutabilityError::new().into())
    }

    fn unbox_const(&self)->RefConstValue;

    fn clone_ref(&self) -> RegType{
        todo!()
    }
}

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
        Self::RefNil(RefNil::new())
    }
}

impl RegType{
    #[inline(always)]
    pub fn unbox_const(&self) -> RefConstValue{
        reg_type_match!(self,reg,{
            reg.unbox_const().into()
        })
    }
    #[inline(always)]
    pub fn unbox_mut(&self) -> Result<RefMutValue>{
        reg_type_match!(self,reg,{
            Ok(reg.unbox_mut()?.into())
        })
    }
}

#[macro_export]
macro_rules! call_op {
    ($op_name:literal,$left:expr,$right:expr) => {
        match_ref_const_val!(($left).unbox_const(),left,{
            BinaryOp::<$op_name>::op_call(left,$right)
        })
    };

    ($op_name:literal,$val:expr) => {
        match_ref_const_val!($val.unbox_const(),val,{
            UnaryOp::<$op_name>::op_call(val)
        })
    };
}

#[macro_export]
macro_rules! call_mut_op {
    ($op_name:literal,$left:expr,$right:expr) => {
        match_ref_mut_val!(($left).unbox_mut()?,left,{
            BinaryMutOp::<$op_name>::op_call(left,$right)
        })
    };

    ($op_name:literal,$val:expr) => {
        match_ref_mut_val!(($val).unbox_mut()?,val,{
            UnaryMutOp::<$op_name>::op_call(val,$right)
        })
    };
}
#[inline(always)]
pub fn call_op<const op_name:&'static str>(){

}