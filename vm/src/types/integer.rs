use std::fmt::Binary;
use crate::errors::*;
use crate::types::{BinaryMutOp, BinaryOp, UnaryOp};
use crate::util::UncheckMut;
use super::Val;
use super::float::*;
use super::RefConstValue;
use crate::types::RegType;
use super::*;

pub type Integer = i64;

impl BinaryOp<"op_or"> for Integer {
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        let left = *self != 0;
        match other.unbox_const(){
            RefConstValue::Bool   (right) => Ok(Value::Bool(left||*right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(left||(*right!=0))),
            _ => Result::Err(UnsupportedOp::new("op_or").into())
        }
    }
}

impl BinaryOp<"op_and"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        let left = *self != 0;
        match other.unbox_const(){
            RefConstValue::Bool   (right) => Ok(Value::Bool(left&&*right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(left&&(*right!=0))),
            _ => Result::Err(UnsupportedOp::new("op_and").into())
        }
    }
}

impl BinaryOp<"op_bit_or"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self | *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_or").into())
        }
    }
}

impl BinaryOp<"op_bit_xor"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self ^ *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_xor").into())
        }
    }
}

impl BinaryOp<"op_bit_and"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self & *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_and").into())
        }
    }
}

impl BinaryOp<"op_ne"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self != *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float != *right)),
            _ => Result::Err(UnsupportedOp::new("op_ne").into())
        }
    }
}

impl BinaryOp<"op_eq"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self == *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float == *right)),
            _ => Result::Err(UnsupportedOp::new("op_eq").into())
        }
    }
}

impl BinaryOp<"op_lt"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self < *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool((*self as Float) < *right)),
            _ => Result::Err(UnsupportedOp::new("op_lt").into())
        }
    }
}

impl BinaryOp<"op_gt"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self > *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float > *right)),
            _ => Result::Err(UnsupportedOp::new("op_gt").into())
        }
    }
}

impl BinaryOp<"op_le"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self <= *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float <= *right)),
            _ => Result::Err(UnsupportedOp::new("op_le").into())
        }
    }
}

impl BinaryOp<"op_ge"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self >= *right)),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float >= *right)),
            _ => Result::Err(UnsupportedOp::new("op_ge").into())
        }
    }
}

impl BinaryOp<"op_l_mov"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self << *right)),
            _ => Result::Err(UnsupportedOp::new("op_l_mov").into())
        }
    }
}

impl BinaryOp<"op_r_mov"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self >> *right)),
            _ => Result::Err(UnsupportedOp::new("op_r_mov").into())
        }
    }
}

impl BinaryOp<"op_add"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self + *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float + *right)),
            _ => Result::Err(UnsupportedOp::new("op_add").into())
        }
    }
}

impl BinaryOp<"op_sub"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self - *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float - *right)),
            _ => Result::Err(UnsupportedOp::new("op_sub").into())
        }
    }
}

impl BinaryOp<"op_mul"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self * *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float * *right)),
            _ => Result::Err(UnsupportedOp::new("op_mul").into())
        }
    }
}

impl BinaryOp<"op_div"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self / *right)),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float / *right)),
            _ => Result::Err(UnsupportedOp::new("op_div").into())
        }
    }
}

impl BinaryOp<"op_mod"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self % *right)),
            _ => Result::Err(UnsupportedOp::new("op_mod").into())
        }
    }
}

impl BinaryOp<"op_fact"> for Integer{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            _ => Result::Err(UnsupportedOp::new("op_fact").into())
        }
    }
}

impl BinaryMutOp<"op_assign"> for Integer {
    fn op_call(&mut self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => {
                *self = *right;
                Ok((*self).into())
            },
            _ => Result::Err(UnsupportedOp::new("op_assign").into())
        }
    }
}

impl UnaryOp<"op_bit_not"> for Integer {
    fn op_call(&self) -> Result<Value> {
        Ok((!*self).into())
    }
}

impl UnaryOp<"op_not"> for Integer{
    fn op_call(&self) -> Result<Value> {
        Ok((*self==0).into())
    }
}

impl UnaryOp<"op_neg"> for Integer{
    fn op_call(&self) -> Result<Value> {
        Ok((-*self).into())
    }
}

impl UnaryOp<"op_pos"> for Integer{
    fn op_call(&self) -> Result<Value> {
        Ok((*self).into())
    }
}

// reg types
pub struct InlineInteger<const MUTABLE:bool>(UncheckMut<Integer>);

impl<const MUTABLE:bool> InlineInteger<MUTABLE> {
    pub fn new(val:Integer)->Self{
        Self(UncheckMut::new(val))
    }
}
impl<const MUTABLE:bool> RegTy for InlineInteger<MUTABLE>{
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    fn unbox_mut(&self) -> Result<RefMutValue> {
        if MUTABLE {
            Ok(self.0.get_mut().into())
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl Val for Integer{
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            RegType::InlineInteger(InlineInteger::new(*self))
        }else{
            RegType::ConstInlineInteger(InlineInteger::new(*self))
        }
    }
}



