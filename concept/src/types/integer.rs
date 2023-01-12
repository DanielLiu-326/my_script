use std::fmt::Binary;
use utilities::UncheckMut;
use crate::types::{BinaryMutOp, BinaryOp, RefMutValue, Float, RefConstValue, UnaryOp, Value};
use super::errors::*;

pub type Integer = i64;

impl BinaryOp<"op_or"> for Integer {
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        let left = *self != 0;
        match other{
            RefConstValue::Bool   (right) => Ok(Value::Bool(left||*right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(left||(*right!=0))),
            _ => Result::Err(UnsupportedOp::new("op_or").into())
        }
    }
}

impl BinaryOp<"op_and"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        let left = *self != 0;
        match other{
            RefConstValue::Bool   (right) => Ok(Value::Bool(left&&*right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(left&&(*right!=0))),
            _ => Result::Err(UnsupportedOp::new("op_and").into())
        }
    }
}

impl BinaryOp<"op_bit_or"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self | *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_or").into())
        }
    }
}

impl BinaryOp<"op_bit_xor"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self ^ *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_xor").into())
        }
    }
}

impl BinaryOp<"op_bit_and"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self & *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_and").into())
        }
    }
}

impl BinaryOp<"op_ne"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Bool(*self != *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float != *right)),
            _ => Result::Err(UnsupportedOp::new("op_ne").into())
        }
    }
}

impl BinaryOp<"op_eq"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Bool(*self == *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float == *right)),
            _ => Result::Err(UnsupportedOp::new("op_eq").into())
        }
    }
}

impl BinaryOp<"op_lt"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Bool(*self < *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool((*self as Float) < *right)),
            _ => Result::Err(UnsupportedOp::new("op_lt").into())
        }
    }
}

impl BinaryOp<"op_gt"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Bool(*self > *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float > *right)),
            _ => Result::Err(UnsupportedOp::new("op_gt").into())
        }
    }
}

impl BinaryOp<"op_le"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Bool(*self <= *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float <= *right)),
            _ => Result::Err(UnsupportedOp::new("op_le").into())
        }
    }
}

impl BinaryOp<"op_ge"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Bool(*self >= *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float >= *right)),
            _ => Result::Err(UnsupportedOp::new("op_ge").into())
        }
    }
}

impl BinaryOp<"op_l_mov"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self << *right)),
            _ => Result::Err(UnsupportedOp::new("op_l_mov").into())
        }
    }
}

impl BinaryOp<"op_r_mov"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self >> *right)),
            _ => Result::Err(UnsupportedOp::new("op_r_mov").into())
        }
    }
}

impl BinaryOp<"op_add"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self + *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float + *right)),
            _ => Result::Err(UnsupportedOp::new("op_add").into())
        }
    }
}

impl BinaryOp<"op_sub"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self - *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float - *right)),
            _ => Result::Err(UnsupportedOp::new("op_sub").into())
        }
    }
}

impl BinaryOp<"op_mul"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self * *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float * *right)),
            _ => Result::Err(UnsupportedOp::new("op_mul").into())
        }
    }
}

impl BinaryOp<"op_div"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self / *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float / *right)),
            _ => Result::Err(UnsupportedOp::new("op_div").into())
        }
    }
}

impl BinaryOp<"op_mod"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            RefConstValue::Integer(right) => Ok(Value::Integer(*self % *right)),
            _ => Result::Err(UnsupportedOp::new("op_mod").into())
        }
    }
}

impl BinaryOp<"op_fact"> for Integer{
    #[inline(always)]
    fn op_call(&self, other: RefConstValue) -> Result<Value> {
        match other{
            _ => Result::Err(UnsupportedOp::new("op_fact").into())
        }
    }
}

impl BinaryMutOp<"op_assign"> for Integer {
    #[inline(always)]
    fn op_call(&mut self, other: RefMutValue) -> Result<Value> {
        match other{
            RefMutValue::Integer(right) => {
                *self = *right;
                Ok((*self).into())
            },
            _ => Result::Err(UnsupportedOp::new("op_assign").into())
        }
    }
}

impl UnaryOp<"op_bit_not"> for Integer {
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((!*self).into())
    }
}

impl UnaryOp<"op_not"> for Integer{
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((*self==0).into())
    }
}

impl UnaryOp<"op_neg"> for Integer{
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((-*self).into())
    }
}

impl UnaryOp<"op_pos"> for Integer{
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((*self).into())
    }
}
