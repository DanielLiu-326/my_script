use std::fmt::{Debug, Display, Formatter};
use macros::mux;




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
    UnsupportedOp(UnsupportedOp),
}

pub(crate) type Result<T> = std::result::Result<T,Error>;
