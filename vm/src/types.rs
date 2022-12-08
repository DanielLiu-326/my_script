use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use macros::{val_enum_def, reg_enum_def, match_reg, impl_binary_ops, call_binary_op, match_value};
use crate::errors;
use crate::mem_collection::RefCount;
use crate::util::UncheckMut;
use crate::errors::*;

///
/// Value Types
///

pub trait Val{
    fn load_variable(&self,mutable:bool) -> RegType{

    }
    fn load_mutable(&self)
}
pub type Integer    = i64;

pub type Float      = f64;

pub type Bool       = bool;


#[derive(Debug,Clone,Copy)]
pub struct Nil();

#[val_enum_def]
#[derive(Debug)]
pub enum Value{
    Integer(Integer),
    Float(Float),
    Bool(Bool),
    Nil(Nil),
}
impl Value{
    #[inline(always)]
    pub fn load_variable(&self,mutable:bool)->RegType{
        match (self,mutable) {
            (Value::Integer(val),true) => {
                RegType::InlineInteger(InlineInteger::new(*val))
            },
            (Value::Integer(val),false) => {
                RegType::ConstInlineInteger(ConstInlineInteger::new(*val))
            },

            (Value::Float(val),true) => {
                RegType::InlineFloat(InlineFloat::new(*val))
            },
            (Value::Float(val),false) => {
                RegType::ConstInlineFloat(ConstInlineFloat::new(*val))
            },

            (Value::Bool(val) ,true) => {
                RegType::InlineBool(InlineBool::new(*val))
            },
            (Value::Bool(val),false) => {
                RegType::ConstInlineBool(ConstInlineBool::new(*val))
            },

            (Value::Nil(val) ,true) => {
                RegType::RefNil(RefNil::new(*val))
            },
            (Value::Nil(val),false) => {
                RegType::ConstRefNil(ConstRefNil::new(*val))
            }
        }
    }
    #[inline(always)]
    pub fn load_constant(&self)->RegType{
        match self{
            Value::Integer(val) => {
                RegType::ConstInteger(ConstInteger::new(*val))
            }
            Value::Float(val) => {
                RegType::ConstFloat(ConstFloat::new(*val))
            }
            Value::Bool(val) => {
                RegType::ConstBool(ConstBool::new(*val))
            }
            Value::Nil(val) => {
                RegType::ConstRefNil(ConstRefNil::new(*val))
            }
        }
    }
}

///
/// Register Types
///
pub trait RegTy{
    type Output;

    //unbox the reference into the type that can be operatee.
    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output>{
        Err(errors::MutabilityError::new(false).into())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output>{
        Err(errors::MutabilityError::new(true).into())
    }
}

#[reg_enum_def]
pub enum RegType{
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

impl Debug for RegType{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match_reg!(self => a{
            write!(f,"RegType::{}({:?})",__variant__,a.unbox_const().unwrap())?;
        });

        Ok(())
    }
}

impl Default for RegType{
    fn default() -> Self {
        Self::RefNil(RefNil::new(Nil()))
    }
}

pub struct InlineInteger(UncheckMut<Integer>);

impl InlineInteger{
    pub fn new(val:Integer) -> Self{
        Self(UncheckMut::new(val))
    }
}

impl RegTy for InlineInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}

pub struct InlineFloat(UncheckMut<Float>);

impl InlineFloat{
    pub fn new(val:Float) -> Self{
        Self(UncheckMut::new(val))
    }
}

impl RegTy for InlineFloat {
    type Output = Float;
    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}

pub struct InlineBool(UncheckMut<Bool>);

impl InlineBool{
    pub fn new(val:Bool) -> Self{
        Self(UncheckMut::new(val))
    }
}

impl RegTy for InlineBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}

pub struct ConstInlineInteger(Integer);

impl ConstInlineInteger{
    pub fn new(val:Integer) -> Self{
        Self(val)
    }
}

impl RegTy for ConstInlineInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub struct ConstInlineFloat(Float);

impl ConstInlineFloat {
    pub fn new(val:Float)->Self{
        Self::new(val)
    }
}

impl RegTy for ConstInlineFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub struct ConstInlineBool(Bool);

impl ConstInlineBool{
    pub fn new(val:Bool)->Self{
        Self(val)
    }
}

impl RegTy for ConstInlineBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub struct RefInteger(UncheckMut<RefCount<Integer>>);

impl RegTy for RefInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}

pub struct RefFloat(UncheckMut<RefCount<Float>>);

impl RegTy for RefFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}

pub struct RefBool(UncheckMut<RefCount<Bool>>);

impl RegTy for RefBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}


pub struct ConstRefInteger(RefCount<Integer>);

impl RegTy for ConstRefInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.deref())
    }
}

pub struct ConstRefFloat(RefCount<Float>);

impl RegTy for ConstRefFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.deref())
    }
}

pub struct ConstRefBool(RefCount<Bool>);

impl RegTy for ConstRefBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.deref())
    }
}

pub struct ConstInteger(Integer);

impl ConstInteger{
    pub fn new(val:Integer)->Self{
        Self(val)
    }
}

impl RegTy for ConstInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub struct ConstFloat(Float);

impl ConstFloat{
    pub fn new(val:Float) ->Self {
        Self(val)
    }
}

impl RegTy for ConstFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }

}

pub struct ConstBool(Bool);

impl ConstBool{
    pub fn new(val:Bool) -> Self{
        Self(val)
    }
}

impl RegTy for ConstBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub struct RefNil(UncheckMut<Nil>);

impl RefNil{
    pub fn new(val:Nil) -> Self{
        Self(UncheckMut::new(val))
    }
}

impl RegTy for RefNil{
    type Output = Nil;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}

pub struct ConstRefNil(Nil);

impl ConstRefNil{
    pub fn new(val:Nil) -> Self{
        Self(val)
    }
}

impl RegTy for ConstRefNil{
    type Output = Nil;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub fn op_OR(a:RegType,b:RegType)->Result<RegType>{
    use Value::*;
    match(a.unbox_const()?,b.unbox_const()?){
        (Integer(left),Integer(right)) => Ok(Value::Bool(*left != 0 || *right != 0)),


    }
}
impl_binary_ops!{
    OpOr => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Bool(*left != 0 || *right != 0)),
        (Integer,Float)     => Ok(Value::Bool(*left != 0 || *right != 0f64)),
        (Integer,Bool)      => Ok(Value::Bool(*left != 0 || *right)),

        //
        // Float
        //
        (Float,Integer) => Ok(Value::Bool(*left != 0f64 || *right != 0)),
        (Float,Float)   => Ok(Value::Bool(*left != 0f64 || *right != 0f64)),
        (Float,Bool)    => Ok(Value::Bool(*left != 0f64 || *right)),

        //
        // Bool
        //
        (Bool,Integer)  => Ok(Value::Bool(*left || *right != 0)),
        (Bool,Bool)     => Ok(Value::Bool(*left || *right)),
        (Bool,Float)    => Ok(Value::Bool(*left || *right != 0f64)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into())
    },

    OpAnd => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Bool(*left != 0 && *right != 0)),
        (Integer,Float)     => Ok(Value::Bool(*left != 0 && *right != 0f64)),
        (Integer,Bool)      => Ok(Value::Bool(*left != 0 && *right)),

        //
        // Float
        //
        (Float,Integer) => Ok(Value::Bool(*left != 0f64 && *right != 0)),
        (Float,Float)   => Ok(Value::Bool(*left != 0f64 && *right != 0f64)),
        (Float,Bool)    => Ok(Value::Bool(*left != 0f64 && *right)),

        //
        // Bool
        //
        (Bool,Integer)  => Ok(Value::Bool(*left && *right != 0)),
        (Bool,Float)    => Ok(Value::Bool(*left && *right != 0f64)),
        (Bool,Bool)     => Ok(Value::Bool(*left && *right )),

        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpBitOr => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Integer(*left | *right)),

        //
        // Bool
        //
        (Bool,Bool) => Ok(Value::Bool(*left || *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpBitXor => {
        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Integer(*left ^ *right)),

        //
        // Bool
        //
        (Bool,Bool) => Ok(Value::Bool(*left ^ *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpBitAnd => {
        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Integer(*left & *right)),

        //
        // Bool
        //
        (Bool,Bool) => Ok(Value::Bool(*left && *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpNe => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Bool(*left != *right)),
        (Integer,Float)     => Ok(Value::Bool(*left as Float != *right)),

        //
        // Float
        //
        (Float,Float)   => Ok(Value::Bool(*left != *right)),

        //
        // Bool
        //
        (Bool,Bool)     => Ok(Value::Bool(*left != *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpEq => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Bool(*left == *right)),
        (Integer,Float)     => Ok(Value::Bool((*left as Float) == *right)),

        //
        // Float
        //
        (Float,Integer) => Ok(Value::Bool(*left == *right as Float)),
        (Float,Float)   => Ok(Value::Bool(*left == *right)),

        //
        // Bool
        //
        (Bool,Bool) => Ok(Value::Bool(*left == *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpLt => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Bool(*left < *right)),
        (Integer,Float)     => Ok(Value::Bool((*left as Float) < *right)),

        //
        // Float
        //
        (Float,Integer) => Ok(Value::Bool(*left < *right as Float)),
        (Float,Float)   => Ok(Value::Bool(*left < *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpGt => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Bool(*left > *right)),
        (Integer,Float)     => Ok(Value::Bool((*left as Float) > *right)),

        //
        // Float
        //
        (Float,Float)   => Ok(Value::Bool(*left > *right)),
        (Float,Integer) => Ok(Value::Bool(*left > *right as Float)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpLe => {

        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Bool(*left <= *right)),

        //
        // Float
        //
        (Float,Float) => Ok(Value::Bool(*left < *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpGe=>{
        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Bool(*left >= *right)),

        //
        // Float
        //
        (Float,Float) => Ok(Value::Bool(*left >= *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpLMov=>{
        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Integer(*left << *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpRMov => {
        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Integer(*left >> *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpAdd =>{
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Integer(*left + *right)),
        (Integer,Float)     => Ok(Value::Float((*left as Float) + *right)),

        //
        //Float
        //
        (Float,Float)   => Ok(Value::Float(*left + *right)),
        (Float,Integer) => Ok(Value::Float(*left + *right as Float)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpSub => {
        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Integer(*left - *right)),
        (Integer,Float)   => Ok(Value::Float((*left as Float) - *right)),

        //
        // Float
        //
        (Float,Integer) => Ok(Value::Float(*left - *right as Float)),
        (Float,Float)   => Ok(Value::Float(*left - *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpMul => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Integer(*left * *right)),
        (Integer,Float)     => Ok(Value::Float((*left as Float) * *right)),

        //
        // Float
        //
        (Float,Integer) => Ok(Value::Float(*left * *right as Float)),
        (Float,Float)   => Ok(Value::Float(*left * *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpDiv => {
        //
        // Integer
        //
        (Integer,Integer)   => Ok(Value::Integer(*left / *right)),
        (Integer,Float)     => Ok(Value::Float((*left as Float) / *right)),

        //
        // Float
        //
        (Float,Integer) => Ok(Value::Float(*left / *right as Float)),
        (Float,Float)   => Ok(Value::Float(*left / *right)),

        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpMod => {
        //
        // Integer
        //
        (Integer,Integer) => Ok(Value::Integer(*left % *right)),

        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    OpFact => {
        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    },

    mut OpAssign => {
        //
        // Default
        //
        (_,_) => Err(UnsupportedOp::new(__op_name__).into()),
    }
}

pub fn op_or(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpOr,b).unwrap()
}

pub fn op_and(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpAnd,b).unwrap()
}

pub fn op_bit_or(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpBitOr,b).unwrap()
}

pub fn op_bit_xor(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpBitXor,b).unwrap()
}

pub fn op_bit_and(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpBitAnd,b).unwrap()
}

pub fn op_ne(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpNe,b).unwrap()
}

pub fn op_eq(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpEq,b).unwrap()
}

pub fn op_lt(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpLt,b).unwrap()
}

pub fn op_gt(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpGt,b).unwrap()
}

pub fn op_le(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpLe,b).unwrap()
}

pub fn op_ge(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpGe,b).unwrap()
}

pub fn op_l_mov(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpLMov,b).unwrap()
}

pub fn op_r_mov(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpRMov,b).unwrap()
}

pub fn op_add(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpAdd,b).unwrap()
}

pub fn op_sub(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpSub,b).unwrap()
}

pub fn op_mul(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpMul,b).unwrap()
}

pub fn op_div(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpDiv,b).unwrap()
}

pub fn op_mod(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpMod,b).unwrap()
}

pub fn op_fact(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpFact,b).unwrap()
}

// impl_default!(
//     OpOr    => {unimplemented!()},
//     OpAnd   => {unimplemented!()},
//     OpBitOr => {unimplemented!()},
//     OpBitXor=> {unimplemented!()},
//     OpBitAnd=> {unimplemented!()},
//     OpNe    => {unimplemented!()},
//     OpEq    => {unimplemented!()},
//     OpLt    => {unimplemented!()},
//     OpGt    => {unimplemented!()},
//     OpLe    => {unimplemented!()},
//     OpGe    => {unimplemented!()},
//     OpLMov  => {unimplemented!()},
//     OpRMov  => {unimplemented!()},
//     OpAdd   => {unimplemented!()},
//     OpSub   => {unimplemented!()},
//     OpMul   => {unimplemented!()},
//     OpDiv   => {unimplemented!()},
//     OpMod   => {unimplemented!()},
//     OpFact  => {unimplemented!()},
//     OpAssign=> {unimplemented!()},
//
//     //unary_ops:
//     OpBitNot=> {unimplemented!()},
//     OpNot   => {unimplemented!()},
//     OpNeg   => {unimplemented!()},
//     OpPos   => {unimplemented!()}
// );
