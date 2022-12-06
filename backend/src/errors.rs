use std::fmt::{Debug, Display, Formatter, write};
use crate::{limit, ScopeBase};

pub struct ScopeOverSize{}

impl ScopeOverSize{
    pub fn new()->Self{

        Self{}
    }
}
impl Debug for ScopeOverSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self,f)
    }
}
impl Display for ScopeOverSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!("no more than {} variable in a scope",limit::MAX_VARIABLE_NUM)
    }
}
impl std::error::Error for ScopeOverSize{}


pub struct DoubleDefine{
    ident:String,
}
impl DoubleDefine{
    pub fn new(ident:String)->Self{
        Self{
            ident,
        }
    }
}
impl Debug for DoubleDefine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Variable {} is double defined",self.ident.as_str())
    }
}

impl Display for DoubleDefine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Variable {} is double defined",self.ident.as_str())
    }
}

impl std::error::Error for DoubleDefine{}


pub enum Error{
    ScopeOverSize(ScopeOverSize),
    DoubleDefine(DoubleDefine)
}

pub type Result<T> = std::result::Result<T,Error>;