use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use macros::{val_enum_def, reg_enum_def, match_1_reg, impl_binary_ops, call_binary_op};
use crate::errors;
use crate::mem_collection::RefCount;
use crate::util::UncheckMut;
use crate::errors::*;

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
        match_1_reg!(self => a,{
            write!(f,"RegType::{}({:?})",__variant__,a.unbox_const().unwrap()).unwrap();
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
        Ok(self.0.get())
    }

    #[inline(always)]
    fn unbox_mut(&self) -> Result<&mut Self::Output> {
        Ok(self.0.get_mut())
    }
}

pub struct InlineFloat(UncheckMut<Float>);

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

impl RegTy for ConstInlineInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub struct ConstInlineFloat(Float);

impl RegTy for ConstInlineFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

pub struct ConstInlineBool(Bool);

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

pub struct ConstInteger(RefCount<Integer>);

impl RegTy for ConstInteger{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.deref())
    }
}

pub struct ConstFloat(RefCount<Float>);

impl RegTy for ConstFloat{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.deref())
    }

}

pub struct ConstBool(RefCount<Bool>);

impl RegTy for ConstBool{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(self.0.deref())
    }
}

pub struct RefNil(UncheckMut<Nil>);

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

impl RegTy for ConstRefNil{
    type Output = Nil;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

impl_binary_ops!{
    OpOr => {
        (Integer,Integer) => {
            Ok(Value::Bool(*left!=0||*right!=0))
        },
        (Bool,Bool) => {
            Ok(Value::Bool(*left || *right))
        },
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpAnd => {
        (Integer,Integer) => {
            Ok(Value::Bool(*lef!=0&&*right!=0))
        },
        (Bool,Bool) => {
            Ok(Value::Bool(*left && *right))
        },
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpBitOr => {
        (Integer,Integer) => {
            Ok(Value::Bool(*lef!=0&&*right!=0))
        },
        (Bool,Bool) => {
            Ok(Value::Bool(*left || *right))
        },
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpBitXor => {
        (Bool,Bool) => {
            Ok(Value::Bool(*left ^ *right))
        },
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpBitAnd => {
        (Bool,Bool) => {
            Ok(Value::Bool(*left && *right))
        },
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpNe => {
        (Bool,Bool) => {
            Ok(Value::Bool(*left != *right))
        },
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpEq => {
        (Bool,Bool) => {
            Ok(Value::Bool(*left == *right))
        },
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpLt => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpGt => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpLe => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpGe=>{
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpLMov=>{
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpRMov => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpAdd =>{
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpSub => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpMul => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpDiv => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpMod => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    OpFact => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
        }
    },

    mut OpAssign => {
        (_,_) => {
            Err(UnsupportedOp::new(__op_name__).into())
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
