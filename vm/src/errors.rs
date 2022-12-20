use std::fmt::{Debug, Display, Formatter};
use macros::mux;

pub struct MutabilityError;

impl MutabilityError{
    #[inline(always)]
    pub fn new()->Self{
        Self
    }
}

impl Debug for MutabilityError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Can't access object mutably with the reference")
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

pub struct DerefNull;

impl DerefNull{
    pub fn new()->Self{
        Self
    }
}

impl Debug for DerefNull {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"attempt deref on a null reference")
    }
}


#[mux]
#[derive(Debug)]
pub enum Error{
    MutabilityError(MutabilityError),
    UnsupportedOp(UnsupportedOp),
    DerefNull(DerefNull)
}

pub(crate) type Result<T> = std::result::Result<T,Error>;
