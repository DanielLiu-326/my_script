use std::collections::HashMap;
use std::str::FromStr;
use ast;
pub use my_parser::*;

peg::parser!{ grammar my_parser() for str{
    rule _ = [' ' | '\n']*
    //终结符定义
    rule OP_BIT_NOT()   =  "~" _
    rule OP_BIT_LMOV()  =  "<<" _
    rule OP_BIT_RMOV()  =  ">>" _
    rule OP_BIT_AND()   =  "&" _
    rule OP_BIT_OR()    =  "|" _
    rule OP_BIT_XOR()   =  "^" _

    rule OP_ADD()       =  "+" _
    rule OP_SUB()       =  "-" _
    rule OP_MULT()      =  "*" _
    rule OP_DIV()       =  "/" _
    rule OP_MOD()       =  "%" _
    rule OP_FACT()      =  "**" _

    rule OP_EQ()        =  "==" _
    rule OP_NE()        =  "!=" _
    rule OP_GT()        =  ">" _
    rule OP_LT()        =  "<" _
    rule OP_GE()        =  ">=" _
    rule OP_LE()        =  "<=" _

    rule OP_REF_EQ()    =  ":==" _
    rule OP_REF_NE()    =  ":!=" _
    rule OP_REF_GT()    =  ":>" _
    rule OP_REF_LT()    =  ":<" _
    rule OP_REF_GE()    =  ":>=" _
    rule OP_REF_LE()    =  ":<=" _

    rule OP_AND()       =  "&&" _
    rule OP_OR()        =  "||" _
    rule OP_NOT()       =  "!" _

    rule OP_ASSIGN()    =  "=" _

    rule OP_REF_ASSIGN()=  ":=" _

    rule OP_ARROW_RIGHT()   ="->" _
    // 整型常量
    rule VAL_INTEGER_HEX()->i64 =  "0x" n:$((['0'..='9']/['a'..='f']/['A'..='F'])+) _{?
        i64::from_str_radix(n,16).map_err(|e|{
            "error occured during parsing integer literal"
        })
    }

    rule VAL_INTEGER_OCT()->i64 =  "0" n:$(['0'..='9']+) _{?
        i64::from_str_radix(n,8).map_err(|e|{
            "error occured during parsing integer literal"
        })
    }

    rule VAL_INTEGER_DEC()->i64 =  n:$(['1'..='9']['0'..='9']*) _{?
        i64::from_str_radix(n,10).map_err(|e|{
            "error occured during parsing integer literal"
        })
    }

    rule VAL_INTEGER()->i64 =  n:(VAL_INTEGER_HEX()/VAL_INTEGER_OCT()/VAL_INTEGER_DEC()) _{
        n
    }

    // 浮点常量
    rule VAL_FLOAT()->f64   =  n:$(['1'..='9']['0'..='9']*("." ['0'..='9']*)?) _{
        f64::from_str(n).unwrap()
    }

    // TODO:字符串常量
    rule VAL_STR()->&'input str =  n:$("\" "[^ '\n']* "\"") _{
        n
    }

    // 布尔常量
    rule VAL_BOOL()->bool =  n:$("true"/"false") _ {
        n=="true"
    }

    // Nil常量
    rule VAL_NIL() =  "nil" _

    // 关键字
    rule KW_IF()   =  "if" _
    rule KW_ELSE() =  "else" _
    rule KW_WHILE()=  "while" _
    rule KW_BREAK()=  "break" _
    rule KW_CONTINUE()  =  "continue" _
    rule KW_IMPORT()    =  "import" _
    rule KW_RETURN()    =  "return" _
    rule KW_SELF()      =  "self" _
    rule KW_LOOP()      =  "loop" _
    rule KW_STRUCT()    = "struct" _
    rule KW_FN()        = "fn" _
    rule KW_CONST()     = "const" _
    rule KW_STATIC()    = "static" _
    rule KW_VAR()       = "var" _

    rule BRACE_S_L()    = "(" _
    rule BRACE_S_R()    = ")" _
    rule BRACE_M_L()    = "[" _
    rule BRACE_M_R()    = "]" _
    rule BRACE_L_L()    = "{" _
    rule BRACE_L_R()    = "}" _

    rule COMMA()        = "," _
    rule SEMICOLON()    = ";" _
    rule DOT()          = "." _
    rule COLON()        = ":" _
    rule DOUBLE_COLON()  = "::" _
    rule IDENT()->&'input str  = !KEY_WORDS() n:$(['a'..='z'|'_'|'A'..='Z']['a'..='z'|'A'..='Z'|'_'|'0'..='9']*) _{
        n
    }

    rule KEY_WORDS()
        = KW_IF()/KW_ELSE()/KW_WHILE()/
        KW_BREAK()/KW_CONTINUE()/KW_IMPORT()/
        KW_RETURN()/KW_SELF()/KW_LOOP()/
        KW_STRUCT()/KW_FN()/KW_CONST()/
        KW_STATIC()/KW_VAR()/VAL_NIL()/VAL_BOOL()

    pub rule src_file() -> ast::Stmts<'input >
        = stmts:stmts(){
        stmts
    }

    // 语句stmt定义
    rule stmts()->ast::Stmts<'input>
        = n:stmt()*{
        return ast::Stmts{
            stmts:n,
        }
    }

    rule stmt()->ast::Stmt<'input>
        = n:(if_stmt()/break_stmt()/continue_stmt()/
        assign_stmt()/assign_stmt()/ref_define_stmt()/
        normal_stmt()/empty_stmt()/while_stmt()/
        loop_stmt()/block_stmt()/return_stmt()){
        n
    }

    rule if_stmt()->ast::Stmt<'input>
        = precedence!{
        KW_IF() BRACE_S_L() condition:expr() BRACE_S_R() success_branch:stmt() KW_ELSE() failure_branch:stmt() {
            ast::Stmt::IfStmt(
                condition,
                Box::new(success_branch),
                Some(Box::new(failure_branch)),
            )
        }
        --
        KW_IF() BRACE_S_L() condition:expr() BRACE_S_R() success_branch:stmt(){
            ast::Stmt::IfStmt(
                condition,
                Box::new(success_branch),
                None
            )
        }
    }

    rule break_stmt()->ast::Stmt<'input>
        =KW_BREAK() SEMICOLON() {
        ast::Stmt::BreakStmt
    }

    rule continue_stmt()->ast::Stmt<'input>
        =KW_CONTINUE() SEMICOLON() {
        ast::Stmt::ContinueStmt
    }

    rule assign_stmt() -> ast::Stmt<'input>
        = n:(ref_assign_stmt()/value_assign_stmt()){
        n
    }

    rule ref_assign_stmt() -> ast::Stmt<'input>
        = left:expr() OP_REF_ASSIGN() right:expr() SEMICOLON(){
        ast::Stmt::ReferenceAssignStmt(left,right)
    }

    rule value_assign_stmt() -> ast::Stmt<'input>
        = left:expr() OP_ASSIGN() right:expr() SEMICOLON(){
        ast::Stmt::ReferenceAssignStmt(left,right)
    }

    rule ref_define_stmt() -> ast::Stmt<'input>
        = left:ref_define() OP_REF_ASSIGN() right:expr() SEMICOLON() {
        ast::Stmt::RefDefineStmt(left,right)
    }
    
    rule normal_stmt()->ast::Stmt<'input>
        =ex:expr() SEMICOLON(){
        ast::Stmt::NormalStmt(ex)
    }

    rule empty_stmt()->ast::Stmt<'input>
        = SEMICOLON(){
        ast::Stmt::EmptyStmt
    }

    rule while_stmt()->ast::Stmt<'input>
        = KW_WHILE() BRACE_S_L() condition:expr() BRACE_S_R()  body:stmt(){
        ast::Stmt::WhileStmt(condition,Box::new(body))
    }

    rule loop_stmt()->ast::Stmt<'input>
        = KW_LOOP() body:stmt(){
        ast::Stmt::LoopStmt(Box::new(body))
    }

    rule block_stmt()->ast::Stmt<'input>
        = BRACE_L_L() body:stmts() BRACE_L_R(){
        ast::Stmt::BlockStmt(body)
    }

    rule return_stmt()->ast::Stmt<'input>
        = KW_RETURN() ret:expr() SEMICOLON(){
        ast::Stmt::ReturnStmt(ret)
    }


    rule expr()->ast::Expr<'input>
        = precedence!{
        i:IDENT() {
            ast::Expr::Ident(i)
        }
        --
        left:(@) OP_OR() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Or,Box::new(right))
        }
        --
        left:(@) OP_AND() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::And,Box::new(right))
        }
        --
        left:(@) OP_BIT_OR() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitOr,Box::new(right))
        }
        --
        left:(@) OP_BIT_XOR() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitXor,Box::new(right))
        }
        --
        left:(@) OP_BIT_AND() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitAnd,Box::new(right))
        }
        --
        left:(@) OP_NE() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::NE,Box::new(right))
        }
        left:(@) OP_EQ() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::EQ,Box::new(right))
        }
        left:(@) OP_REF_EQ() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefEQ,Box::new(right))
        }
        left:(@) OP_REF_NE() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefNE,Box::new(right))
        }
        --
        left:(@) OP_LT() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::LT,Box::new(right))
        }

        left:(@) OP_GT() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::GT,Box::new(right))
        }

        left:(@) OP_LE() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::LE,Box::new(right))
        }

        left:(@) OP_GE() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::GE,Box::new(right))
        }

        left:(@) OP_REF_LT() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefLT,Box::new(right))
        }

        left:(@) OP_REF_GT() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefGT,Box::new(right))
        }

        left:(@) OP_REF_LT() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefLT,Box::new(right))
        }

        left:(@) OP_REF_LE() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefLE,Box::new(right))
        }

        left:(@) OP_REF_GE() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefGE,Box::new(right))
        }
        --
        left:(@) OP_BIT_LMOV() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitLMov,Box::new(right))
        }

        left:(@) OP_BIT_RMOV() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitRMov,Box::new(right))
        }
        --
        left:(@) OP_ADD() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Add,Box::new(right))
        }

        left:(@) OP_SUB() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Sub,Box::new(right))
        }
        --
        left:(@) OP_MULT() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Mult,Box::new(right))
        }

        left:(@) OP_DIV() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Div,Box::new(right))
        }

        left:(@) OP_MOD() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Mod,Box::new(right))
        }
        --
        left:(@) OP_FACT() right:@{
            ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Fact,Box::new(right))
        }
        --
        //unary
        OP_BIT_NOT() right:(@){
            ast::Expr::UnaryOp(ast::UnaryOp::BitNot,Box::new(right))
        }
        OP_NOT() right:(@){
            ast::Expr::UnaryOp(ast::UnaryOp::Not,Box::new(right))
        }
        OP_SUB() right:(@){
            ast::Expr::UnaryOp(ast::UnaryOp::Negative,Box::new(right))
        }
        OP_ADD() right:(@){
            ast::Expr::UnaryOp(ast::UnaryOp::Positive,Box::new(right))
        }
        //函数调用
        function:@ BRACE_S_L() arguments:expr()* BRACE_S_R(){
            ast::Expr::FunctionCall(Box::new(function),ast::ArgumentList{
                args:arguments
            })
        }
        //索引访问
        left:@ BRACE_M_L() index:expr() BRACE_M_R(){
            ast::Expr::IndexVisit(Box::new(left),Box::new(index))
        }
        --
        //成员访问运算符
        left:@ DOT() ident:IDENT(){
            ast::Expr::MemVisit(Box::new(left),ident)
        }
        --
        //各种值
        n:value(){
            ast::Expr::Value(n)
        }
    }

    rule value()->ast::Value<'input>
        = n:(val_integer()/val_float()/val_struct()/val_nil()/val_bool()/val_str()/val_function()){
        n
    }

    rule val_str()->ast::Value<'input>
        = val:VAL_STR(){
        ast::Value::String(val)
    }

    rule val_bool()->ast::Value<'input>
    = val:VAL_BOOL(){
        ast::Value::Bool(val)
    }

    rule val_nil()->ast::Value<'input>
        = VAL_NIL(){
        ast::Value::Nil
    }

    rule val_integer()->ast::Value<'input>
        = value:VAL_INTEGER(){
        ast::Value::Integer(value)
    }

    rule val_float()->ast::Value<'input>
        = value:VAL_FLOAT(){
        ast::Value::Float(value)
    }

    rule val_struct()->ast::Value<'input>
        = KW_STRUCT() BRACE_L_L() fields:struct_fields() BRACE_L_R(){
        ast::Value::Struct(ast::StructFields{
            fields:fields.fields,
        })
    }

    rule struct_fields() -> ast::StructFields<'input>
        = fields:struct_field() ** COMMA() COMMA()?{
        ast::StructFields{
            fields,
        }
    }

    rule struct_field()->ast::StructField<'input>
        = ref_define:ref_define() COLON() right_expr:expr(){
        ast::StructField{
            inline:false,
            ref_define,
            right_expr
        }
    }/ref_define:ref_define() DOUBLE_COLON() right_expr:expr(){
        ast::StructField{
            inline:true,
            ref_define,
            right_expr
        }
    }

    rule ref_define() -> ast::RefDefine<'input>
        = KW_STATIC() n:non_static_ref_define() {
            let mut b = n;
            b.is_static = true;
            b
        }
        /n:non_static_ref_define(){
            n
        }

    rule non_static_ref_define()->ast::RefDefine<'input>
        = KW_CONST() ident:IDENT(){
            ast::RefDefine{
                is_static:false,
                is_mutable:false,
                ident,
            }
        }
        /KW_VAR() ident:IDENT(){
            ast::RefDefine{
                is_static:false,
                is_mutable:true,
                ident,
            }
        }

    rule val_function() -> ast::Value<'input>
        = KW_FN() BRACE_S_L() params:param_list() BRACE_S_R() OP_ARROW_RIGHT() return_is_mutable:mutability_def() BRACE_L_L() body:stmts() BRACE_L_R() {
        ast::Value::Function(ast::Function{
            params,
            body,
            return_is_mutable,
        })
    }

    rule mutability_def() -> bool
        = KW_VAR(){
        return true;
    }/KW_CONST(){
        return false;
    }

    rule param_list()->ast::ParamList<'input>
        = ref_defines:ref_define() ** COMMA() {
        ast::ParamList{
            params:ref_defines
        }
    }

}}
// peg::parser!( grammar frontend() for str {
//
//     rule _ = [' ' | '\n']*
//     //终结符定义
//     rule OP_BIT_NOT()   =  "~" _
//     rule OP_BIT_LMOV()  =  "<<" _
//     rule OP_BIT_RMOV()  =  ">>" _
//     rule OP_BIT_AND()   =  "&" _
//     rule OP_BIT_OR()    =  "|" _
//     rule OP_BIT_XOR()   =  "^" _
//
//     rule OP_ADD()       =  "+" _
//     rule OP_SUB()       =  "-" _
//     rule OP_MULT()      =  "*" _
//     rule OP_DIV()       =  "/" _
//     rule OP_MOD()       =  "%" _
//     rule OP_FACT()      =  "**" _
//
//     rule OP_EQ()        =  "==" _
//     rule OP_NE()        =  "!=" _
//     rule OP_GT()        =  ">" _
//     rule OP_LT()        =  "<" _
//     rule OP_GE()        =  ">=" _
//     rule OP_LE()        =  "<=" _
//
//     rule OP_REF_EQ()    =  ":==" _
//     rule OP_REF_NE()    =  ":!=" _
//     rule OP_REF_GT()    =  ":>" _
//     rule OP_REF_LT()    =  ":<" _
//     rule OP_REF_GE()    =  ":>=" _
//     rule OP_REF_LE()    =  ":<=" _
//
//     rule OP_AND()       =  "&&" _
//     rule OP_OR()        =  "||" _
//     rule OP_NOT()       =  "!" _
//
//     rule OP_ASSIGN()    =  "=" _
//
//     rule OP_REF_ASSIGN()=  ":=" _
//
//     rule OP_ARROW_RIGHT()   ="->" _
//     rule VAL_INTEGER_HEX()->i64 =  "0x" n:$((['0'..='9']/['a'..='f']/['A'..='F'])+) _{?
//         i64::from_str_radix(n,16).map_err(|e|{
//             "error occured during parsing integer literal"
//         })
//     }
//     rule VAL_INTEGER_OCT()->i64 =  "0" n:$(['0'..='9']+) _{?
//         i64::from_str_radix(n,8).map_err(|e|{
//             "error occured during parsing integer literal"
//         })
//     }
//     rule VAL_INTEGER_DEC()->i64 =  n:$(['1'..='9']['0'..='9']*) _{?
//         i64::from_str_radix(n,10).map_err(|e|{
//             "error occured during parsing integer literal"
//         })
//     }
//     rule VAL_INTEGER()->i64 =  n:(VAL_INTEGER_HEX()/VAL_INTEGER_OCT()/VAL_INTEGER_DEC()) _{
//         n
//     }
//     rule VAL_FLOAT()->f64   =  n:$(['1'..='9']['0'..='9']*("." ['0'..='9']*)?) _{
//         f64::from_str(n).unwrap()
//     }
//     //todo matching string has problem
//     rule VAL_STR()->&'input str =  n:$("\" "[^ '\n']* "\"") _{
//         n
//     }
//
//     rule VAL_NIL() =  "nil" _
//
//     rule VAL_BOOL()->bool =  n:$("true"/"false") _ {
//         n=="true"
//     }
//
//     rule KW_IF()   =  "if" _
//     rule KW_ELSE() =  "else" _
//     rule KW_WHILE()=  "while" _
//     rule KW_BREAK()=  "break" _
//     rule KW_CONTINUE()  =  "continue" _
//     rule KW_IMPORT()    =  "import" _
//     rule KW_RETURN()    =  "return" _
//     rule KW_SELF()      =  "self" _
//     rule KW_LOOP()      =  "loop" _
//     rule KW_STRUCT()    = "struct" _
//     rule KW_FN()        = "fn" _
//     rule KW_CONST()     = "const" _
//     rule KW_STATIC()    = "static" _
//     rule KW_VAR()       = "var" _
//
//     rule KEY_WORDS()
//         = KW_IF()/KW_ELSE()/KW_WHILE()/
//         KW_BREAK()/KW_CONTINUE()/KW_IMPORT()/
//         KW_RETURN()/KW_SELF()/KW_LOOP()/
//         KW_STRUCT()/KW_FN()/KW_CONST()/
//         KW_STATIC()/KW_VAR()/VAL_NIL()/VAL_BOOL()
//
//     rule BRACE_S_L()    = "(" _
//     rule BRACE_S_R()    = ")" _
//     rule BRACE_M_L()    = "[" _
//     rule BRACE_M_R()    = "]" _
//     rule BRACE_L_L()    = "{" _
//     rule BRACE_L_R()    = "}" _
//
//     rule COMMA()        = "," _
//     rule SEMICOLON()    = ";" _
//     rule DOT()          = "." _
//     rule COLON()        = ":" _
//     rule DOUBLE_COLON()  = "::" _
//     rule IDENT()->&'input str  = !KEY_WORDS() n:$(['a'..='z'|'_'|'A'..='Z']['a'..='z'|'A'..='Z'|'_'|'0'..='9']*) _{
//         n
//     }
//     ////////////////////////////////////program/////////////////////////////////////////////////
//
//     pub rule program() -> ast::Sentences<'input >
//         = sentences:sentences(){
//         sentences
//     }
//
//     rule sentences()->ast::Sentences<'input>
//         = n:sentence()*{
//         return ast::Sentences{
//             sentences:n,
//         }
//     }
//     // rule sentence()->ast::Sentence
//     //     = $(if_sentence()/empty_sentence()/assign_sentence()/
//     //     normal_sentence()/circle_sentence()/block_sentence()/
//     //     ref_define_sentence()/break_sentence()/continue_sentence())
//
//     rule sentence()->ast::Sentence<'input>
//         = n:(if_sentence()/break_sentence()/continue_sentence()/
//         assign_sentence()/assign_sentence()/ref_define_sentence()/
//         normal_sentence()/empty_sentence()/while_sentence()/
//         loop_sentence()/block_sentence()/return_sentence()){
//         n
//     }
//
//
//
//     rule if_sentence()->ast::Sentence<'input>
//         = precedence!{
//         KW_IF() BRACE_S_L() condition:expr() BRACE_S_R() success_branch:sentence() KW_ELSE() fail_branch:sentence() {
//             ast::Sentence::IfSentence(
//                 condition,
//                 Box::new(success_branch),
//                 Some(Box::new(fail_branch)),
//             )
//         }
//         --
//         KW_IF() BRACE_S_L() condition:expr() BRACE_S_R() success_branch:sentence(){
//             ast::Sentence::IfSentence(
//                 condition,
//                 Box::new(success_branch),
//                 None
//             )
//         }
//     }
//
//     rule break_sentence()->ast::Sentence<'input>
//         =KW_BREAK() SEMICOLON() {
//         ast::Sentence::BreakSentence
//     }
//     rule continue_sentence()->ast::Sentence<'input>
//         =KW_CONTINUE() SEMICOLON() {
//         ast::Sentence::ContinueSentence
//     }
//     rule assign_sentence() -> ast::Sentence<'input>
//         = n:(ref_assign_sentence()/value_assign_sentence()){
//         n
//     }
//     rule ref_assign_sentence() -> ast::Sentence<'input>
//         = left:expr() OP_REF_ASSIGN() right:expr() SEMICOLON(){
//         ast::Sentence::ReferenceAssignSentence(left,right)
//     }
//
//     rule value_assign_sentence() -> ast::Sentence<'input>
//         = left:expr() OP_ASSIGN() right:expr() SEMICOLON(){
//         ast::Sentence::ReferenceAssignSentence(left,right)
//     }
//
//     rule ref_define_sentence() -> ast::Sentence<'input>
//         = left:ref_define() OP_REF_ASSIGN() right:expr() SEMICOLON() {
//         ast::Sentence::RefDefineSentence(left,right)
//     }
//
//     rule ref_define() -> ast::RefDefine<'input>
//         = KW_STATIC() n:non_static_ref_define() {
//             let mut b = n;
//             b.is_static = true;
//             b
//         }
//         /n:non_static_ref_define(){
//             n
//         }
//
//     rule non_static_ref_define()->ast::RefDefine<'input>
//         = KW_CONST() ident:IDENT(){
//             ast::RefDefine{
//                 is_static:false,
//                 is_mutable:false,
//                 ident,
//             }
//         }
//         /KW_VAR() ident:IDENT(){
//             ast::RefDefine{
//                 is_static:false,
//                 is_mutable:true,
//                 ident,
//             }
//         }
//
//     rule normal_sentence()->ast::Sentence<'input>
//         =ex:expr() SEMICOLON(){
//         ast::Sentence::NormalSentence(ex)
//     }
//
//     rule empty_sentence()->ast::Sentence<'input>
//         = SEMICOLON(){
//         ast::Sentence::EmptySentence
//     }
//
//     rule while_sentence()->ast::Sentence<'input>
//         = KW_WHILE() BRACE_S_L() condition:expr() BRACE_S_R()  body:sentence(){
//         ast::Sentence::WhileSentence(condition,Box::new(body))
//     }
//
//     rule loop_sentence()->ast::Sentence<'input>
//         = KW_LOOP() body:sentence(){
//         ast::Sentence::LoopSentence(Box::new(body))
//     }
//
//     rule block_sentence()->ast::Sentence<'input>
//         = BRACE_L_L() body:sentences() BRACE_L_R(){
//         ast::Sentence::BlockSentence(body)
//     }
//
//     rule return_sentence()->ast::Sentence<'input>
//         = KW_RETURN() ret:expr() SEMICOLON(){
//         ast::Sentence::ReturnSentence(ret)
//     }
//
//     rule expr()->ast::Expr<'input>
//         = precedence!{
//         i:IDENT() {
//             ast::Expr::Ident(i)
//         }
//         --
//         left:(@) OP_OR() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Or,Box::new(right))
//         }
//         --
//         left:(@) OP_AND() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::And,Box::new(right))
//         }
//         --
//         left:(@) OP_BIT_OR() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitOr,Box::new(right))
//         }
//         --
//         left:(@) OP_BIT_XOR() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitXor,Box::new(right))
//         }
//         --
//         left:(@) OP_BIT_AND() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitAnd,Box::new(right))
//         }
//         --
//         left:(@) OP_NE() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::NE,Box::new(right))
//         }
//         left:(@) OP_EQ() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::EQ,Box::new(right))
//         }
//         left:(@) OP_REF_EQ() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefEQ,Box::new(right))
//         }
//         left:(@) OP_REF_NE() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefNE,Box::new(right))
//         }
//         --
//         left:(@) OP_LT() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::LT,Box::new(right))
//         }
//
//         left:(@) OP_GT() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::GT,Box::new(right))
//         }
//
//         left:(@) OP_LE() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::LE,Box::new(right))
//         }
//
//         left:(@) OP_GE() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::GE,Box::new(right))
//         }
//
//         left:(@) OP_REF_LT() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefLT,Box::new(right))
//         }
//
//         left:(@) OP_REF_GT() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefGT,Box::new(right))
//         }
//
//         left:(@) OP_REF_LT() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefLT,Box::new(right))
//         }
//
//         left:(@) OP_REF_LE() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefLE,Box::new(right))
//         }
//
//         left:(@) OP_REF_GE() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::RefGE,Box::new(right))
//         }
//         --
//         left:(@) OP_BIT_LMOV() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitLMov,Box::new(right))
//         }
//
//         left:(@) OP_BIT_RMOV() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::BitRMov,Box::new(right))
//         }
//         --
//         left:(@) OP_ADD() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Add,Box::new(right))
//         }
//
//         left:(@) OP_SUB() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Sub,Box::new(right))
//         }
//         --
//         left:(@) OP_MULT() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Mult,Box::new(right))
//         }
//
//         left:(@) OP_DIV() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Div,Box::new(right))
//         }
//
//         left:(@) OP_MOD() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Mod,Box::new(right))
//         }
//         --
//         left:(@) OP_FACT() right:@{
//             ast::Expr::BinaryOp(Box::new(left),ast::BinaryOp::Fact,Box::new(right))
//         }
//         --
//         //unary
//         OP_BIT_NOT() right:(@){
//             ast::Expr::UnaryOp(ast::UnaryOp::BitNot,Box::new(right))
//         }
//         OP_NOT() right:(@){
//             ast::Expr::UnaryOp(ast::UnaryOp::Not,Box::new(right))
//         }
//         OP_SUB() right:(@){
//             ast::Expr::UnaryOp(ast::UnaryOp::Negative,Box::new(right))
//         }
//         OP_ADD() right:(@){
//             ast::Expr::UnaryOp(ast::UnaryOp::Positive,Box::new(right))
//         }
//         //函数调用
//         function:@ BRACE_S_L() arguments:expr()* BRACE_S_R(){
//             ast::Expr::FunctionCall(Box::new(function),ast::ArgumentList{
//                 args:arguments
//             })
//         }
//         //索引访问
//         left:@ BRACE_M_L() index:expr() BRACE_M_R(){
//             ast::Expr::IndexVisit(Box::new(left),Box::new(index))
//         }
//         --
//         //成员访问运算符
//         left:@ DOT() ident:IDENT(){
//             ast::Expr::MemVisit(Box::new(left),ident)
//         }
//         --
//         //各种值
//         n:value(){
//             ast::Expr::Value(n)
//         }
//     }
//     rule value()->ast::Value<'input>
//         = n:(val_integer()/val_float()/val_struct()/val_nil()/val_bool()/val_str()/val_function()){
//         n
//     }
//     rule val_str()->ast::Value<'input>
//         = val:VAL_STR(){
//         ast::Value::String(val)
//     }
//     rule val_bool()->ast::Value<'input>
//     = val:VAL_BOOL(){
//         ast::Value::Bool(val)
//     }
//     rule val_nil()->ast::Value<'input>
//         = VAL_NIL(){
//         ast::Value::Nil
//     }
//     rule val_integer()->ast::Value<'input>
//         = value:VAL_INTEGER(){
//         ast::Value::Integer(value)
//     }
//     rule val_float()->ast::Value<'input>
//         = value:VAL_FLOAT(){
//         ast::Value::Float(value)
//     }
//     rule val_struct()->ast::Value<'input>
//         = KW_STRUCT() BRACE_L_L() fields:struct_fields() BRACE_L_R(){
//         ast::Value::Struct(ast::StructFields{
//             fields:fields.fields,
//         })
//     }
//     rule struct_fields() -> ast::StructFields<'input>
//         = fields:struct_field() ** COMMA() COMMA()?{
//         ast::StructFields{
//             fields,
//         }
//     }
//     rule struct_field()->ast::StructField<'input>
//         = ref_define:ref_define() COLON() right_expr:expr(){
//         ast::StructField{
//             inline:false,
//             ref_define,
//             right_expr
//         }
//     }/ref_define:ref_define() DOUBLE_COLON() right_expr:expr(){
//         ast::StructField{
//             inline:true,
//             ref_define,
//             right_expr
//         }
//     }
//
//     rule val_function() -> ast::Value<'input>
//         = KW_FN() BRACE_S_L() params:param_list() BRACE_S_R() OP_ARROW_RIGHT() return_is_mutable:mutability_def() BRACE_L_L() body:sentences() BRACE_L_R() {
//         ast::Value::Function(ast::Function{
//             params,
//             body,
//             return_is_mutable,
//         })
//     }
//
//     rule mutability_def() -> bool
//         = KW_VAR(){
//         return true;
//     }/KW_CONST(){
//         return false;
//     }
//
//     rule param_list()->ast::ParamList<'input>
//         = ref_defines:ref_define() ** COMMA() {
//         ast::ParamList{
//             params:ref_defines
//         }
//     }
//
// });
