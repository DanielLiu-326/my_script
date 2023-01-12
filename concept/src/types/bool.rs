use errors::*;
use crate::types::{BinaryMutOp, BinaryOp, UnaryOp};
use super::Val;
use super::*;
use utilities::UncheckMut;

pub type Bool = bool;

impl BinaryOp<"op_or"> for Bool {
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Bool   (right) => Ok(Value::Bool(*self || *right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(*self || (*right!=0))),
            _ => Result::Err(UnsupportedOp::new("op_or").into())
        }
    }
}

impl BinaryOp<"op_and"> for Bool{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Bool   (right) => Ok(Value::Bool(*self && *right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(*self && (*right!=0))),
            _ => Result::Err(UnsupportedOp::new("op_and").into())
        }
    }
}

impl BinaryOp<"op_bit_or"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Bool(right) => Ok(Value::Bool(*self | *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_or").into())
        }
    }
}

impl BinaryOp<"op_bit_xor"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Bool(right) => Ok(Value::Bool(*self ^ *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_xor").into())
        }
    }
}

impl BinaryOp<"op_bit_and"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Bool(right) => Ok(Value::Bool(*self & *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_and").into())
        }
    }
}

impl BinaryOp<"op_ne"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Bool(right) => Ok(Value::Bool(*self != *right)),
            _ => Ok(Value::Bool(true))
        }
    }
}

impl BinaryOp<"op_eq"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Bool(right) => Ok(Value::Bool(*self == *right)),
            _ => Result::Err(UnsupportedOp::new("op_eq").into())
        }
    }
}

impl BinaryOp<"op_lt"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        Err(UnsupportedOp::new("op_lt").into())
    }
}

impl BinaryOp<"op_gt"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        Err(UnsupportedOp::new("op_gt").into())
    }
}

impl BinaryOp<"op_le"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        Err(UnsupportedOp::new("op_le").into())
    }
}

impl BinaryOp<"op_ge"> for Bool{
    #[inline(always)]
    fn op_call(&self, other:RefConstValue) -> Result<Value> {
        Err(UnsupportedOp::new("op_ge").into())
    }
}

impl BinaryOp<"op_l_mov"> for Bool{}

impl BinaryOp<"op_r_mov"> for Bool{}

impl BinaryOp<"op_add"> for Bool{}

impl BinaryOp<"op_sub"> for Bool{}

impl BinaryOp<"op_mul"> for Bool{}

impl BinaryOp<"op_div"> for Bool{}

impl BinaryOp<"op_mod"> for Bool{}

impl BinaryOp<"op_fact"> for Bool{}

impl BinaryMutOp<"op_assign"> for Bool {
    #[inline(always)]
    fn op_call(&mut self, other: RefMutValue) -> Result<Value> {
        match other{
            RefMutValue::Bool(right) => {
                *self = *right;
                Ok((*self).into())
            },
            _ => Result::Err(UnsupportedOp::new("op_assign").into())
        }
    }
}

impl UnaryOp<"op_bit_not"> for Bool {
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((!*self).into())
    }
}

impl UnaryOp<"op_not"> for Bool{
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((!*self).into())
    }
}

impl UnaryOp<"op_neg"> for Bool{}

impl UnaryOp<"op_pos"> for Bool{}
