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


bin_op_def!{
    op_or  => {
        // Integer
        (left:Integer,right:Integer) -> Result<Bool> => Ok(left != 0 || right!=0),
        (left:Integer,right:Bool   ) -> Result<Bool> => Ok(left != 0 || right),

        // Bool
        (left:Bool   ,right:Integer) -> Result<Bool> => Ok(left || right!=0),
        (left:Bool   ,right:Bool   ) -> Result<Bool> => Ok(left || right),
    }
    op_and => {
        // Integer
        (left:Integer,right:Integer) -> Bool => (left != 0 && right!=0),
        (left:Integer,right:Bool   ) -> Bool => (left != 0 && right),

        // Bool
        (left:Bool   ,right:Integer) -> Bool => (left && right!=0),
        (left:Bool   ,right:Bool   ) -> Bool => (left && right),
    }
    op_bit_or => {
        // Integer
        (left:Integer,right:Integer) -> Integer => (left | right),

        // Bool
        (left:Bool   ,right:Bool   ) -> Bool    => (left | right),
    }
    op_bit_xor => {
        // Integer
        (left:Integer,right:Integer) -> Integer => (left ^ right),

        // Bool
        (left:Bool   ,right:Bool   ) -> Bool    => (left ^ right),

    }
    op_bit_and => {
        // Integer
        (left:Integer,right:Integer) -> Integer => (left & right),

        // Bool
        (left:Bool   ,right:Bool   ) -> Bool    => (left & right),
    }
    op_ne => {
        // Integer
        (left:Integer ,right:Integer) -> Bool => (left != right),
        (left:Integer ,right:Float  ) -> Bool => (left as f64 != right),

        // Float
        (left:Float   ,right:Float  ) -> Bool => (left != right),
        (left:Float   ,right:Integer) -> Bool => (left != right as Float),

        // Bool
        (left:Bool    ,right:Bool   ) -> Bool => (left != right),

        // default
        (_,_) -> Bool  => (true),
    }
    op_eq => {
        // Integer
        (left:Integer ,right:Integer) -> Bool => (left == right),
        (left:Integer ,right:Float  ) -> Bool => (left as f64 == right),

        // Float
        (left:Float   ,right:Float  ) -> Bool => (left == right),
        (left:Float   ,right:Integer) -> Bool => (left == right as Float),

        // Bool
        (left:Bool    ,right:Bool   ) -> Bool => (left == right),

        // default
        (_,_) -> Bool  => (false),
    }
    op_lt => {
        // Integer
        (left:Integer ,right:Integer) -> Bool => (left < right),
        (left:Integer ,right:Float  ) -> Bool => ((left as Float) < right),

        // Float
        (left:Float   ,right:Float  ) -> Bool => (left < right),
        (left:Float   ,right:Integer) -> Bool => (left < right as Float),
    }
    op_gt => {
        // Integer
        (left:Integer ,right:Integer) -> Bool => (left > right),
        (left:Integer ,right:Float  ) -> Bool => (left as f64 > right),

        // Float
        (left:Float   ,right:Float  ) -> Bool => (left > right),
        (left:Float   ,right:Integer) -> Bool => (left > right as Float),
    }

    op_le => {
        // Integer
        (left:Integer ,right:Integer) -> Bool => (left <= right),
        (left:Integer ,right:Float  ) -> Bool => (left as f64 <= right),

        // Float
        (left:Float   ,right:Float  ) -> Bool => (left <= right),
        (left:Float   ,right:Integer) -> Bool => (left <= right as Float),
    }
    op_ge => {
        // Integer
        (left:Integer ,right:Integer) -> Bool => (left >= right),
        (left:Integer ,right:Float  ) -> Bool => (left as f64 >= right),

        // Float
        (left:Float   ,right:Float  ) -> Bool => (left >= right),
        (left:Float   ,right:Integer) -> Bool => (left >= right as Float),
    }
    op_l_mov => {
        // Integer
        (left:Integer ,right:Integer) -> Integer => (left << right),
    }
    op_r_mov => {
        // Integer
        (left:Integer ,right:Integer) -> Integer => (left >> right),
    }
    op_add => {
        // Integer
        (left:Integer ,right:Integer) -> Integer => (left + right),
        (left:Integer ,right:Float  ) -> Float   => ((left as Float) + right),

        // Float
        (left:Float   ,right:Integer) -> Float   => (left + right as Float),
        (left:Float   ,right:Float  ) -> Float   => (left + right),
    }
    op_sub => {
        // Integer
        (left:Integer ,right:Integer) -> Integer => (left + right),
        (left:Integer ,right:Float  ) -> Float   => ((left as Float) + right),

        // Float
        (left:Float   ,right:Integer) -> Float   => (left + right as Float),
        (left:Float   ,right:Float  ) -> Float   => (left + right),
    }


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
