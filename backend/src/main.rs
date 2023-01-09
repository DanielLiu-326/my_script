#![feature(decl_macro)]

extern crate core;

mod errors;
mod limit;
mod reg_alloc;
mod compiler;

use errors::Result;
use std::alloc::alloc;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::identity;
use std::iter::Map;
use std::os::linux::raw::stat;
use std::process::id;
use ast::{BinaryOp, Expr, Stmt};
use opcode::{OpCode, OpReg};
use crate::errors::{DoubleDefine, Error, ScopeOverSize, UndefIdent};
use crate::reg_alloc::RegisterAllocator;

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
