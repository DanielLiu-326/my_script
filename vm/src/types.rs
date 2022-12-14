use paste::paste;
use overloadf::overload;
use crate::errors::*;

/// Implements the function for ops
macro_rules! bin_op_impl {
    ($op_name:ident,(_,_) -> $ret_ty:ty => $expr:expr) => {paste!{
        #[overload]
        pub fn $op_name<Left,Right>(_:Left,_:Right) -> $ret_ty
            where (Left,Right):[<__ $op_name:camel:upper>]
        {
            $expr
        }
    }};

    ($op_name:ident,($left_var:ident:$left_ty:ty,$right_var:ident:$right_ty:ty) -> $ret_ty:ty => $expr:expr) => {paste!{
        impl ![<__ $op_name:camel:upper>] for ($left_ty,$right_ty){}

        #[overload]
        pub fn $op_name($left_var:$left_ty,$right_var:$right_ty) -> $ret_ty
        {
            $expr
        }
    }}
}

macro_rules! bin_op_def {
    ( $($op_name:ident =>  { $($args:tt -> $ret_ty:ty => $body:expr ,)* })* ) => {paste!{
        $(
            auto trait [<__ $op_name:camel:upper>]{}
            $(
                bin_op_impl!{$op_name,$args -> $ret_ty => $body}
            )*
        )*
    }}
}

///
/// value types
///

type Integer = i64;
type Float   = f64;
type Bool    = bool;

/* operations */

bin_op_def!{
    op_or  => {
        // Integer
        (left:Integer,right:Integer) -> Result<Bool> => Ok(left != 0 || right!=0),
        (left:Integer,right:Bool   ) -> Result<Bool> => Ok(left != 0 || right),

        // Bool
        (left:Bool   ,right:Integer) -> Result<Bool> => Ok(left || right!=0),
        (left:Bool   ,right:Bool   ) -> Result<Bool> => Ok(left || right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_or").into()),
    }

    op_and => {
        // Integer
        (left:Integer,right:Integer) -> Result<Bool> => Ok(left != 0 && right!=0),
        (left:Integer,right:Bool   ) -> Result<Bool> => Ok(left != 0 && right),

        // Bool
        (left:Bool   ,right:Integer) -> Result<Bool> => Ok(left && right!=0),
        (left:Bool   ,right:Bool   ) -> Result<Bool> => Ok(left && right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_and").into()),
    }

    op_bit_or => {
        // Integer
        (left:Integer,right:Integer) -> Result<Integer> => Ok(left | right),

        // Bool
        (left:Bool   ,right:Bool   ) -> Result<Bool>    => Ok(left | right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_bit_or").into()),
    }

    op_bit_xor => {
        // Integer
        (left:Integer,right:Integer) -> Result<Integer> => Ok(left ^ right),

        // Bool
        (left:Bool   ,right:Bool   ) -> Result<Bool>    => Ok(left ^ right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_bit_xor").into()),
    }

    op_bit_and => {
        // Integer
        (left:Integer,right:Integer) -> Result<Integer> => Ok(left & right),

        // Bool
        (left:Bool   ,right:Bool   ) -> Result<Bool>    => Ok(left & right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_bit_and").into()),
    }

    op_ne => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Bool> => Ok(left != right),
        (left:Integer ,right:Float  ) -> Result<Bool> => Ok(left as f64 != right),

        // Float
        (left:Float   ,right:Float  ) -> Result<Bool> => Ok(left != right),
        (left:Float   ,right:Integer) -> Result<Bool> => Ok(left != right as Float),

        // Bool
        (left:Bool    ,right:Bool   ) -> Result<Bool> => Ok(left != right),

        // default
        (_,_) -> Result<Bool>  => Ok(true),
    }

    op_eq => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Bool> => Ok(left == right),
        (left:Integer ,right:Float  ) -> Result<Bool> => Ok(left as f64 == right),

        // Float
        (left:Float   ,right:Float  ) -> Result<Bool> => Ok(left == right),
        (left:Float   ,right:Integer) -> Result<Bool> => Ok(left == right as Float),

        // Bool
        (left:Bool    ,right:Bool   ) -> Result<Bool> => Ok(left == right),

        // default
        (_,_) -> Result<Bool>  => Ok(false),
    }

    op_lt => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Bool> => Ok(left < right),
        (left:Integer ,right:Float  ) -> Result<Bool> => Ok((left as Float) < right),

        // Float
        (left:Float   ,right:Float  ) -> Result<Bool> => Ok(left < right),
        (left:Float   ,right:Integer) -> Result<Bool> => Ok(left < right as Float),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_lt").into()),
    }

    op_gt => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Bool> => Ok(left > right),
        (left:Integer ,right:Float  ) -> Result<Bool> => Ok(left as f64 > right),

        // Float
        (left:Float   ,right:Float  ) -> Result<Bool> => Ok(left > right),
        (left:Float   ,right:Integer) -> Result<Bool> => Ok(left > right as Float),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_gt").into()),
    }

    op_le => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Bool> => Ok(left <= right),
        (left:Integer ,right:Float  ) -> Result<Bool> => Ok(left as f64 <= right),

        // Float
        (left:Float   ,right:Float  ) -> Result<Bool> => Ok(left <= right),
        (left:Float   ,right:Integer) -> Result<Bool> => Ok(left <= right as Float),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_le").into()),
    }

    op_ge => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Bool> => Ok(left >= right),
        (left:Integer ,right:Float  ) -> Result<Bool> => Ok(left as f64 >= right),

        // Float
        (left:Float   ,right:Float  ) -> Result<Bool> => Ok(left >= right),
        (left:Float   ,right:Integer) -> Result<Bool> => Ok(left >= right as Float),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_ge").into()),
    }

    op_l_mov => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Integer> => Ok(left << right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_l_mov").into()),
    }

    op_r_mov => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Integer> => Ok(left >> right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_r_mov").into()),
    }

    op_add => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Integer> => Ok(left + right),
        (left:Integer ,right:Float  ) -> Result<Float>   => Ok((left as Float) + right),

        // Float
        (left:Float   ,right:Integer) -> Result<Float>   => Ok(left + right as Float),
        (left:Float   ,right:Float  ) -> Result<Float>   => Ok(left + right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_add").into()),
    }

    op_sub => {
        // Integer
        (left:Integer ,right:Integer) -> Result<Integer> => Ok(left - right),
        (left:Integer ,right:Float  ) -> Result<Float>   => Ok((left as Float) - right),

        // Float
        (left:Float   ,right:Integer) -> Result<Float>   => Ok(left - right as Float),
        (left:Float   ,right:Float  ) -> Result<Float>   => Ok(left - right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_sub").into()),
    }

    op_mul => {
        // Integer
        (left:Integer,right:Integer) -> Result<Integer> => Ok(left * right),
        (left:Integer,right:Float  ) -> Result<Float>   => Ok((left as Float) * right),
        // Float
        (left:Float,right:Integer  ) -> Result<Float>   => Ok(left * right as Float),
        (left:Float,right:Float    ) -> Result<Float>   => Ok(left * right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_mul").into()),
    }

    op_div => {
        // Integer
        (left:Integer,right:Integer) -> Result<Integer> => Ok(left / right),
        (left:Integer,right:Float  ) -> Result<Float>   => Ok((left as Float) / right),
        // Float
        (left:Float,right:Integer  ) -> Result<Float>   => Ok(left / right as Float),
        (left:Float,right:Float    ) -> Result<Float>   => Ok(left / right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_div").into()),
    }

    op_mod => {
        // Integer
        (left:Integer,right:Integer) -> Result<Integer> => Ok(left % right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_mod").into()),
    }

    op_fact => {
        // Integer
        (left:Integer,right:Integer) -> Result<Integer> => Ok(left % right),

        //default
        (_,_) -> Result<()> => Err(UnsupportedOp::new("op_fact").into()),
    }

    op_assign => {
        // todo:
        // 1.mut keyword support for macro
        // 2.implements op_assign
    }
}

#[mux]
pub enum RegType{

}

pub struct InlineInteger<const mutable:Bool>(Integer);

pub struct InlineFloat<const mutable:Bool>(Float);

pub struct InlineBool<const mutable:Bool>(Bool);

pub struct RefInteger<const mutable:Bool>()


#[test]
fn test(){
    println!("1+1={:?}",op_or(0.0,0.0));
    println!("1+1={:?}",op_or(0,0));
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
