// use crate::types::{Const, Value};
//
// pub type ConstAddr = u16;
//
// #[derive(Default)]
// pub struct ConstTable{
//     consts:Vec<Const>,
// }
//
// impl ConstTable{
//     pub fn new(consts:Vec<Const>)->Self{
//         Self{
//             consts
//         }
//     }
//
//     /// create object  and return it's const reference
//     #[inline(always)]
//     pub fn load_const_ref(&self, offset:ConstAddr) -> Value{
//         self.consts[offset as usize].load_const_ref()
//     }
//
//     /// create object  and return it's mutable reference
//     #[inline(always)]
//     pub fn load_mut_ref(&self,offset:ConstAddr) -> Value{
//         self.consts[offset as usize].load_mut_ref()
//     }
//
//     /// create constant
//     #[inline(always)]
//     pub fn load_const(&self, offset:ConstAddr) -> Value {
//         self.consts[offset as usize].load_const()
//     }
// }