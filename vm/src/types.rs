use paste::paste;
use overloadf::overload;
use crate::errors::*;
use crate::mem_collection::RefCount;
use macros::mux;

/// Implements the function for ops
macro_rules! bin_op_impl {
    ($op_name:ident,(_,_) => $expr:expr) => {paste!{
        impl<Left,Right> Op for [<$op_name:camel:upper>] <Left,Right>{
            default fn calculate(self) -> Result<Value>{
                $expr
            }
        }
    }};

    ($op_name:ident,($left_var:ident:$left_ty:ty,$right_var:ident:$right_ty:ty) => $expr:expr) => {paste!{
        impl<'a> Op for [<$op_name:camel:upper>]<&'a $left_ty,&'a $right_ty>{
            #[inline(always)]
            fn calculate(self) -> Result<Value>{
                let Self(
                    $left_var,
                    $right_var,
                ) = self;
                $expr
            }
        }
    }};

    (mut $op_name:ident,($left_var:ident:$left_ty:ty,$right_var:ident:$right_ty:ty) => $expr:expr) => {paste!{
        impl<'a> OpMut for [<$op_name:camel:upper>]<&'a mut $left_ty,&'a $right_ty>{
            #[inline(always)]
            fn calculate(self) -> Result<Value>{
                let Self(
                    $left_var,
                    $right_var,
                ) = self;
                $expr
            }
        }
    }};

    (mut $op_name:ident,(_,_) => $expr:expr) => {paste!{
        impl<Left,Right> OpMut for [<$op_name:camel:upper>] <Left,Right>{
            default fn calculate(self) -> Result<Value>{
                $expr
            }
        }
    }};
}
macro_rules! def_bin_op_struct {
    ($op_name:ident) => {paste!{
        pub struct [<$op_name:camel:upper>]<Left,Right>(
            Left,
            Right,
        );

        impl<Left,Right> [<$op_name:camel:upper>]<Left,Right>{
            pub fn new(left:Left,right:Right)->Self{
                Self(
                    left,
                    right,
                )
            }
        }
    }};
}

macro_rules! mut_bin_op_def {
    ( $( mut $op_name:ident =>  { $($args:tt => $body:expr ,)* })* ) => {paste!{
        pub trait OpMut {
            fn calculate(self) -> Result<Value>;
        }

        $(
            def_bin_op_struct!($op_name);

            $(bin_op_impl!{mut $op_name,$args => $body})*

            pub fn $op_name<Left,Right>(left:&mut Left,right:&Right)->Result<Value>{
                [<$op_name:camel:upper>]::new(left,right).calculate()
            }
        )*
    }}
}
macro_rules! bin_op_def {
    ( $( $op_name:ident => { $($args:tt => $body:expr ,)* })* ) => {paste!{
        pub trait Op {
            fn calculate(self) -> Result<Value>;
        }
        $(
            def_bin_op_struct!( $op_name );

            $(bin_op_impl!{$op_name,$args => $body})*

            pub fn $op_name<Left,Right>(left:&Left,right:&Right)->Result<Value>{
                [<$op_name:camel:upper>]::new(left,right).calculate()
            }
        )*
    }}
}

///
/// value types
///
pub trait ValueType{
    fn into_reg(self) -> RegType;
}

type Integer = i64;
type Float   = f64;
type Bool    = bool;

#[mux]
#[derive(Debug)]
pub enum Value{
    Integer(Integer),
    Float(Float),
    Bool(Bool),
}





impl ValueType for Integer{
    fn into_reg(self) -> RegType {
        RegType::InlineInteger(InlineInteger::new(self))
    }
}

impl ValueType for Float{
    fn into_reg(self) -> RegType {
        RegType::InlineFloat(InlineFloat::new(self))
    }
}

impl ValueType for Bool{
    fn into_reg(self) -> RegType {
        RegType::InlineBool(InlineBool::new(self))
    }
}

/* operations */

bin_op_def!{
    op_or  => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left != 0 || *right!=0).into()),
        (left:Integer,right:Bool   ) => Ok((*left != 0 || *right).into()),

        // Bool
        (left:Bool   ,right:Integer) => Ok((*left || *right!=0).into()),
        (left:Bool   ,right:Bool   ) => Ok((*left || *right).into()),

        //default
        (_,_) => Result::Err(UnsupportedOp::new("op_or").into()),
    }

    op_and => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left != 0 && *right!=0).into()),
        (left:Integer,right:Bool   ) => Ok((*left != 0 && *right).into()),

        // Bool
        (left:Bool   ,right:Integer) => Ok((*left && *right!=0).into()),
        (left:Bool   ,right:Bool   ) => Ok((*left && *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_and").into()),
    }

    op_bit_or => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left | *right).into()),

        // Bool
        (left:Bool   ,right:Bool   ) => Ok((*left | *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_bit_or").into()),
    }

    op_bit_xor => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left ^ *right).into()),

        // Bool
        (left:Bool   ,right:Bool   ) => Ok((*left ^ *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_bit_xor").into()),
    }

    op_bit_and => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left & *right).into()),

        // Bool
        (left:Bool   ,right:Bool   ) => Ok((*left & *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_bit_and").into()),
    }

    op_ne => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left != *right).into()),
        (left:Integer ,right:Float  ) => Ok((*left as f64 != *right).into()),

        // Float
        (left:Float   ,right:Float  ) => Ok((*left != *right).into()),
        (left:Float   ,right:Integer) => Ok((*left != *right as Float).into()),

        // Bool
        (left:Bool    ,right:Bool   ) => Ok((*left != *right).into()),

        // default
        (_,_) => Ok((true).into()),
    }

    op_eq => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left == *right).into()),
        (left:Integer ,right:Float  ) => Ok((*left as f64 == *right).into()),

        // Float
        (left:Float   ,right:Float  ) => Ok((*left == *right).into()),
        (left:Float   ,right:Integer) => Ok((*left == *right as Float).into()),

        // Bool
        (left:Bool    ,right:Bool   ) => Ok((*left == *right).into()),

        // default
        (_,_) => Ok((false).into()),
    }

    op_lt => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left < *right).into()),
        (left:Integer ,right:Float  ) => Ok(((*left as Float) < *right).into()),

        // Float
        (left:Float   ,right:Float  ) => Ok((*left < *right).into()),
        (left:Float   ,right:Integer) => Ok((*left < *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_lt").into()),
    }

    op_gt => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left > *right).into()),
        (left:Integer ,right:Float  ) => Ok((*left as f64 > *right).into()),

        // Float
        (left:Float   ,right:Float  ) => Ok((*left > *right).into()),
        (left:Float   ,right:Integer) => Ok((*left > *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_gt").into()),
    }

    op_le => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left <= *right).into()),
        (left:Integer ,right:Float  ) => Ok((*left as f64 <= *right).into()),

        // Float
        (left:Float   ,right:Float  ) => Ok((*left <= *right).into()),
        (left:Float   ,right:Integer) => Ok((*left <= *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_le").into()),
    }

    op_ge => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left >= *right).into()),
        (left:Integer ,right:Float  ) => Ok((*left as f64 >= *right).into()),

        // Float
        (left:Float   ,right:Float  ) => Ok((*left >= *right).into()),
        (left:Float   ,right:Integer) => Ok((*left >= *right as Float).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_ge").into()),
    }

    op_l_mov => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left << *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_l_mov").into()),
    }

    op_r_mov => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left >> *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_r_mov").into()),
    }

    op_add => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left + *right).into()),
        (left:Integer ,right:Float  ) => Ok(((*left as Float) + *right).into()),

        // Float
        (left:Float   ,right:Integer) => Ok((*left + *right as Float).into()),
        (left:Float   ,right:Float  ) => Ok((*left + *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_add").into()),
    }

    op_sub => {
        // Integer
        (left:Integer ,right:Integer) => Ok((*left - *right).into()),
        (left:Integer ,right:Float  ) => Ok(((*left as Float) - *right).into()),

        // Float
        (left:Float   ,right:Integer) => Ok((*left - *right as Float).into()),
        (left:Float   ,right:Float  ) => Ok((*left - *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_sub").into()),
    }

    op_mul => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left * *right).into()),
        (left:Integer,right:Float  ) => Ok(((*left as Float) * *right).into()),
        // Float
        (left:Float,right:Integer  ) => Ok((*left * *right as Float).into()),
        (left:Float,right:Float    ) => Ok((*left * *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_mul").into()),
    }

    op_div => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left / *right).into()),
        (left:Integer,right:Float  ) => Ok(((*left as Float) / *right).into()),
        // Float
        (left:Float,right:Integer  ) => Ok((*left / *right as Float).into()),
        (left:Float,right:Float    ) => Ok((*left / *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_div").into()),
    }

    op_mod => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left % *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_mod").into()),
    }

    op_fact => {
        // Integer
        (left:Integer,right:Integer) => Ok((*left % *right).into()),

        //default
        (_,_) => Err(UnsupportedOp::new("op_fact").into()),
    }
}

mut_bin_op_def!{
    mut op_assign => {
        (_,_) => Err(UnsupportedOp::new("op_assign").into()),
    }
}

pub trait RegTy{
    type Output;
    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output>;

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output>{
        Err(MutabilityError::new().into())
    }
}

#[mux]
pub enum RegType{
    InlineInteger(InlineInteger<true>),
    InlineFloat(InlineFloat<true>),
    InlineBool(InlineBool<true>),

    ConstInlineInteger(InlineInteger<false>),
    ConstInlineFloat(InlineFloat<false>),
    ConstInlineBool(InlineBool<false>),

    RefInteger(RefInteger<true>),
    RefFloat(RefFloat<true>),
    RefBool(RefBool<true>),

    ConstRefInteger(RefInteger<false>),
    ConstRefFloat(RefFloat<false>),
    ConstRefBool(RefBool<false>),

    RefNil(RefNil<true>),
    ConstRefNil(RefNil<false>),
}

pub struct InlineInteger<const MUTABLE:Bool>(Integer);

pub struct InlineFloat<const MUTABLE:Bool>(Float);

pub struct InlineBool<const MUTABLE:Bool>(Bool);


pub struct RefInteger<const MUTABLE:Bool>(RefCount<Integer>);

pub struct RefFloat<const MUTABLE:Bool>(RefCount<Float>);

pub struct RefBool<const MUTABLE:Bool>(RefCount<Bool>);


pub struct ConstInteger(Integer);

pub struct ConstFloat(Float);

pub struct ConstBool(Bool);


pub struct RefNil<const MUTABLE:Bool>;

impl<const MUTABLE:bool> InlineInteger<MUTABLE>{
    fn new(val:Integer)->Self{
        Self(val)
    }
}

impl<const MUTABLE:bool> RegTy for InlineInteger<MUTABLE>{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output> {
        if MUTABLE{
            Ok(&mut self.0)
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl<const MUTABLE:bool> InlineFloat<MUTABLE>{
    fn new(val:Float)->Self{
        Self(val)
    }
}

impl<const MUTABLE:bool> RegTy for InlineFloat<MUTABLE>{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output> {
        if MUTABLE{
            Ok(&mut self.0)
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl<const MUTABLE:bool> InlineBool<MUTABLE>{
    fn new(val:Bool)->Self{
        Self(val)
    }
}

impl<const MUTABLE:bool> RegTy for InlineBool<MUTABLE>{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output> {
        if MUTABLE{
            Ok(&mut self.0)
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl<const MUTABLE:bool> RegTy for RefInteger<MUTABLE>{
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&*self.0)
    }

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output> {
        if MUTABLE{
            Ok(&mut *self.0)
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl<const MUTABLE:bool> RegTy for RefFloat<MUTABLE>{
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&*self.0)
    }

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output> {
        if MUTABLE{
            Ok(&mut *self.0)
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl<const MUTABLE:bool> RegTy for RefBool<MUTABLE>{
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&*self.0)
    }

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output> {
        if MUTABLE{
            Ok(&mut *self.0)
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl RegTy for ConstInteger {
    type Output = Integer;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

impl RegTy for ConstFloat {
    type Output = Float;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

impl RegTy for ConstBool {
    type Output = Bool;

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Ok(&self.0)
    }
}

impl<const MUTABLE:bool> RegTy for RefNil<MUTABLE>{
    type Output = ();

    #[inline(always)]
    fn unbox_const(&self) -> Result<&Self::Output> {
        Err(DerefNull::new().into())
    }

    #[inline(always)]
    fn unbox_mut(&mut self) -> Result<&mut Self::Output> {
        Err(DerefNull::new().into())
    }
}

#[test]
fn test(){
    println!("1+1={:?}",op_or(&0.0,&0.0));
    println!("1+1={:?}",op_or(&0i64,&0i64));
    let a = RegType::InlineInteger(InlineInteger(1));
    println!("1+1={:?}",op_bit_or(&0,&0));
    println!("1+1={:?}",op_assign(&mut 0,&0));

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
