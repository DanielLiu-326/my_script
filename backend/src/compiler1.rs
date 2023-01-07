use std::collections::{HashMap, HashSet};
use ast::{BinaryOp, Expr, Stmt, Stmts, Value};
use ast::Expr::FunctionCall;
use opcode::OpCode;
use crate::mir::MirCode;
use super::errors::*;

const MIR_TMP_VAR_PREFIX:&'static str = "__QwQ_TmP_vAR";
const DROP_RES:&'static str = "__QwQ_DROP";

/// Compile Ast Into Mir

pub enum FrameType{
    Capture,
    Inherit,
}

pub struct VarStackFrame{
    frame_ty:FrameType,
    imported:HashSet<String>,
    declared:HashSet<String>,
}

pub struct VarStack{
    stack:Vec<VarStackFrame>,
    ident_table:HashSet<String>,
    tmp_alloc:usize,
}
impl VarStack{
    pub fn new() ->Self{
        Self{
            stack: vec![],
            ident_table: Default::default(),
            tmp_alloc: 0,
        }
    }

    pub fn find_or_import(&mut self,ident:&str) -> Result<String>{
        todo!()
    }

    pub fn alloc_temp(&mut self)->&str{
        todo!()
    }
}

pub struct Compiler1<'input>{
    ast:Stmts<'input>,
    stack:VarStack,
    const_table:HashSet<Value<'input>>,
}
impl<'input> Compiler1{
    pub fn new(stmts:Stmts<'input>)->Self{
        Self{
            ast: stmts,
            stack: VarStack::new(),
            const_table: Default::default(),
        }
    }
    pub fn compile_expr(&mut self,expr:Expr,dst:Option<&str>)->Result<Vec<MirCode>>{
        match expr{
            Expr::Ident(src) => {
                if let Some(dst) = dst{
                    Ok(vec![MirCode::RefAssign(dst.into(),src.into())])
                }else{
                    Ok(vec![MirCode::RefAssign(dst.into(),src.into())])
                }
            }
            Expr::BinaryOp(left,op , right) => {
                let dst = dst.unwrap_or(DROP_RES).into();
                Ok(vec![MirCode::BinaryOp(op,dst,left.into(),right.into())])
            }
            Expr::UnaryOp(op,data) => {
                let dst = dst.unwrap_or(DROP_RES).into();
                Ok(vec![MirCode::UnaryOp(op,dst,data.into())])
            }
            Expr::MemVisit(left,right) => {
                todo!()
            }
            Expr::FunctionCall(callee, args) => {
                todo!()
            }
            Expr::IndexVisit(a, b) => {
                todo!()
            }
            Expr::Value(val) => {

            }
            Expr::Brace(_) => {}
        }
    }

    pub fn compile_stmt(&mut self,stmt:Stmt) ->Result<Vec<OpCode>>{
        match stmt{
            Stmt::IfStmt(cond, succsess,failure ) => {}
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

    pub fn compile(&mut self,stmts:ast::Stmts)->Result<Vec<OpCode>>{
        for x in stmts{
            self.compile_expr(x)
        }
    }
}


