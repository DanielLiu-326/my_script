#![feature(decl_macro)]

extern crate core;

mod errors;
mod limit;
mod reg_alloc;
mod compiler;


/// 0 : 结果
/// 1 : this

pub struct VM{
    alloc:AllocStack,
}

pub enum FrameType{
    Capture,
    Inherit,
}

pub struct Frame{
    frame_type:FrameType,
    imported  :HashSet<String>,
    allocated :HashSet<String>,
}


fn main() {
    println!("Hello, world!");
}
