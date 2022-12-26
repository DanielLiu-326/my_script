use std::collections::hash_map::Values;
use macros::mux;
use paste::paste;
use crate::errors::*;
use crate::mem_collection::RefConst;
use crate::util::UncheckMut;

//*****************************************************
pub trait Val{
    fn load_variable(&self,mutable:bool) -> RegType;
}

pub type Integer = i64;
pub type Float   = f64;
pub type Bool    = bool;
pub type Nil     = ();

#[mux]
#[derive(Debug)]
pub enum Value{
    Integer (Integer),
    Float   (Float),
    Bool    (Bool),
    Nil     (Nil),
}

#[mux]
#[derive(Debug)]
pub enum RefConstValue<'a>{
    Integer (&'a Integer),
    Float   (&'a Float),
    Bool    (&'a Bool),
    Nil     (&'a Nil),
}

#[mux]
#[derive(Debug)]
pub enum RefMutValue<'a>{
    Integer (&'a mut Integer),
    Float   (&'a mut Float),
    Bool    (&'a mut Bool),
    Nil     (&'a mut Nil),
}

impl Val for Integer{
    #[inline(always)]
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            InlineInteger::<true>::new(*self).into()
        }else{
            InlineInteger::<false>::new(*self).into()
        }
    }

}
impl Val for Float{
    #[inline(always)]
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            InlineFloat::<true>::new(*self).into()
        }else{
            InlineFloat::<false>::new(*self).into()
        }
    }
}
impl Val for Bool{
    #[inline(always)]
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            InlineBool::<true>::new(*self).into()
        }else{
            InlineBool::<false>::new(*self).into()
        }
    }
}
impl Val for Nil{
    #[inline(always)]
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            RefNil::<true>::new().into()
        }else{
            RefNil::<false>::new().into()
        }
    }
}

impl Value{
    #[inline(always)]
    pub fn load_variable(&self,mutable:bool) -> RegType{
        value_match!(self,val,{
            val.load_variable(mutable)
        })
    }
}

impl<'a> RefConstValue<'a>{
    #[inline(always)]
    pub fn try_into_bool(&self)->Result<&bool>{
        match self{
            Bool(ret) => {
                Ok(ret)
            },
            _ => {
                Err(TypeError::new("Bool","...").into())
            },
        }
    }
}

//*****************************************************

pub trait RegTy{
    #[inline(always)]
    fn unbox_mut(&self)->Result<RefMutValue>{
        Err(MutabilityError::new().into())
    }

    fn unbox_const(&self)->RefConstValue;

    fn clone_ref(&self) -> RegType{
        todo!()
    }
}

pub struct InlineInteger<const MUTABLE:bool>(UncheckMut<Integer>);
pub struct InlineFloat<const MUTABLE:bool>  (UncheckMut<Float>);
pub struct InlineBool<const MUTABLE:bool>   (UncheckMut<Bool>);
pub struct RefNil<const MUTABLE:bool>       (UncheckMut<Nil>);

#[mux]
pub enum RegType{
    InlineInteger(InlineInteger<true>),
    InlineFloat(InlineFloat<true>),
    InlineBool(InlineBool<true>),

    ConstInlineInteger(InlineInteger<false>),
    ConstInlineFloat(InlineFloat<false>),
    ConstInlineBool(InlineBool<false>),

    RefNil(RefNil<true>),
    ConstRefNil(RefNil<false>),
}
impl Default for RegType{
    fn default() -> Self {
        Self::RefNil(RefNil::new())
    }
}

impl RegType{
    #[inline(always)]
    pub fn unbox_const(&self) -> RefConstValue{
        reg_type_match!(self,reg,{
            reg.unbox_const().into()
        })
    }
    #[inline(always)]
    pub fn unbox_mut(&self) -> Result<RefMutValue>{
        reg_type_match!(self,reg,{
            Ok(reg.unbox_mut()?.into())
        })
    }
}

impl<const MUTABLE:bool> InlineInteger<MUTABLE>{
    fn new(val:Integer)->Self{
        Self(UncheckMut::new(val))
    }
}

impl<const MUTABLE:bool> InlineFloat<MUTABLE>{
    fn new(val:Float)->Self{
        Self(UncheckMut::new(val))
    }
}

impl<const MUTABLE:bool> InlineBool<MUTABLE>{
    fn new(val:Bool)->Self{
        Self(UncheckMut::new(val))
    }
}

impl<const MUTABLE:Bool> RegTy for InlineInteger<MUTABLE>{
    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue> {
        if MUTABLE{
            Ok(self.0.get_mut().into())
        }else{
            Err(MutabilityError::new().into())
        }
    }
}
impl<const MUTABLE:Bool> RegTy for InlineFloat<MUTABLE>{
    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue> {
        if MUTABLE{
            Ok(self.0.get_mut().into())
        }else{
            Err(MutabilityError::new().into())
        }
    }
}
impl<const MUTABLE:Bool> RegTy for InlineBool<MUTABLE>{
    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue> {
        if MUTABLE{
            Ok(self.0.get_mut().into())
        }else{
            Err(MutabilityError::new().into())
        }
    }
}
impl<const MUTABLE:Bool> RefNil<MUTABLE>{
    pub fn new()->Self{
        Self(UncheckMut::new(()))
    }
}
impl<const MUTABLE:Bool> RegTy for RefNil<MUTABLE>{
    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue> {
        if MUTABLE{
            Ok(self.0.get_mut().into())
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

macro_rules! def_bin_op {
    ($($op_name:ident => {$($pattern:tt => $output:expr,)*} )* ) => {
        $(
            #[inline(always)]
            pub fn $op_name(left:&RegType,right:&RegType) -> Result<Value>{
                use RefConstValue::*;
                match (left.unbox_const(),right.unbox_const()){
                    $(
                        $pattern => {
                            use crate::types::*; //to solve RefConstValue::Float and types::Float
                            $output
                        }
                    )*
                }
            }
        )*
    };
}

macro_rules! def_mut_bin_op {
    ($($op_name:ident => {$( $pattern:tt => $output:expr,)*} )* ) => {
        $(
            #[inline(always)]
            pub fn $op_name(left:&RegType,right:&RegType) -> Result<Value>{
                match (left.unbox_mut()?,right.unbox_const()){
                    $(
                        $pattern => {$output}
                    )*
                }
            }
        )*
    };
}
macro_rules! def_unary_op {
    ($($op_name:ident => {$( $pattern:tt => $output:expr,)*} )* ) => {
        $(
            #[inline(always)]
            pub fn $op_name(left:&RegType) -> Result<Value>{
                use crate::types::RefConstValue::*;
                match (left.unbox_const()){
                    $(
                        $pattern => {
                            use crate::types::*; //to solve RefConstValue::Float and types::Float
                            $output
                        }
                    )*
                }
            }
        )*
    };
}

def_bin_op!{
    op_or  => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left != 0 || *right!=0).into()),
        (Integer(left),Bool(right)   ) => Ok((*left != 0 || *right).into()),

        // Bool
        (Bool(left)   ,Integer(right)) => Ok((*left || *right!=0).into()),
        (Bool(left)   ,Bool(right)   ) => Ok((*left || *right).into()),

        //default
        (_,_) => Result::Err(UnsupportedOp::new("op_or").into()),
    }

    op_and => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left != 0 && *right!=0).into()),
        (Integer(left),Bool(right)   ) => Ok((*left != 0 && *right).into()),

        // Bool
        (Bool(left)   ,Integer(right)) => Ok((*left && *right!=0).into()),
        (Bool(left)   ,Bool(right)   ) => Ok((*left && *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_and").into()),
    }

    op_bit_or => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left | *right).into()),

        // Bool
        (Bool(left)   ,Bool(right)   ) => Ok((*left | *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_bit_or").into()),
    }

    op_bit_xor => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left ^ *right).into()),

        // Bool
        (Bool(left)   ,Bool(right)   ) => Ok((*left ^ *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_bit_xor").into()),
    }

    op_bit_and => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left & *right).into()),

        // Bool
        (Bool(left)   ,Bool(right)   ) => Ok((*left & *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_bit_and").into()),
    }

    op_ne => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left != *right).into()),
        (Integer(left) ,Float(right)  ) => Ok((*left as f64 != *right).into()),

        // Float
        (Float(left)   ,Float(right)  ) => Ok((*left != *right).into()),
        (Float(left)   ,Integer(right)) => Ok((*left != *right as Float).into()),

        // Bool
        (Bool(left)    ,Bool(right)   ) => Ok((*left != *right).into()),

        // default
        (_,_) => Ok((true).into()),
    }

    op_eq => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left == *right).into()),
        (Integer(left) ,Float(right)  ) => Ok((*left as f64 == *right).into()),

        // Float
        (Float(left)   ,Float(right)  ) => Ok((*left == *right).into()),
        (Float(left)   ,Integer(right)) => Ok((*left == *right as Float).into()),

        // Bool
        (Bool(left)    ,Bool(right)   ) => Ok((*left == *right).into()),

        // default
        (_,_) => Ok((false).into()),
    }

    op_lt => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left < *right).into()),
        (Integer(left) ,Float(right)  ) => Ok(((*left as Float) < *right).into()),

        // Float
        (Float(left)   ,Float(right)  ) => Ok((*left < *right).into()),
        (Float(left)   ,Integer(right)) => Ok((*left < *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_lt").into()),
    }

    op_gt => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left > *right).into()),
        (Integer(left) ,Float(right)  ) => Ok((*left as f64 > *right).into()),

        // Float
        (Float(left)   ,Float(right)  ) => Ok((*left > *right).into()),
        (Float(left)   ,Integer(right)) => Ok((*left > *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_gt").into()),
    }

    op_le => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left <= *right).into()),
        (Integer(left) ,Float(right)  ) => Ok((*left as f64 <= *right).into()),

        // Float
        (Float(left)   ,Float(right)  ) => Ok((*left <= *right).into()),
        (Float(left)   ,Integer(right)) => Ok((*left <= *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_le").into()),
    }

    op_ge => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left >= *right).into()),
        (Integer(left) ,Float(right)  ) => Ok((*left as f64 >= *right).into()),

        // Float
        (Float(left)   ,Float(right)  ) => Ok((*left >= *right).into()),
        (Float(left)   ,Integer(right)) => Ok((*left >= *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_ge").into()),
    }

    op_l_mov => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left << *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_l_mov").into()),
    }

    op_r_mov => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left >> *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_r_mov").into()),
    }

    op_add => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left + *right).into()),
        (Integer(left) ,Float(right)  ) => Ok(((*left as Float) + *right).into()),

        // Float
        (Float(left)   ,Integer(right)) => Ok((*left + *right as Float).into()),
        (Float(left)   ,Float(right)  ) => Ok((*left + *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_add").into()),
    }

    op_sub => {
        // Integer
        (Integer(left) ,Integer(right)) => Ok((*left - *right).into()),
        (Integer(left) ,Float(right)  ) => Ok(((*left as Float) - *right).into()),

        // Float
        (Float(left)   ,Integer(right)) => Ok((*left - *right as Float).into()),
        (Float(left)   ,Float(right)  ) => Ok((*left - *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_sub").into()),
    }

    op_mul => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left * *right).into()),
        (Integer(left),Float(right)  ) => Ok(((*left as Float) * *right).into()),
        // Float
        (Float(left),Integer(right)  ) => Ok((*left * *right as Float).into()),
        (Float(left),Float(right)    ) => Ok((*left * *right).into()),

        // default
        (_,_) => Err(UnsupportedOp::new("op_mul").into()),
    }

    op_div => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left / *right).into()),
        (Integer(left),Float(right)  ) => Ok(((*left as Float) / *right).into()),
        // Float
        (Float(left),Integer(right)  ) => Ok((*left / *right as Float).into()),
        (Float(left),Float(right)    ) => Ok((*left / *right).into()),

        // default
        (_,_) => Err(UnsupportedOp::new("op_div").into()),
    }

    op_mod => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left % *right).into()),

        // default
        (_,_) => Err(UnsupportedOp::new("op_mod").into()),
    }

    op_fact => {
        // Integer
        (Integer(left),Integer(right)) => Ok((*left % *right).into()),

        // default
        (_,_) => Err(UnsupportedOp::new("op_fact").into()),
    }
}
def_mut_bin_op!{
    op_assign => {
        (RefMutValue::Integer(left),RefConstValue::Integer(right)) => {*left = *right;Ok((*left).into())},
        (RefMutValue::Float(left)  ,RefConstValue::Float(right)  ) => {*left = *right;Ok((*left).into())},
        (RefMutValue::Bool(left)   ,RefConstValue::Bool(right)   ) => {*left = *right;Ok((*left).into())},
        (_,_) => Err(UnsupportedOp::new("op_assign").into()),
    }
}

def_unary_op!{
    op_bit_not => {
        (Integer(left)) => Ok((!left).into()),
        (Bool(left)   ) => Ok((!left).into()),
        (_) => Err(UnsupportedOp::new("op_bit_not").into()),
    }
    op_not => {
        (Bool(left)   ) => Ok((!left).into()),
        (_) => Err(UnsupportedOp::new("op_not").into()),
    }
    op_neg => {
        (Integer(left)) => Ok((-left).into()),
        (Float(left)  ) => Ok((-left).into()),
        (_) => Err(UnsupportedOp::new("op_neg").into()),
    }
    op_pos => {
        (Integer(left)) => Ok((*left).into()),
        (Float(left)  ) => Ok((*left).into()),
        (_) => Err(UnsupportedOp::new("op_pos").into()),
    }
}