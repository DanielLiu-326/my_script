use std::arch::x86_64::_xabort;
use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::mem::Discriminant;
use std::net::IpAddr;
use std::simd::{Simd, usizex2};
use crate::mem_collection::{Addr, RefAddr, RefConst, RefCount};
use paste::paste;
use macros::{define_val, impl_default, impl_op, match_1_value, match_2_values, op_define};


type Integer    = i64;

type Float      = f64;

type Bool       = bool;

pub enum Value{
    Integer(Integer),
    Float(Float),
    Bool(Bool),
}

macro_rules! def_binary_op_trait {
    ($trait_name:ident,$fn_name:ident) => {paste!{
        #[op_define]
        pub trait $trait_name<T> {
            fn $fn_name(&self,_other:&T) -> Value;
        }
        #[inline(always)]
        pub fn $fn_name(a:&Value,b:&Value) -> Value {
            return match_2_values!((a,b),{a.unbox().$fn_name(b.unbox())});
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
            return match_1_value!(a,{a.unbox().$fn_name()});
        }
    }}
}

///
/// # Binary Ops
///

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

def_binary_op_trait!(OpAssign,    op_assign);


///
/// # Unary Ops
///

def_unary_op_trait!(OpBitNot,    op_bit_not);
def_unary_op_trait!(OpNot,       op_not);
def_unary_op_trait!(OpNeg,       op_neg);
def_unary_op_trait!(OpPos,       op_pos);



pub trait Ref{
    type Output;
    fn unbox(&self) -> &mut Self::Output;    //unbox the reference into the type that can be operatee.
}

pub struct InlineInteger(UnsafeCell<Integer>);

impl Ref for InlineInteger{
    type Output = Integer;

    fn unbox(&self) -> &mut Self::Output {
        self.0
    }
}

pub struct InlineFloat(Float);

impl Ref for InlineFloat{
    type Output = Float;

    fn unbox(&self) -> Self::Output {
        self.0
    }
}

pub struct InlineBool(Bool);

impl Ref for InlineBool{
    type Output = Bool;

    fn unbox(&self) -> Self::Output {
        self.0
    }
}

pub struct ConstInlineInteger(Integer);

impl Ref for ConstInlineInteger{
    type Output = Integer;

    fn unbox(&self) -> Self::Output {
        self.0
    }
}

pub struct ConstInlineFloat(Float);

impl Ref for ConstInlineFloat{
    type Output = Float;

    fn unbox(&self) -> Self::Output {
        self.0
    }
}

pub struct ConstInlineBool(Bool);

impl Ref for ConstInlineBool{
    type Output = Bool;

    fn unbox(&self) -> Self::Output {
        self.0
    }
}

pub struct RefInteger(RefCount<Integer>);

impl Ref for ConstInlineInteger{
    type Output = Integer;

    fn unbox(&self) -> Self::Output {
        **self
    }
}

pub enum Reference{
    /// 内联可变变量
    InlineInteger(Integer),
    InlineFloat  (Float),
    InlineBool   (Bool),

    /// 内联不可变变量
    ConstInlineInteger  (Integer),
    ConstInlineFloat    (Float),
    ConstInlineBool     (Bool),

    /// 可变引用
    RefInteger   (RefInteger),
    RefFloat     (RefFloat),
    RefBool      (RefBool),

    /// 不可变引用
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

    /// 常量
    ConstInteger(Integer),
    ConstBool(Bool),
    ConstFloat(Float),
    // ConstString(String),

    /// NIL
    RefNil(()),
    ConstRefNil(()),
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

///所有在寄存器可能出现的类型组合
#[define_val]
#[derive(Debug)]
pub enum Value {
    /// 内联可变变量
    InlineInteger(Integer),
    InlineFloat  (Float),
    InlineBool   (Bool),

    /// 内联不可变变量
    ConstInlineInteger  (Integer),
    ConstInlineFloat    (Float),
    ConstInlineBool     (Bool),

    /// 可变引用
    RefInteger   (RefInteger),
    RefFloat     (RefFloat),
    RefBool      (RefBool),

    /// 不可变引用
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

    /// 常量
    ConstInteger(Integer),
    ConstBool(Bool),
    ConstFloat(Float),
    // ConstString(String),

    /// NIL
    RefNil(()),
    ConstRefNil(()),
}

pub enum RefCompare{
    ConstInteger(Integer),
    ConstFloat(Float),
    ConstBool(Bool),
    MemAddr(RefAddr),
    InlineValue,
    Nil(),
}

impl Value{

    pub fn ref_addr(&self) -> Option<usizex2>{
        match self{
            Value::InlineInteger(_) => {Option::None}
            Value::InlineFloat(_) => {Option::None}
            Value::InlineBool(_) => {Option::None}
            Value::ConstInlineInteger(_) => {Option::None}
            Value::ConstInlineFloat(_) => {Option::None}
            Value::ConstInlineBool(_) => {Option::None}
            Value::RefInteger(a) => {Option::Some(a.ref_addr().into())}
            Value::RefFloat(a) => {Option::Some(a.ref_addr().into())}
            Value::RefBool(a) => {Option::Some(a.ref_addr().into())}
            Value::ConstRefInteger(a) => {Option::Some(a.ref_addr().into())}
            Value::ConstRefFloat(a) => {Option::Some(a.ref_addr().into())}
            Value::ConstRefBool(a) => {Option::Some(a.ref_addr().into())}
            Value::ConstInteger(_) => {Option::Some()}
            Value::ConstBool(_) => {}
            Value::ConstFloat(_) => {}
            Value::RefNil(_) => {}
            Value::ConstRefNil(_) => {}
        }
    }
}

impl Default for Value{
    fn default() -> Self {
        Self::RefNil(())
    }
}

/// Unbox a type into the operatee type
/// - every type should implement Unbox as inline functions. if the type is operatee type,just
/// implement it by make Output Type.
/// - all operatee types should define the operations that they can do. and
pub trait Unbox {
    type Output;
    fn unbox(&self) -> &Self::Output;
}






impl<T:Copy> Unbox for T {
    type Output = T;

    fn unbox(&self) -> &T {
        self
    }
}

impl<T:Copy> Unbox for RefCount<T>{
    type Output = T;

    fn unbox(&self) -> &Self::Output {
        &**self
    }
}

impl<T:Copy> Unbox for RefConst<T>{
    type Output = T;

    fn unbox(&self) -> &Self::Output {
        &**self
    }
}






impl Debug for RefInteger{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref {}",**self)
    }
}

impl Debug for RefFloat{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref {}",**self)
    }
}

impl Debug for RefBool{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref {}",**self)
    }
}

impl Debug for RefConstInteger{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref {}",**self)
    }
}

impl Debug for RefConstFloat{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref const {}",**self)
    }
}

impl Debug for RefConstBool{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ref const {}",**self)
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

pub fn op_ref_eq(a:&Value,b:&Value) -> Value{

}
pub fn op_ref_ne(a:&Value,b:&Value) -> Value{

}