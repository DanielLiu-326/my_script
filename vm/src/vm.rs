use crate::opcode::OpCode;
use crate::stack::VmStack;
use crate::types::Value;

pub struct VM{
    stack:VmStack,
    pc:usize,
    op_codes:Vec<OpCode>,
}

impl VM{
    pub fn new(op_codes:Vec<OpCode>)->Self{
        Self{
            stack: Default::default(),
            pc: 0,
            op_codes,
        }
    }

    #[inline]
    pub fn execute_or(&mut self,a:u8,b:u8,c:u8){

        // match(self.stack.register(a),self.stack.register(b)){
        //     (Value::RefBool(a),Value::RefBool(b))=>{
        //         self.stack.register(c) = ;
        //     }
        //     (Value::ConstBool(a),Value::Bool)
        //     _ => {
        //         unimplemented!("not support or operation between...")
        //     }
        // }
    }

    pub fn execute_code(&mut self,op:OpCode){
        match op {
            OpCode::Or(a, b, c) => {}
            OpCode::And(_, _, _) => {}
            OpCode::BitOr(_, _, _) => {}
            OpCode::BitXor(_, _, _) => {}
            OpCode::BitAnd(_, _, _) => {}
            OpCode::NE(_, _, _) => {}
            OpCode::EQ(_, _, _) => {}
            OpCode::RefEQ(_, _, _) => {}
            OpCode::RefNE(_, _, _) => {}
            OpCode::LT(_, _, _) => {}
            OpCode::GT(_, _, _) => {}
            OpCode::LE(_, _, _) => {}
            OpCode::GE(_, _, _) => {}
            OpCode::RefLT(_, _, _) => {}
            OpCode::RefGT(_, _, _) => {}
            OpCode::RefLE(_, _, _) => {}
            OpCode::RefGE(_, _, _) => {}
            OpCode::LMov(_, _, _) => {}
            OpCode::RMov(_, _, _) => {}
            OpCode::Add(_, _, _) => {}
            OpCode::Sub(_, _, _) => {}
            OpCode::Mul(_, _, _) => {}
            OpCode::Div(_, _, _) => {}
            OpCode::Mod(_, _, _) => {}
            OpCode::Fact(_, _, _) => {}
            OpCode::BitNot(_, _) => {}
            OpCode::Not(_, _) => {}
            OpCode::Neg(_, _) => {}
            OpCode::Pos(_, _) => {}
            OpCode::ArrayVisit(_, _, _) => {}
            OpCode::MemberGet(_, _, _) => {}
            OpCode::MemberSet(_, _, _) => {}
            OpCode::RefAssign(_, _) => {}
            OpCode::ValAssign(_, _) => {}
            OpCode::MovConst0(_, _) => {}
            OpCode::MovConst1(_, _) => {}
            OpCode::MovConst2(_, _) => {}
            OpCode::MovConst3(_, _) => {}
            OpCode::LoadFromConst0(_, _) => {}
            OpCode::LoadFromConst1(_, _) => {}
            OpCode::LoadFromConst2(_, _) => {}
            OpCode::LoadFromConst3(_, _) => {}
            OpCode::LoadTrue(_) => {}
            OpCode::LoadFalse(_) => {}
            OpCode::LoadPosShort(_, _) => {}
            OpCode::LoadNegShort(_, _) => {}
            OpCode::LoadNil(_) => {}
            OpCode::JmpIfPrev0(_, _) => {}
            OpCode::JmpIfPrev1(_, _) => {}
            OpCode::JmpIfPrev2(_, _) => {}
            OpCode::JmpIfPrev3(_, _) => {}
            OpCode::JmpIfPost0(_, _) => {}
            OpCode::JmpIfPost1(_, _) => {}
            OpCode::JmpIfPost2(_, _) => {}
            OpCode::JmpIfPost3(_, _) => {}
            OpCode::Call(_) => {}
            OpCode::CallConst0(_) => {}
            OpCode::Ret => {}
        }
    }


    pub fn run(&mut self){
        let mut op = self.op_codes[0].clone();
        loop {
            // 读取code

            // 执行code

        }
    }

}