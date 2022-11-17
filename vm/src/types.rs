#[feature(repru8)]

use crate::mem_collection::{RefConst, RefCount};
use paste::paste;
use std::mem::size_of;
use std::ops::{Add, Sub};
use std::process::Output;
use std::ptr::NonNull;
use crate::util::allocate;
use macros::match_2_value ;


macro_rules! enum_vals {
    () => {
        InlineInteger,InlineBool,InlineFloat,RefInteger,
        RefBool,RefFloat,ConstRefInteger,ConstRefBool,
        ConstRefFloat,ConstInteger,ConstBool,
        ConstFloat,RefNil,ConstRefNil
    };
}

macro_rules! def_binary_op_trait {
    ($trait_name:ident,$fn_name:ident) => {
        pub trait $trait_name<T,U> {
            fn $fn_name(&self,value:T)->U{
                unimplemented!()
            }
        }
    };
}

macro_rules! def_unary_op_trait {
    ($trait_name:ident,$fn_name:ident) => {
        pub trait $trait_name<U> {
            fn $fn_name(&self)->U{
                unimplemented!()
            }
        }
    };
}

def_binary_op_trait!(OpOr,      or);
def_binary_op_trait!(OpAnd,     and);
def_binary_op_trait!(OpBitOr,   bit_or);
def_binary_op_trait!(OpBitXor,  bit_xor);
def_binary_op_trait!(OpBitAnd,  bit_and);
def_binary_op_trait!(OpNe,      ne);
def_binary_op_trait!(OpEq,      eq);
def_binary_op_trait!(OpRefEq,   ref_eq);
def_binary_op_trait!(OpRefNe,   ref_ne);
def_binary_op_trait!(OpLt,      lt);
def_binary_op_trait!(OpGt,      gt);
def_binary_op_trait!(OpLe,      le);
def_binary_op_trait!(OpGe,      ge);
def_binary_op_trait!(OpRefLt,   ref_lt);
def_binary_op_trait!(OpRefGt,   ref_gt);
def_binary_op_trait!(OpRefLe,   ref_le);
def_binary_op_trait!(OpRefGe,   ref_ge);
def_binary_op_trait!(OpLMov,    ref_left_move);
def_binary_op_trait!(OpRMov,    ref_right_move);
def_binary_op_trait!(OpAdd,     add);
def_binary_op_trait!(OpSub,     sub);
def_binary_op_trait!(OpMul,     mul);
def_binary_op_trait!(OpDiv,     div);
def_binary_op_trait!(OpMod,     modulo);
def_binary_op_trait!(OpFact,    fact);

def_unary_op_trait!(OpBitNot,   bit_not);
def_unary_op_trait!(OpNot,      not);
def_unary_op_trait!(OpNeg,      neg);
def_unary_op_trait!(OpPos,      pos);

pub trait AssignVal<T>{
    fn assign_val(&mut self,val:T){
        unimplemented!("assign")
    }
}


///所有在寄存器可能出现的类型组合
pub enum Value {
    ///未装箱的基本类型
    InlineInteger(Integer),
    InlineBool   (Bool),
    InlineFloat  (Float),

    ///已经装箱的基本类型
    RefInteger   (RefInteger),
    RefBool      (RefBool),
    RefFloat     (RefFloat),

    ConstRefInteger (RefConstInteger),
    ConstRefBool    (RefConstBool),
    ConstRefFloat   (RefConstFloat),


    ///对象类型
    // RefArray(Array),
    // RefDict(Ptr),
    // RefStruct(Ptr),
    // RefFunction(Ptr),

    // ConstRefArray(ArrayObject),
    // ConstRefDict(Ptr),
    // ConstRefStruct(Ptr),
    // ConstRefFunction(Ptr),

    ///Load const指令加载的常量类型
    ConstInteger(Integer),
    ConstBool(Bool),
    ConstFloat(Float),
    // ConstString(String),

    RefNil(()),
    ConstRefNil(()),
}


#[inline(always)]
pub fn add(left:&Value,right:&Value)->Value{
    let a = match_2_value!((left,right),a+b,enum_vals!());
}

impl Default for Value{
    fn default() -> Self {
        Self::RefNil(())
    }
}

pub trait Unbox{
    type Output:Copy;

    fn unbox(&self) -> Self::Output;
}

type Integer    = i64;
type Float      = f64;
type Bool       = bool;

type RefInteger = RefCount<i64>;
type RefFloat   = RefCount<f64>;
type RefBool    = RefCount<bool>;

type RefConstInteger = RefConst<i64>;
type RefConstFloat   = RefConst<f64>;
type RefConstBool    = RefConst<bool>;