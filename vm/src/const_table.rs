use crate::types::{RegType, Value};

pub type ConstAddr = u16;

#[derive(Default)]
pub struct ConstTable{
    consts:Vec<Value>,
}

impl ConstTable{
    pub fn new(consts:Vec<Value>)->Self{
        Self{consts}
    }

    #[inline(always)]
    pub fn push_constant(&mut self,constant:Value){
        self.consts.push(constant)
    }

    /// create object  and return it's mutable reference
    #[inline(always)]
    pub fn load_variable(&self,offset:ConstAddr,is_mut:bool) -> RegType{
        self.consts[offset as usize].load_variable(is_mut)
    }
}