use std::arch::x86_64::_xabort;
use std::cell::{RefMut, UnsafeCell};
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::mem::Discriminant;
use std::net::IpAddr;
use std::ops::{Deref, DerefMut};
use std::process::Output;
use crate::mem_collection::{Addr, RefAddr, RefConst, RefCount};
use paste::paste;
use macros::{define_val, impl_default, impl_op, match_1_value, match_2_values, op_define};


///
/// Value Types
///

pub type Integer    = i64;

pub type Float      = f64;

pub type Bool       = bool;

pub struct Nil();

#[val_def]
pub enum Value{
    Integer(Integer),
    Float(Float),
    Bool(Bool),
    Nil(Nil),
}

///
/// Variability
///

pub struct UncheckMut<T>(UnsafeCell<T>);

impl<T> UncheckMut<T> {
    fn new(val:T)->Self{
        Self(UnsafeCell::new(val))
    }
    #[inline(always)]
    fn ref_mut(&self)-> &mut T{unsafe{
        (&mut *self.0.get())
    }}
}


///
/// Register Types
///
pub trait RegTy{
    type Output;
    //unbox the reference into the type that can be operatee.
    fn unbox_const(&self) -> Option<&Self::Output>{
        None
    }
    fn unbox_mut(&self) -> Option<&mut Self::Output>{
        None
    }
}



#[ref_define]
pub enum RegType {
    /// 内联可变变量
    InlineInteger(InlineInteger),
    InlineFloat  (InlineFloat),
    InlineBool   (InlineBool),

    /// 内联不可变变量
    ConstInlineInteger  (ConstInlineInteger),
    ConstInlineFloat    (ConstInlineFloat),
    ConstInlineBool     (ConstInlineBool),

    /// 可变引用
    RefInteger   (RefInteger),
    RefFloat     (RefFloat),
    RefBool      (RefBool),

    /// 不可变引用
    ConstRefInteger (ConstRefInteger),
    ConstRefFloat   (ConstRefFloat),
    ConstRefBool    (ConstRefBool),


    ///对象类型
    // RefArray(Array),
    // RefDict(Ptr),
    // RefStruct(Ptr),
    // RefFunction(Ptr),

    // ConstRefArray(ArrayObject),
    // ConstRefDict(Ptr),
    // ConstRefStruct(Ptr),
    // ConstRefFunction(Ptr),

    /// 常量
    ConstInteger(ConstInteger),
    ConstFloat  (ConstFloat),
    ConstBool   (ConstBool),
    // ConstString(String),

    /// NIL
    RefNil      (RefNil),
    ConstRefNil (ConstRefNil),
}

pub struct InlineInteger(UncheckMut<Integer>);

impl RegTy for InlineInteger{
    type Output = Integer;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&*self.0.ref_mut())
    }

    fn unbox_mut(&self) -> Option<&mut Self::Output> {
        Some(self.0.ref_mut())
    }
}

pub struct InlineFloat(UncheckMut<Float>);

impl RegTy for InlineFloat {
    type Output = Float;
    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&*self.0.ref_mut())
    }
    fn unbox_mut(&self) -> Option<&mut Self::Output> {
        Some(self.0.ref_mut())
    }
}

pub struct InlineBool(UncheckMut<Bool>);

impl RegTy for InlineBool{
    type Output = Bool;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&*self.0.ref_mut())
    }

    fn unbox_mut(&self) -> Option<&mut Self::Output> {
        Some(self.0.ref_mut())
    }
}

pub struct ConstInlineInteger(Integer);

impl RegTy for ConstInlineInteger{
    type Output = Integer;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&self.0)
    }
}

pub struct ConstInlineFloat(Float);

impl RegTy for ConstInlineFloat{
    type Output = Float;
    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&self.0)
    }
}

pub struct ConstInlineBool(Bool);

impl RegTy for ConstInlineBool{
    type Output = Bool;
    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&self.0)
    }
}

pub struct RefInteger(UncheckMut<RefCount<Integer>>);

impl RegTy for RefInteger{
    type Output = Integer;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&*self.0.ref_mut())
    }

    fn unbox_mut(&self) -> Option<&mut Self::Output> {
        Some(self.0.ref_mut())
    }
}

pub struct RefFloat(UncheckMut<RefCount<Float>>);

impl RegTy for RefFloat{
    type Output = Float;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&*self.0.ref_mut())
    }

    fn unbox_mut(&self) -> Option<&mut Self::Output> {
        Some(self.0.ref_mut())
    }
}

pub struct RefBool(UncheckMut<RefCount<Bool>>);

impl RegTy for RefFloat{
    type Output = Bool;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&*self.0.ref_mut())
    }

    fn unbox_mut(&self) -> Option<&mut Self::Output> {
        Some(self.0.ref_mut())
    }
}


pub struct ConstRefInteger(RefCount<Integer>);

impl RegTy for ConstRefInteger{
    type Output = Integer;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstRefFloat(RefCount<Float>);

impl RegTy for ConstRefFloat{
    type Output = Float;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstRefBool(RefCount<Bool>);

impl RegTy for ConstRefBool{
    type Output = Bool;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstInteger(RefCount<Integer>);

impl RegTy for ConstInteger{
    type Output = Integer;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstFloat(RefCount<Float>);

impl RegTy for ConstFloat{
    type Output = Float;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(self.0.deref())
    }

}

pub struct ConstBool(RefCount<Bool>);

impl RegTy for ConstBool{
    type Output = Bool;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct RefNil(UncheckMut<Nil>);

impl RegTy for RefNil{
    type Output = Nil;
    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&*self.0.ref_mut())
    }
    fn unbox_mut(&self) -> Option<&mut Self::Output> {
        Some(self.0.ref_mut())
    }
}

pub struct ConstRefNil(Nil);

impl RegTy for ConstRefNil{
    type Output = Nil;

    fn unbox_const(&self) -> Option<&Self::Output> {
        Some(&self.0)
    }
}



///
/// Operators
///

macro_rules! def_binary_op_trait {
    ($trait_name:ident,$fn_name:ident) => {paste!{
        #[op_define]
        pub trait $trait_name<T> {
            fn $fn_name(&self,_other:&T) -> Value;
        }
        #[inline(always)]
        pub fn $fn_name(a:&Value,b:&Value) -> Value {
            return match_2_values!((a,b),{a.unbox_const().$fn_name(b.unbox_const())});
        }
    }};
}

macro_rules! def_unary_op_trait {
    ($trait_name:ident,$fn_name:ident) => {paste!{
        #[op_define]
        pub trait $trait_name {
            #[inline(always)]
            fn $fn_name(&self)->Value{
                unimplemented!()
            }
        }
        #[inline(always)]
        pub fn $fn_name<T:$trait_name>(a:&Value) -> Value{
            return match_1_value!(a,{a.unbox_const().$fn_name()});
        }
    }}
}

//
// Binary Ops
//

def_binary_op_trait!(OpOr,       op_or);
def_binary_op_trait!(OpAnd,      op_and);
def_binary_op_trait!(OpBitOr,    op_bit_or);
def_binary_op_trait!(OpBitXor,   op_bit_xor);
def_binary_op_trait!(OpBitAnd,   op_bit_and);
def_binary_op_trait!(OpNe,       op_ne);
def_binary_op_trait!(OpEq,       op_eq);
def_binary_op_trait!(OpLt,       op_lt);
def_binary_op_trait!(OpGt,       op_gt);
def_binary_op_trait!(OpLe,       op_le);
def_binary_op_trait!(OpGe,       op_ge);
def_binary_op_trait!(OpLMov,     op_l_mov);
def_binary_op_trait!(OpRMov,     op_r_mov);
def_binary_op_trait!(OpAdd,      op_add);
def_binary_op_trait!(OpSub,      op_sub);
def_binary_op_trait!(OpMul,      op_mul);
def_binary_op_trait!(OpDiv,      op_div);
def_binary_op_trait!(OpMod,      op_mod);
def_binary_op_trait!(OpFact,     op_fact);

//
// Unary Ops
//

def_unary_op_trait!(OpBitNot,    op_bit_not);
def_unary_op_trait!(OpNot,       op_not);
def_unary_op_trait!(OpNeg,       op_neg);
def_unary_op_trait!(OpPos,       op_pos);



#[inline(always)]
pub fn op_assign_val<T>(a:&Value,b:&Value) -> Value{
    return match_2_values!((a,b),{a.unbox().op_assign_val(b.unbox)})
}





#[derive(Clone)]
pub enum Const{
    Integer(RefConst<Integer>),
    Float(RefConst<Float>),
    Bool(RefConst<Bool>),
}
impl Const{
    pub fn load_mut_ref(&self) -> Value{
        match self {
            Const::Integer(x) => {Value::InlineInteger(**x)}
            Const::Float(x) => {Value::InlineFloat(**x)}
            Const::Bool(x) => {Value::InlineBool(**x)}
        }
    }

    pub fn load_const_ref(&self) ->Value{
        match self{
            Const::Integer(x) => {Value::ConstInlineInteger(**x)}
            Const::Float(x) => {Value::ConstInlineFloat(**x)}
            Const::Bool(x) => {Value::ConstInlineBool(**x)}
        }
    }

    pub fn load_const(&self) -> Value{
        match self {
            Const::Integer(x) => {Value::ConstInteger(**x)}
            Const::Float(x) => {Value::ConstFloat(**x)}
            Const::Bool(x) => {Value::ConstBool(**x)}
        }
    }
}

/// **************************************** Op Implementations ****************************************

//////////////////////////////////////////////////////////////
/////////////////////////////BOOL/////////////////////////////
//////////////////////////////////////////////////////////////
#[impl_op]
impl OpOr<Bool> for Bool{
    #[inline(always)]
    fn op_or(&self,other:&Bool)->Value{
        Value::InlineBool(*self || *other)
    }
}

#[impl_op]
impl OpAnd<Bool> for Bool{
    #[inline(always)]
    fn op_and(&self,other:&Bool) -> Value{
        Value::InlineBool(*self && *other)
    }
}

#[impl_op]
impl OpBitOr<Bool> for Bool{
    #[inline(always)]
    fn op_bit_or(&self,other:&Bool) -> Value{
        Value::InlineBool(*self^*other)
    }
}

#[impl_op]
impl OpBitXor<Bool> for Bool{
    #[inline(always)]
    fn op_bit_xor(&self,other:&Bool) -> Value{
        Value::InlineBool(*self|other)
    }
}

#[impl_op]
impl OpBitAnd<Bool> for Bool{
    #[inline(always)]
    fn op_bit_and(&self,other:&Bool) -> Value{
        Value::InlineBool(*self&*other)
    }
}

#[impl_op]
impl OpNe<Bool> for Bool{
    #[inline(always)]
    fn op_ne(&self,other:&Bool) -> Value{
        Value::InlineBool(*self != *other)
    }
}

#[impl_op]
impl OpEq<Bool> for Bool{
    #[inline(always)]
    fn op_eq(&self,other:&Bool) -> Value{
        Value::InlineBool(*self == *other)
    }
}

//////////////////////////////////////////////////////////////
///////////////////////////Integer////////////////////////////
//////////////////////////////////////////////////////////////
#[impl_op]
impl OpBitOr<Integer> for Integer{
    fn op_bit_or(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self | *other)
    }
}

#[impl_op]
impl OpBitXor<Integer> for Integer{
    fn op_bit_xor(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self^*other)
    }
}

#[impl_op]
impl OpBitAnd<Integer> for Integer{
    fn op_bit_and(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self & *other)
    }
}

#[impl_op]
impl OpNe<Integer> for Integer{
    fn op_ne(&self,other:&Integer) -> Value{
        Value::InlineBool(*self != *other)
    }
}

#[impl_op]
impl OpEq<Integer> for Integer{
    fn op_eq(&self,other:&Integer) -> Value{
        Value::InlineBool(*self == *other)
    }
}

#[impl_op]
impl OpLt<Integer> for Integer{
    fn op_lt(&self,other:&Integer) -> Value {
        Value::InlineBool(*self < *other)
    }
}

#[impl_op]
impl OpGt<Integer> for Integer{
    fn op_gt(&self,other:&Integer) -> Value{
        Value::InlineBool(*self>*other)
    }
}

#[impl_op]
impl OpLe<Integer> for Integer{
    fn op_le(&self,other:&Integer) -> Value{
        Value::InlineBool(*self<=*other)
    }
}

#[impl_op]
impl OpGe<Integer> for Integer{
    fn op_ge(&self,other:&Integer) -> Value{
        Value::InlineBool(*self>=*other)
    }
}

#[impl_op]
impl OpLMov<Integer> for Integer {
    fn op_l_mov(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self << *other)
    }
}

#[impl_op]
impl OpRMov<Integer> for Integer{
    fn op_r_mov(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self>> *other)
    }
}

#[impl_op]
impl OpAdd<Integer> for Integer{
    fn op_add(&self,other:&Integer) -> Value{
        Value::InlineInteger(self + other)
    }
}

#[impl_op]
impl OpSub<Integer> for Integer{
    fn op_sub(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self - *other)
    }
}

#[impl_op]
impl OpMul<Integer> for Integer{
    fn op_mul(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self * *other)
    }
}

#[impl_op]
impl OpDiv<Integer> for Integer{
    fn op_div(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self / *other)
    }
}

#[impl_op]
impl OpMod<Integer> for Integer{
    fn op_mod(&self,other:&Integer) -> Value{
        Value::InlineInteger(*self % *other)
    }
}

#[impl_op]
impl OpFact<Integer> for Integer{
    fn op_fact(&self,other:&Integer) -> Value{
        unimplemented!("op fact is unimplemented now")
    }
}

#[impl_op]
impl OpBitNot for Integer{
    fn op_bit_not(&self) -> Value{
        Value::InlineInteger(! *self)
    }
}

#[impl_op]
impl OpNot for Integer{
    fn op_not(&self) -> Value{
        Value::InlineInteger(! *self)
    }
}

#[impl_op]
impl OpNeg for Integer{
    fn op_neg(&self) -> Value{
        Value::InlineInteger(- *self)
    }
}

#[impl_op]
impl OpPos for Integer{
    fn op_pos(&self) -> Value{
        Value::InlineInteger(*self)
    }
}



impl_default!(
    OpOr    => {unimplemented!()},
    OpAnd   => {unimplemented!()},
    OpBitOr => {unimplemented!()},
    OpBitXor=> {unimplemented!()},
    OpBitAnd=> {unimplemented!()},
    OpNe    => {unimplemented!()},
    OpEq    => {unimplemented!()},
    OpLt    => {unimplemented!()},
    OpGt    => {unimplemented!()},
    OpLe    => {unimplemented!()},
    OpGe    => {unimplemented!()},
    OpLMov  => {unimplemented!()},
    OpRMov  => {unimplemented!()},
    OpAdd   => {unimplemented!()},
    OpSub   => {unimplemented!()},
    OpMul   => {unimplemented!()},
    OpDiv   => {unimplemented!()},
    OpMod   => {unimplemented!()},
    OpFact  => {unimplemented!()},
    OpAssign=> {unimplemented!()},

    //unary_ops:
    OpBitNot=> {unimplemented!()},
    OpNot   => {unimplemented!()},
    OpNeg   => {unimplemented!()},
    OpPos   => {unimplemented!()}
);

