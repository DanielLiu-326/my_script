use concept::types::errors::*;
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


pub struct TypeError(&'static str,&'static str);
impl Debug for TypeError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"expect {} ,found {}",self.1,self.0)
    }
}
impl TypeError{
    pub fn new(expect:&'static str,found:&'static str)->Self{
        Self(expect,found)
    }
}

#[mux]
#[derive(Debug)]
pub enum Error{
    MutabilityError(MutabilityError),
    UnsupportedOp(UnsupportedOp),
    DerefNull(DerefNull),
    TypeError(TypeError),
}

pub(crate) type Result<T> = std::result::Result<T,Error>;
