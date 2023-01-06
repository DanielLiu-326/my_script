#![feature(decl_macro)]

extern crate core;

mod errors;
mod limit;
mod reg_alloc;
mod mir;

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
    imported  :HashMap<String,OpReg>,
    allocated :HashMap<String,OpReg>,
    alloc     :RegisterAllocator,
}
impl Frame{
    pub fn new_capture()->Self{
        Self{
            frame_type: FrameType::Capture,
            imported: Default::default(),
            allocated: Default::default(),
            alloc: RegisterAllocator::new(2),
        }
    }

    pub fn new_inherit(parent_alloc:RegisterAllocator)->Self{
        Self{
            frame_type: FrameType::Inherit,
            imported: Default::default(),
            allocated: Default::default(),
            alloc: parent_alloc,
        }
    }

    // find the allocated register no matter is was imported or natively defined;
    pub fn find_variable(&self,ident:&str)->Option<OpReg>{
        let Some(op_reg) = self.allocated.get(ident) else{
            return self.allocated.get(ident).copied();
        };
        return Some(*op_reg);
    }

    //todo check register number
    pub fn import(&mut self,ident:&str,prev_reg:OpReg)->Result<OpReg>{
        assert!(!self.imported.contains_key(ident));
        let alloc = if let FrameType::Capture = self.frame_type{
            self.alloc.alloc()?
        }else{
            prev_reg
        };
        self.imported.insert(ident.to_string(),alloc);
        Ok(alloc)
    }
}

pub struct AllocStack{
    stack:Vec<Frame>,
}

impl AllocStack{
    pub fn push_frame(&mut self,frame_type:FrameType)->&mut Frame{
        let frame = match frame_type{
            FrameType::Capture => Frame::new_capture(),
            FrameType::Inherit => {
                if let Some(parent) = self.stack.last(){
                    Frame::new_inherit(parent.alloc.clone())
                }else{
                    Frame::new_inherit(RegisterAllocator::new(0))
                }
            },
        };
        self.stack.push(frame);
        self.stack.last_mut().unwrap()
    }

    fn get_temp(&mut self)->Result<OpReg>{
        Ok(0)
    }

    fn find_or_import(&mut self,ident:&str)->Result<OpReg>{
        let mut i = self.stack.len()-2;
        // iterate the frame to find ident
        while i > 0 {
            let frame = &self.stack[i];
            if let Some(op_reg) = frame.find_variable(ident){
                let mut prev = op_reg;
                for x in &mut self.stack[i..]{
                    prev = x.import(ident,prev)?;
                }
            }
            i-=1;
        }
        return Err(UndefIdent::new(ident.to_string()).into());
    }
}


/// scope:
///     captured_variable
///     inherited

pub struct Compiler{
    opcode:Vec<OpCode>,
    alloc:AllocStack,
}
impl Compiler{
    pub fn compile_expr_ident(&mut self,ident:&str,result:OpReg) -> Result<Vec<OpCode>> {

    }
    pub fn compile_expr_ident(&mut self,ident:&str,result:OpReg) -> Result<Vec<OpCode>>{
        let mut ret = Vec::new();
        let origin = self.alloc.find_or_import(ident)?;
        if origin != result{
            ret.push(OpCode::RefAssign(result,origin));
        }
        Ok(ret)
    }
    pub fn compile_expr_binary_op(&mut self,target:OpReg,left:OpReg,op:BinaryOp,right:Expr) ->{
        match op{
            BinaryOp::Add => {

                OpCode::Add(target,)
            }
            BinaryOp::Sub => {}
            BinaryOp::Mult => {}
            BinaryOp::Div => {}
            BinaryOp::Mod => {}
            BinaryOp::Fact => {}
            BinaryOp::EQ => {}
            BinaryOp::NE => {}
            BinaryOp::GT => {}
            BinaryOp::LT => {}
            BinaryOp::GE => {}
            BinaryOp::LE => {}
            BinaryOp::BitLMov => {}
            BinaryOp::BitRMov => {}
            BinaryOp::BitAnd => {}
            BinaryOp::BitOr => {}
            BinaryOp::BitXor => {}
            BinaryOp::RefEQ => {}
            BinaryOp::RefNE => {}
            BinaryOp::RefGT => {}
            BinaryOp::RefLT => {}
            BinaryOp::RefGE => {}
            BinaryOp::RefLE => {}
            BinaryOp::And => {}
            BinaryOp::Or => {}
        }
    }
    pub fn compile_expr_with_target(&mut self,expr:Expr,result:OpReg) ->Result<Vec<OpCode>> {
        match expr{
            Expr::Ident(_) => {}
            Expr::BinaryOp(_, _, _) => {}
            Expr::UnaryOp(_, _) => {}
            Expr::MemVisit(_, _) => {}
            Expr::FunctionCall(_, _) => {}
            Expr::IndexVisit(_, _) => {}
            Expr::Value(_) => {}
            Expr::Brace(_) => {}
        }
    }
    pub fn compile_expr(&mut self,expr:Expr)->Result<(Vec<OpCode>,OpReg)> {
        match expr{
            Expr::Ident(ident) => {
                let res = self.alloc.find_or_import(ident)?;
                return Ok((vec![],res));
            }
            Expr::BinaryOp(left, op, right) => {
                self.compile_expr_binary_op(*left,op,*right);
            }
            Expr::UnaryOp(_, _) => {}
            Expr::MemVisit(_, _) => {}
            Expr::FunctionCall(_, _) => {}
            Expr::IndexVisit(_, _) => {}
            Expr::Value(_) => {}
            Expr::Brace(_) => {}
        }
        todo!()
    }
    pub fn compile_if_stmt(&mut self,cond:Expr,success:Box<Stmt>,failure:Option<Box<Stmt>>){

    }
    pub fn compile_statement(&mut self,stmt:Stmt){
        match stmt{
            Stmt::IfStmt(cond, success, failure) => {
                self.compile_if_stmt(cond,success,failure);
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
    pub fn compile_block(&mut self,block:ast::Stmts,frame:FrameType) {
        for x in block.stmts{
            self.compile_statement(x);
        }
    }

}

fn main() {
    println!("Hello, world!");
}
