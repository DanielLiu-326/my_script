#![feature(ptr_metadata)]
mod opcode;

mod util;
mod base_types;
mod mem_collection;
//
// use std::alloc::{Layout};
// use std::collections::HashMap;
// use std::ffi::c_void;
// use std::process::Child;
// use std::ptr::null_mut;
// use std::sync::atomic::AtomicUsize;

// pub struct ConstantPtr{
//     ptr:usize,
// }
//
// pub struct CodeSegPtr{
//
// }
//
//
//
// pub enum StackValue {
//     ImmediateBool(bool),
//     ImmediateInteger(i64),
//     ImmediateFloat(f64),
//     ImmediateString(StringBody),
//     ImmediateArray(ArrayBody),
//     ImmediateStruct(StructBody),
//     ImmediateClosure(ClosureBody),
//
//     RefString(StringRef),
//     RefBool(BoolRef),
//     RefInteger(IntegerRef),
//     RefFloat(FloatRef),
//     RefStruct(StructRef),
//     RefArray(ArrayRef),
//     RefClosure(ClosureRef),
//
//     Nil(),
//     Function(),
//     ConstString(ConstantStringRef),
// }

fn main() {
    println!("Hello, world!");
}
