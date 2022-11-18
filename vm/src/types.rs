#[feature(repru8)]

use crate::mem_collection::{RefConst, RefCount};
use paste::paste;
use std::mem::size_of;
use std::process::Output;
use std::ptr::NonNull;
use crate::util::allocate;
use macros::match_2_value ;

///所有在寄存器可能出现的类型组合
pub enum Value {
    ///未装箱的基本类型
    InlineInteger(Integer),
    InlineFloat  (Float),
    InlineBool   (Bool),

    ///已经装箱的基本类型
    RefInteger   (RefInteger),
    RefFloat     (RefFloat),
    RefBool      (RefBool),

    ConstRefInteger (RefConstInteger),
    ConstRefFloat   (RefConstFloat),
    ConstRefBool    (RefConstBool),


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

macro_rules! def_binary_op_trait {
    ($trait_name:ident,$fn_name:ident) => {
        pub trait $trait_name<T> {
            #[inline(always)]
            fn $fn_name(&self,other:T) -> Value{
                unimplemented!()
            }
        }
        impl<T,U> $trait_name<U> for T{}
    };
}

macro_rules! def_unary_op_trait {
    ($trait_name:ident,$fn_name:ident) => {
        pub trait $trait_name {
            #[inline(always)]
            fn $fn_name(&self)->Value{
                unimplemented!()
            }
        }
        impl<T> $trait_name for T{}
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

macro_rules! def_binary_value_op {
    ($a:ident,$b:ident,$stmt:stmt) => {
        paste
        match_2_value!({a.add(b)},InlineInteger,InlineFloat,InlineBool,
            RefInteger,RefFloat,RefBool,
            ConstRefInteger,ConstRefFloat,ConstRefBool,
            ConstInteger,ConstFloat,ConstBool,
            RefNil,ConstRefNil);
    };
}

#[inline]
pub fn add(a:&Value,b:&Value){
    let mut a = def_binary_value_op!(a,b,a.add(b));
    ;
}
