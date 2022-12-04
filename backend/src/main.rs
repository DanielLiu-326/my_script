use ast::{Expr, Stmt};

static mut ALLOC_REG:u8 = 0;

static mut

pub fn compile_expr(expr:Expr){
    match expr{
        Expr::Ident(ident) => {

        }
        Expr::BinaryOp(_, _, _) => {}
        Expr::UnaryOp(_, _) => {}
        Expr::MemVisit(_, _) => {}
        Expr::FunctionCall(_, _) => {}
        Expr::IndexVisit(_, _) => {}
        Expr::Value(_) => {}
        Expr::Brace(_) => {}
    }
}
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
