// use std::ops::{Deref, DerefMut};
// use crate::const_table::ConstTable;
// use crate::opcode::OpCode;
// use crate::stack::VmStack;
// use crate::types::*;
//
// pub struct VM {
//     stack:VmStack,
//     pc:usize,
//     op_codes:Vec<OpCode>,
//     const_table:ConstTable,
// }
//
// impl VM{
//     pub fn new(op_codes:Vec<OpCode>,const_table:Vec<Const>)->Self{
//         Self{
//             stack: Default::default(),
//             pc: 0,
//             op_codes,
//             const_table:ConstTable::new(const_table)
//         }
//     }
//
//     #[inline]
//     pub fn execute_or(&mut self,a:u8,b:u8,c:u8){
//
//         // match(self.stack.register(a),self.stack.register(b)){
//         //     (Value::RefBool(a),Value::RefBool(b))=>{
//         //         self.stack.register(c) = ;
//         //     }
//         //     (Value::ConstBool(a),Value::Bool)
//         //     _ => {
//         //         unimplemented!("not support or operation between...")
//         //     }
//         // }
//     }
//
//     pub fn register(&self, reg:u8) -> &mut Value{
//         self.stack.register(reg)
//     }
//
//     pub fn execute_code(&mut self,op:OpCode){
//         match op {
//             OpCode::Or(a, b, c) => {
//                 *self.register(a) = op_or(self.register(b), self.register(c));
//                 self.pc+=1;
//             }
//             OpCode::And(a, b, c) => {
//                 *self.register(a) = op_and(self.register(b),self.register(c));
//                 self.pc+=1;
//             }
//             OpCode::BitOr(a, b, c) => {
//                 *self.register(a) = op_bit_or(self.register(b),self.register(c));
//                 self.pc+=1;
//             }
//             OpCode::BitXor(a, b, c) => {
//                 *self.register(a) = op_bit_xor(self.register(b),self.register(c));
//                 self.pc+=1;
//             }
//             OpCode::BitAnd(a, b, c) => {
//                 *self.register(a) = op_bit_and(self.register(b),self.register(c));
//                 self.pc+=1;
//             }
//             OpCode::NE(a, b, c) => {
//                 *self.register(a) = op_ne(self.register(b),self.register(c));
//                 self.pc+=1;
//             }
//             OpCode::EQ(a, b, c) => {
//                 *self.register(a) = op_eq(self.register(b),self.register(c));
//                 self.pc+=1;
//             }
//             OpCode::RefEQ(a, b, c) => {
//                 let mut res = false;
//                 if b == c {
//                     res = true;
//                 }else{
//
//                 }
//                 *self.register(a) = Value::InlineBool(res);
//                 self.pc += 1;
//             }
//             OpCode::RefNE(_, _, _) => {unimplemented!()}
//             OpCode::LT(_, _, _) => {unimplemented!()}
//             OpCode::GT(_, _, _) => {unimplemented!()}
//             OpCode::LE(_, _, _) => {unimplemented!()}
//             OpCode::GE(_, _, _) => {unimplemented!()}
//             OpCode::LMov(_, _, _) => {unimplemented!()}
//             OpCode::RMov(_, _, _) => {unimplemented!()}
//             OpCode::Add(_, _, _) => {unimplemented!()}
//             OpCode::Sub(_, _, _) => {unimplemented!()}
//             OpCode::Mul(_, _, _) => {unimplemented!()}
//             OpCode::Div(_, _, _) => {unimplemented!()}
//             OpCode::Mod(_, _, _) => {unimplemented!()}
//             OpCode::Fact(_, _, _) => {unimplemented!()}
//             OpCode::BitNot(_, _) => {unimplemented!()}
//             OpCode::Not(_, _) => {unimplemented!()}
//             OpCode::Neg(_, _) => {unimplemented!()}
//             OpCode::Pos(_, _) => {unimplemented!()}
//             // OpCode::ArrayVisit(_, _, _) => {unimplemented!()}
//             // OpCode::MemberGet(_, _, _) => {unimplemented!()}
//             // OpCode::MemberSet(_, _, _) => {unimplemented!()}
//             OpCode::RefAssign(_, _) => {unimplemented!()}
//             OpCode::ValAssign(_, _) => {unimplemented!()}
//
//             OpCode::LoadTrue(_) => {unimplemented!()}
//             OpCode::LoadFalse(_) => {unimplemented!()}
//             OpCode::LoadPosShort(_, _) => {unimplemented!()}
//             OpCode::LoadNegShort(_, _) => {unimplemented!()}
//             OpCode::LoadNil(_) => {unimplemented!()}
//             OpCode::JmpIfPrev0(_, _) => {unimplemented!()}
//             OpCode::JmpIfPrev1(_, _) => {unimplemented!()}
//             OpCode::JmpIfPrev2(_, _) => {unimplemented!()}
//             OpCode::JmpIfPrev3(_, _) => {unimplemented!()}
//             OpCode::JmpIfPost0(_, _) => {unimplemented!()}
//             OpCode::JmpIfPost1(_, _) => {unimplemented!()}
//             OpCode::JmpIfPost2(_, _) => {unimplemented!()}
//             OpCode::JmpIfPost3(_, _) => {unimplemented!()}
//             OpCode::Call(_) => {unimplemented!()}
//             OpCode::CallConst0(_) => {unimplemented!()}
//             OpCode::Ret => {unimplemented!()}
//
//             OpCode::LoadAsConst(a, addr) => {
//                 *self.register(a) = self.const_table.load_const(addr);
//                 self.pc += 1;
//             }
//             OpCode::LoadAsMutRef(a, addr) => {
//                 *self.register(a) = self.const_table.load_mut_ref(addr);
//                 self.pc += 1;
//             }
//             OpCode::LoadAsConstRef( a, addr) => {
//                 *self.register(a) = self.const_table.load_const_ref(addr);
//                 self.pc += 1;
//
//             }
//         }
//     }
//
//
//     pub fn run(&mut self){
//         loop {
//             self.execute_code(self.op_codes[self.pc]);
//             println!("{:?}",self.stack)
//         }
//     }
//
// }