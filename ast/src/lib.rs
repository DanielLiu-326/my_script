
#[derive(Debug)]
pub struct Sentences<'input>{
    pub sentences:Vec<Sentence<'input>>,
}
#[derive(Debug)]
pub enum Sentence<'input>{
    IfSentence(Expr<'input>,Box<Sentence<'input>>,Option<Box<Sentence<'input>>>),
    EmptySentence,
    ValueAssignSentence(Expr<'input>,Expr<'input>),
    ReferenceAssignSentence(Expr<'input>,Expr<'input>),
    NormalSentence(Expr<'input>),
    WhileSentence(Expr<'input>,Box<Sentence<'input>>),
    LoopSentence(Box<Sentence<'input>>),
    BlockSentence(Sentences<'input>),
    RefDefineSentence(RefDefine<'input>,Expr<'input>),
    BreakSentence,
    ContinueSentence,
    ReturnSentence(Expr<'input>),
}
#[derive(Debug)]
pub struct RefDefine<'input>{
    pub is_static:bool,
    pub is_mutable:bool,
    pub ident:&'input str,
}
#[derive(Debug)]
pub enum Expr<'input>{
    Ident(&'input str),
    BinaryOp(Box<Expr<'input>>,BinaryOp,Box<Expr<'input>>),
    UnaryOp(UnaryOp,Box<Expr<'input>>),
    MemVisit(Box<Expr<'input>>,&'input str),
    FunctionCall(Box<Expr<'input>>,ArgumentList<'input>),
    IndexVisit(Box<Expr<'input>>,Box<Expr<'input>>),
    Value(Value<'input>),
    Brace(Box<Expr<'input>>),
}
#[derive(Debug)]
pub struct ArgumentList<'input>{
    pub args:Vec<Expr<'input>>,
}
#[derive(Debug)]
pub enum UnaryOp{
    RefConst,
    BitNot,
    Not,
    Negative,
    Positive,
}
#[derive(Debug)]
pub enum BinaryOp{
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Fact,
    EQ,
    NE,
    GT,
    LT,
    GE,
    LE,

    BitLMov,
    BitRMov,
    BitAnd,
    BitOr,
    BitXor,

    RefEQ,
    RefNE,
    RefGT,
    RefLT,
    RefGE,
    RefLE,

    And,
    Or,
}

#[derive(Debug)]
pub enum Value<'input>{
    String(&'input str),
    Integer(i64),
    Float(f64),
    Nil,
    Bool(bool),
    Struct(StructFields<'input>),
    Function(Function<'input>),
    //todo string type
}
#[derive(Debug)]
pub struct Function<'input>{
    pub params:ParamList<'input>,
    pub body:Sentences<'input>,
    pub return_is_mutable:bool,
}

#[derive(Debug)]
pub struct StructFields<'input>{
    pub fields:Vec<StructField<'input>>,
}

#[derive(Debug)]
pub struct StructField<'input>{
    pub inline:bool,
    pub ref_define:RefDefine<'input>,
    pub right_expr:Expr<'input>,
}

#[derive(Debug)]
pub struct ParamList<'input>{
    pub params:Vec<RefDefine<'input>>,
}