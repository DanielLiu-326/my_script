use std::fmt::Binary;
use crate::errors::*;
use crate::types::{BinaryMutOp, BinaryOp, UnaryOp};
use crate::util::UncheckMut;
use super::Val;
use super::RefConstValue;
use super::*;

pub type Float = f64;

impl BinaryOp<"op_or"> for Float {
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_or").into())
    }
}

impl BinaryOp<"op_and"> for Float {
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_and").into())
    }
}

impl BinaryOp<"op_bit_or"> for Float {
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_bit_or").into())
    }
}

impl BinaryOp<"op_bit_xor"> for Float {
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_bit_xor").into())
    }
}

impl BinaryOp<"op_bit_and"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_bit_and").into())
    }
}

impl BinaryOp<"op_ne"> for Float {
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self != (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self != *right)),
            _ => Err(UnsupportedOp::new("op_ne").into())
        }
    }
}

impl BinaryOp<"op_eq"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self == (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self == *right)),
            _ => Err(UnsupportedOp::new("op_eq").into())
        }
    }
}

impl BinaryOp<"op_lt"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self < (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self < *right)),
            _ => Err(UnsupportedOp::new("op_lt").into())
        }
    }
}

impl BinaryOp<"op_gt"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self > (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self as Float > *right)),
            _ => Err(UnsupportedOp::new("op_gt").into())
        }
    }
}

impl BinaryOp<"op_le"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self <= (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self <= *right)),
            _ => Err(UnsupportedOp::new("op_le").into())
        }
    }
}

impl BinaryOp<"op_ge"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self >= (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Bool(*self >= *right)),
            _ => Err(UnsupportedOp::new("op_ge").into())
        }
    }
}

impl BinaryOp<"op_l_mov"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_l_mov").into())
    }
}

impl BinaryOp<"op_r_mov"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_r_mov").into())
    }
}

impl BinaryOp<"op_add"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Float(*self + (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Float(*self as Float + *right)),
            _ => Err(UnsupportedOp::new("op_add").into())
        }
    }
}

impl BinaryOp<"op_sub"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Float(*self - (*right  as Float))),
            RefConstValue::Float  (right) => Ok(Value::Float(*self - *right)),
            _ => Err(UnsupportedOp::new("op_sub").into())
        }
    }
}

impl BinaryOp<"op_mul"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Float(*self * (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Float(*self * *right)),
            _ => Err(UnsupportedOp::new("op_mul").into())
        }
    }
}

impl BinaryOp<"op_div"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Float(*self / (*right as Float))),
            RefConstValue::Float  (right) => Ok(Value::Float(*self / *right)),
            _ => Err(UnsupportedOp::new("op_div").into())
        }
    }
}

impl BinaryOp<"op_mod"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_mod").into())
    }
}

impl BinaryOp<"op_fact"> for Float{
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_fact").into())
    }
}

impl BinaryMutOp<"op_assign"> for Float {
    #[inline(always)]
    fn op_call(&mut self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Float(right) => {
                *self = *right;
                Ok((*self).into())
            },
            _ => Err(UnsupportedOp::new("op_assign").into())
        }
    }
}

impl UnaryOp<"op_bit_not"> for Float {
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Err(UnsupportedOp::new("op_assign").into())
    }
}

impl UnaryOp<"op_not"> for Float{
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Err(UnsupportedOp::new("op_not").into())
    }
}

impl UnaryOp<"op_neg"> for Float{
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((-*self).into())
    }
}

impl UnaryOp<"op_pos"> for Float{
    #[inline(always)]
    fn op_call(&self) -> Result<Value> {
        Ok((*self).into())
    }
}

// reg types
pub struct InlineFloat<const MUTABLE:bool>(UncheckMut<Float>);

impl<const MUTABLE:bool> InlineFloat<MUTABLE> {
    #[inline(always)]
    pub fn new(val:Float)->Self{
        Self(UncheckMut::new(val))
    }
}
impl<const MUTABLE:bool> RegTy for InlineFloat<MUTABLE>{
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

impl Val for Float{
    #[inline(always)]
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            RegType::InlineFloat(InlineFloat::new(*self))
        }else{
            RegType::ConstInlineFloat(InlineFloat::new(*self))
        }
    }
}



