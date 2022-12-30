extern crate core;

mod errors;
mod limit;

pub use errors::Result;
use std::alloc::alloc;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::Map;
use std::os::linux::raw::stat;
use std::process::id;
use ast::{Expr, Stmt};
use crate::errors::{DoubleDefine, Error, ScopeOverSize};

static mut IDENT_SCOPE_STACK:Vec<HashSet<String>> = Default::default();

pub struct VM{
    alloc:AllocStack,
}

pub struct AllocStack{
    stack:VecDeque<Frame>
}

pub enum FrameType{
    Capture,
    Inherit,
}

pub struct Frame{
    frame_type:FrameType,
    imported  :HashMap<String,usize>,
    allocated :HashMap<String,usize>,
    alloc_id  :usize,
}

impl Frame{

}



/// scope:
///     captured_variable
///     inherited

pub fn compile_(expr:Expr) {


}
pub fn compile_expr(expr:Expr){unsafe{
    match expr{
        Expr::Ident(ident) => {
            if let Some(reg) = VAR_MAP.get(ident) {

            }else{
                ALLOC_REG += VAR_MAP.insert(VAR_MAP,ALLOC_REG);
                ALLOC_REG += 1;
            }
            if VAR_MAP.get(ident).unwrap() ==  {

            }else{

            }
        }
        Expr::BinaryOp(_, _, _) => {}
        Expr::UnaryOp(_, _) => {}
        Expr::MemVisit(_, _) => {}
        Expr::FunctionCall(_, _) => {}
        Expr::IndexVisit(_, _) => {}
        Expr::Value(_) => {}
        Expr::Brace(_) => {}
    }
}}

pub fn compile_stmt(stmt:Stmt){
    match stmt{
        Stmt::IfStmt(condition, success,failure) => {

        }
        Stmt::EmptyStmt => {}
        Stmt::ValueAssignStmt(_, _) => {}
        Stmt::ReferenceAssignStmt(_, _) => {}
        Stmt::NormalStmt(_) => {}
        Stmt::WhileStmt(_, _) => {}
        Stmt::LoopStmt(_) => {}
        Stmt::BlockStmt(_) => {}
        Stmt::RefDefineStmt(_, _) => {}
        Stmt::BreakStmt => {}
        Stmt::ContinueStmt => {}
        Stmt::ReturnStmt(_) => {}
    }
}

pub fn compile(ast:ast::Stmts){
    for x in ast.stmts{

    }
}

fn main() {
    println!("Hello, world!");
}
