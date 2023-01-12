use super::errors::*;
use crate::types::{BinaryMutOp, BinaryOp, RefMutValue, UnaryOp};
use utilities::UncheckMut;
use crate::types::Val;

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

impl Val for Nil{}


