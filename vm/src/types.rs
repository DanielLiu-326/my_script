use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use macros::{val_enum_def, reg_enum_def, match_1_reg, impl_binary_ops, call_binary_op};
use crate::errors;
use crate::mem_collection::RefCount;
use crate::util::UncheckMut;
use crate::errors::{Error,Result};

///
/// Value Types
///

pub type Integer    = i64;

pub type Float      = f64;

pub type Bool       = bool;

#[derive(Debug)]
pub struct Nil();

#[val_enum_def]
#[derive(Debug)]
pub enum Value{
    Integer(Integer),
    Float(Float),
    Bool(Bool),
    Nil(Nil),
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
        None
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
        match_1_reg!(self => a,{
            write!(f,"RegType::{}({:?})",__variant__,a.unbox_const().unwrap())?;
        });

        Ok(())
    }
}

impl RegType{

}

pub struct InlineInteger(UncheckMut<Integer>);

impl RegTy for InlineInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Some(self.0.get_mut())
    }
}

pub struct InlineFloat(UncheckMut<Float>);

impl RegTy for InlineFloat {
    type Output = Float;
    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Some(self.0.get_mut())
    }
}

pub struct InlineBool(UncheckMut<Bool>);

impl RegTy for InlineBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Some(self.0.get_mut())
    }
}

pub struct ConstInlineInteger(Integer);

impl RegTy for ConstInlineInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(&self.0)
    }
}

pub struct ConstInlineFloat(Float);

impl RegTy for ConstInlineFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(&self.0)
    }
}

pub struct ConstInlineBool(Bool);

impl RegTy for ConstInlineBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(&self.0)
    }
}

pub struct RefInteger(UncheckMut<RefCount<Integer>>);

impl RegTy for RefInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Some(self.0.get_mut())
    }
}

pub struct RefFloat(UncheckMut<RefCount<Float>>);

impl RegTy for RefFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Some(self.0.get_mut())
    }
}

pub struct RefBool(UncheckMut<RefCount<Bool>>);

impl RegTy for RefBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Some(self.0.get_mut())
    }
}


pub struct ConstRefInteger(RefCount<Integer>);

impl RegTy for ConstRefInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstRefFloat(RefCount<Float>);

impl RegTy for ConstRefFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstRefBool(RefCount<Bool>);

impl RegTy for ConstRefBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstInteger(RefCount<Integer>);

impl RegTy for ConstInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct ConstFloat(RefCount<Float>);

impl RegTy for ConstFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.deref())
    }

}

pub struct ConstBool(RefCount<Bool>);

impl RegTy for ConstBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.deref())
    }
}

pub struct RefNil(UncheckMut<Nil>);

impl RegTy for RefNil{
    type Output = Nil;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Some(self.0.get_mut())
    }
}

pub struct ConstRefNil(Nil);

impl RegTy for ConstRefNil{
    type Output = Nil;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Some(&self.0)
    }
}

impl_binary_ops!{
    OpOr => {
        (Integer,Integer) => {
            Some(Value::Integer(left + right))
        },
        (_,_) => {
            None
        }
    },

    OpAnd => {
        (_,_) => {
            None
        }
    },

    OpBitOr => {
        (_,_) => {
            None
        }
    },

    OpBitXor => {

        (_,_) => {
            None
        }
    },

    OpBitAnd => {

        (_,_) => {
            None
        }
    },

    OpNe => {
        (_,_) => {
            None
        }
    },

    OpEq => {
        (_,_) => {
            None
        }
    },

    OpLt => {
        (_,_) => {
            None
        }
    },

    OpGt => {
        (_,_) => {
            None
        }
    },

    OpLe=>{
        (_,_) => {
            None
        }
    },

    OpGe=>{
        (_,_) => {
            None
        }
    },

    OpLMov=>{
        (_,_) => {
            None
        }
    },

    OpRMov => {
        (_,_) => {
            None
        }
    },

    OpAdd =>{
        (_,_) => {
            None
        }
    },

    OpSub => {
        (_,_) => {
            None
        }
    },

    OpMul => {
        (_,_) => {
            None
        }
    },

    OpDiv => {
        (_,_) => {
            None
        }
    },

    OpMod => {
        (_,_) => {
            None
        }
    },

    OpFact => {
        (_,_) => {
            None
        }
    },

    mut OpAssign => {
        (_,_) => {
            None
        }
    }
}

pub fn op_or(a:&RegType,b:&RegType)->Value{
    call_binary_op!(a,OpOr,b).unwrap()
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
