use ast::{UnaryOp,BinaryOp};

pub struct VarDeclare{
    ident:String,
    is_static:bool,
    is_mut:bool,
}
pub enum MirCode {
    VarDeclare(VarDeclare),
    BinaryOp(BinaryOp,String,String,String),
    UnaryOp(UnaryOp,String,String),

    RefAssign(String,String),
    Assign(String,String),

    Label(usize),
    Jmp(usize),

    Call(String,Vec<String>),
    Ret(String),
}

