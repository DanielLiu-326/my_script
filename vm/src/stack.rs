use std::cell::UnsafeCell;
use std::fmt::{Debug, format, Formatter};
use crate::types::Value;

pub struct VmStack{
    stack:UnsafeCell<[Value;65535]>,
    bs:usize,
}

impl Default for VmStack{
    fn default() -> Self {
        unsafe{
            Self{
                stack: std::mem::MaybeUninit::uninit().assume_init(),
                bs: 0
            }
        }
    }
}

impl VmStack{
    #[inline(always)]
    pub fn register(&self, reg:u8) -> &mut Value{
        unsafe {
            &mut (*self.stack.get())[self.bs+reg as usize]
        }
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
            writeln!(f,"{:?}",(*self.stack.get())[x])?
        }
        writeln!(f,"--------------")
    }}
}