use std::fmt::{Debug, Formatter};
use macros::mux;

pub struct MutabilityError(bool);

impl MutabilityError{
    #[inline(always)]
    pub fn new(mutable:bool)->Self{
        Self(mutable)
    }
}

impl Debug for MutabilityError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0{
            write!(f,"Can't access object mutably with the reference")
        }else{
            write!(f,"Can't access object constant with the reference")
        }
    }
}


pub struct UnsupportedOp(&'static str);

impl UnsupportedOp{
    #[inline(always)]
    pub fn new(op_name:&'static str)->Self{
        Self(op_name)
    }
}

impl Debug for UnsupportedOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"UnsupportedOp : {}",self.0)
    }
}

#[mux]
#[derive(Debug)]
pub enum Error{
    MutabilityError(MutabilityError),
    UnsupportedOp(UnsupportedOp),
}

pub(crate) type Result<T> = std::result::Result<T,Error>;
