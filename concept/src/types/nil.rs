use super::errors::*;
use crate::types::{BinaryMutOp, BinaryOp, RefMutValue, UnaryOp};

#[derive(Debug)]
pub struct Nil;

impl BinaryOp<"op_or"> for Nil {}

impl BinaryOp<"op_and"> for Nil{}

impl BinaryOp<"op_bit_or"> for Nil{}

impl BinaryOp<"op_bit_xor"> for Nil{}

impl BinaryOp<"op_bit_and"> for Nil{}

impl BinaryOp<"op_ne"> for Nil{}

impl BinaryOp<"op_eq"> for Nil{}

impl BinaryOp<"op_lt"> for Nil{}

impl BinaryOp<"op_gt"> for Nil{}

impl BinaryOp<"op_le"> for Nil{}

impl BinaryOp<"op_ge"> for Nil{}

impl BinaryOp<"op_l_mov"> for Nil{}

impl BinaryOp<"op_r_mov"> for Nil{}

impl BinaryOp<"op_add"> for Nil{}

impl BinaryOp<"op_sub"> for Nil{}

impl BinaryOp<"op_mul"> for Nil{}

impl BinaryOp<"op_div"> for Nil{}

impl BinaryOp<"op_mod"> for Nil{}

impl BinaryOp<"op_fact"> for Nil{}

impl BinaryMutOp<"op_assign"> for Nil {}

impl UnaryOp<"op_bit_not"> for Nil {}

impl UnaryOp<"op_not"> for Nil {}

impl UnaryOp<"op_neg"> for Nil {}

impl UnaryOp<"op_pos"> for Nil {}

// reg types
pub struct RefNil<const MUTABLE:bool>(UncheckMut<Nil>);

impl<const MUTABLE:bool> RefNil<MUTABLE> {
    #[inline(always)]
    pub fn new()->Self{
        Self(UncheckMut::new(Nil))
    }
}

impl<const MUTABLE:bool> RegTy for RefNil<MUTABLE>{
    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue> {
        if MUTABLE{
            Ok(self.0.get_mut().into())
        } else {
            Err(MutabilityError::new().into())
        }
    }
}

impl Val for Nil{
    #[inline(always)]
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            RegType::RefNil(RefNil::new())
        }else{
            RegType::ConstRefNil(RefNil::new())
        }
    }
}


