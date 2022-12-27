use crate::errors::*;
use crate::types::{BinaryMutOp, BinaryOp, UnaryOp};
use crate::util::UncheckMut;
use super::Val;
use super::*;

pub type Bool = bool;

impl BinaryOp<"op_or"> for Bool {
    #[inline(always)]
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Bool   (right) => Ok(Value::Bool(*self || *right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(*self || *right)),
            _ => Result::Err(UnsupportedOp::new("op_or").into())
        }
    }
}

impl BinaryOp<"op_and"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Bool   (right) => Ok(Value::Bool(*self && *right)),
            RefConstValue::Integer(right) => Ok(Value::Bool(*self && *right)),
            _ => Result::Err(UnsupportedOp::new("op_and").into())
        }
    }
}

impl BinaryOp<"op_bit_or"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Bool(*self | *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_or").into())
        }
    }
}

impl BinaryOp<"op_bit_xor"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self ^ *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_xor").into())
        }
    }
}

impl BinaryOp<"op_bit_and"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Integer(right) => Ok(Value::Integer(*self & *right)),
            _ => Result::Err(UnsupportedOp::new("op_bit_and").into())
        }
    }
}

impl BinaryOp<"op_ne"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Bool(right) => Ok(Value::Bool(*self != *right)),
            _ => Ok(Value::Bool(true))
        }
    }
}

impl BinaryOp<"op_eq"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Bool(right) => Ok(Value::Bool(*self == *right)),
            _ => Result::Err(UnsupportedOp::new("op_eq").into())
        }
    }
}

impl BinaryOp<"op_lt"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_lt").into())
    }
}

impl BinaryOp<"op_gt"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_gt").into())
    }
}

impl BinaryOp<"op_le"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
        Err(UnsupportedOp::new("op_le").into())
    }
}

impl BinaryOp<"op_ge"> for Bool{
    fn op_call(&self, other: &RegType) -> Result<Value> {
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
    fn op_call(&mut self, other: &RegType) -> Result<Value> {
        match other.unbox_const(){
            RefConstValue::Bool(right) => {
                *self = *right;
                Ok((*self).into())
            },
            _ => Result::Err(UnsupportedOp::new("op_assign").into())
        }
    }
}

impl UnaryOp<"op_bit_not"> for Bool {
    fn op_call(&self) -> Result<Value> {
        Ok((!*self).into())
    }
}

impl UnaryOp<"op_not"> for Bool{
    fn op_call(&self) -> Result<Value> {
        Ok((!*self).into())
    }
}

impl UnaryOp<"op_neg"> for Bool{}

impl UnaryOp<"op_pos"> for Bool{}

// reg types
pub struct InlineBool<const MUTABLE:bool>(UncheckMut<Bool>);

impl<const MUTABLE:bool> InlineBool<MUTABLE> {
    pub fn new(val:Bool)->Self{
        Self(UncheckMut::new(val))
    }
}

impl Val for Bool{
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            RegType::InlineInteger(InlineBool::new(*self))
        }else{
            RegType::ConstInlineInteger(InlineBool::new(*self))
        }
    }
}



