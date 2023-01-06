use std::collections::BinaryHeap;
use opcode::OpReg;
use super::errors::*;

#[derive(Clone)]
pub struct RegisterAllocator{
    heap:BinaryHeap<OpReg>,
    max:OpReg,
}
impl RegisterAllocator{
    pub fn new(start:OpReg)->Self{
        Self{
            heap: Default::default(),
            max: start,
        }
    }

    // todo check limit
    pub fn alloc(&mut self)->Result<OpReg>{
        Ok(self.heap.pop().or_else(||{
            let ret = self.max;
            self.max+=1;
            return Some(ret);
        }).unwrap())
    }

    pub fn dealloc(&mut self,register:OpReg){
        self.heap.push(register);
    }
}