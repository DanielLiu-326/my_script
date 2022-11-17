use std::cell::UnsafeCell;
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
    pub fn register(&self,reg:u8) -> &mut Value{
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
