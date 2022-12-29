use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use crate::const_table::ConstTable;
use crate::opcode::OpCode;
use crate::stack::VmStack;
use crate::types::*;

use crate::errors::*;
pub struct VM {
    stack:VmStack,
    pc:usize,
    op_codes:Vec<OpCode>,
    const_table:ConstTable,
}
impl Debug for VM{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,
r#"VM:
----stack print----
{:?}
----program counter----
{}
"#
        ,self.stack,self.pc)
    }
}

impl VM{
    pub fn new(op_codes:Vec<OpCode>,const_table:Vec<Value>)->Self{
        Self {
            stack: Default::default(),
            pc: 0,
            op_codes,
            const_table:ConstTable::new(const_table)
        }
    }

    #[inline(always)]
    pub fn register(&self, reg:u8) -> &RegType{
        self.stack.register(reg)
    }

    #[inline(always)]
    pub fn register_mut(&mut self, reg:u8) -> &mut RegType{
        self.stack.register_mut(reg)
    }

    #[inline(always)]
    pub fn execute_code(&mut self,op:OpCode)->Result<()>{
        match op {
            OpCode::Or(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_or",self.register(b), self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::And(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_and",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitOr(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_bit_or",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitXor(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_bit_xor",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitAnd(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_bit_and",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::NE(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_ne",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::EQ(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_eq",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::RefEQ(a, b, c) => { unimplemented!() }
            OpCode::RefNE(_, _, _) => {unimplemented!()}
            OpCode::LT(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_lt",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::GT(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_gt",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::LE(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_le",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::GE(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_le",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::LMov(a , b, c) => {
                *self.register_mut(a) = call_bin!("op_l_mov",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::RMov(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_r_mov",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Add(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_add",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Sub(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_and",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Mul(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_mul",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Div(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_div",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Mod(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_mod",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Fact(a, b, c) => {
                *self.register_mut(a) = call_bin!("op_fact",self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitNot(a, b) => {
                *self.register_mut(a) = call_unary!("op_bit_not",self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Not(a, b) => {
                *self.register_mut(a) = call_unary!("op_not",self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Neg(a, b) => {
                *self.register_mut(a) = call_unary!("op_neg",self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Pos(a, b) => {
                *self.register_mut(a) = call_unary!("op_pos",self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            // OpCode::ArrayVisit(_, _, _) => {unimplemented!()}
            // OpCode::MemberGet(_, _, _) => {unimplemented!()}
            // OpCode::MemberSet(_, _, _) => {unimplemented!()}
            OpCode::RefAssign(a, b) => {
                unimplemented!()
            }
            OpCode::ValAssign(a, b) => {
                //todo op_assign(self.register_mut(a),self.register(b))?;
                self.pc+=1;
            }
            OpCode::JmpPrev(a,b,c)  => {
                let pos = u32::from_le_bytes([c,b,a,0]);
                self.pc -= pos as usize;
            }
            OpCode::JmpPost(a,b,c)  => {
                let pos = u32::from_le_bytes([c,b,a,0]);
                self.pc += pos as usize;
            }
            OpCode::Chk(a)  => {
                if *self.stack
                    .register(a)
                    .unbox_const()
                    .try_into_bool()?
                {
                    self.pc += 1;
                }else{
                    self.pc += 2;
                }
            }
            OpCode::Call(a) => {

            }
            // OpCode::CallConst0(b) => {unimplemented!()}
            OpCode::Ret => {unimplemented!()}

            OpCode::LoadAsMutRef(a, addr) => {
                *self.register_mut(a) = self.const_table.load_variable(addr,true);
                self.pc += 1;
            }
            OpCode::LoadAsConstRef( a, addr) => {
                *self.register_mut(a) = self.const_table.load_variable(addr,false);
                self.pc += 1;
            }
        }
        Ok(())
    }

    pub fn run(&mut self){
        loop {
            self.execute_code(self.op_codes[self.pc]).unwrap();
            //println!("{:?}",self)
        }
    }

}