use std::fmt::{Debug, format, Formatter};
use crate::types::{RefNil, RegTy, RegType, Value};
use crate::util::UncheckMut;

pub struct VmStack{
    stack:[RegType;1000],
    bs:usize,
}

impl Default for VmStack{
    fn default() -> Self {
        let mut stack = [();1000].map(|_|{
            RegType::default()
        });
        Self{
            stack,
            bs: 0
        }
    }
}

impl VmStack{
    #[inline(always)]
    pub fn register(&self, reg:u8) -> & RegType{
        &self.stack[self.bs+reg as usize]
    }

    #[inline(always)]
    pub fn register_mut(&mut self, reg:u8) -> &mut RegType{
        &mut self.stack[self.bs+reg as usize]
    }

    #[inline(always)]
    pub fn push_frame(&mut self, frame_size:u8){
        self.bs += frame_size as usize;
    }

    #[inline(always)]
    pub fn pop_frame(&mut self, frame_size:u8){
        self.bs -= frame_size as usize;
    }
}

impl Debug for VmStack{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {unsafe{
        writeln!(f,"Stack Dbg:")?;
        for x in 0..(self.bs + 256){
            writeln!(f,"{:?}",self.stack[x].unbox_const())?
        }
        writeln!(f,"----------------------------")
    }}
}