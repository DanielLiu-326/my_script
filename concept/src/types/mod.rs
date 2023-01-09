pub mod integer;
pub mod float;
pub mod bool;
pub mod nil;
pub mod errors;

use std::collections::hash_map::Values;
use macros::mux;
use paste::paste;

pub use integer::*;
pub use float::*;
pub use bool::*;
pub use nil::*;
pub use errors::*;

// ***************** operation defines *****************

pub trait BinaryOp<const OP_NAME:&'static str>{
    #[inline(always)]
    fn op_call(&self,other:RefConstValue) ->Result<Value>{
        Err(UnsupportedOp::new(OP_NAME).into())
    }
}

pub trait BinaryMutOp<const OP_NAME:&'static str>{
    #[inline(always)]
    fn op_call(&mut self,other:RefMutValue) ->Result<Value>{
        Err(UnsupportedOp::new(OP_NAME).into())
    }
}

pub trait UnaryOp<const OP_NAME:&'static str>{
    #[inline(always)]
    fn op_call(&self) ->Result<Value>{
        Err(UnsupportedOp::new(OP_NAME).into())
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

impl Value {
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

pub(crate) macro call_bin($op_name:literal,$left:expr,$right:expr) {
    ref_const_value_match!(($left).unbox_const(),left,{
        BinaryOp::<$op_name>::op_call(left,$right)
    })
}

pub(crate) macro call_unary($op_name:literal,$value:expr) {
    ref_const_value_match!(($value).unbox_const(),value,{
        UnaryOp::<$op_name>::op_call(value)
    })
}