#![feature(unboxed_closures,fn_traits,ptr_metadata,auto_traits,negative_impls)]
#![feature(specialization)]


use crate::opcode::OpCode;
use crate::stack::VmStack;
use crate::vm::VM;

///
/// 虚拟机结构：
/// - 栈
/// - Self指针寄存器
/// - 结果寄存器
/// - 垃圾收集器
/// - 指令译码器
/// - 字符串常量表
/// - 外部对象表
///
/// **编址设置**
///
///
/// **虚拟机指针寄存器**
/// - Ret      函数返回值指针
/// - StackTop 栈顶指针
/// - BaseAddr 基地址指针
/// - PC       程序执行指针
/// **虚拟机数据寄存器(R[0]~R[256])**
/// - R[0]         结果寄存器
/// - R[1]         This指针寄存器
/// - R[2]         PC指针现场保护
/// - R[3]         基地址指针现场保护
/// - R[4]~ R[255] 局部变量寄存器
/// **实例:**
/// ```
/// const func = fn(const a,const b){
///     return a + b;
/// }
/// ```
///
/// var c = func(100,200);
/// **字节码:**
/// ```
///
/// ```
///
/// **寄存器内值类型**
/// - 数据值
/// - 常量表指针
/// - 程序计数器指针
/// - 基地址指针
/// - 左引用
/// - 右引用
/// -
///
/// **程序构成部分**
/// - 常量表:   int,float,String,Array.
/// -
/// **程序加载过程**
/// preload阶段:创建外部调用对象(this指针)，链接符号
/// load阶段   :
///




// extern crate core;
//
// use crate::opcode::OpCode;
// use crate::types::Const;
// use crate::util::{allocate_value};
// use crate::vm::VM;

mod opcode;
mod errors;
mod types;
mod mem_collection;
mod util;
mod vm;
mod stack;
mod const_table;

use crate::types::Value;
// use crate::opcode::OpCode;
// use crate::stack::VmStack;
// use crate::types::Value;
// use crate::vm::VM;

fn main() {

    let mut const_table = Vec::new();
    let mut op_codes = Vec::new();

    //opcodes
    op_codes.push(OpCode::LoadAsConstRef(0,0));
    op_codes.push(OpCode::LoadAsConstRef(1,1));

    op_codes.push(OpCode::And(2,0,1));
    op_codes.push(OpCode::Or(3,0,1));
    op_codes.push(OpCode::BitAnd(4,0,0));

    //consts
    const_table.push(Value::Bool(false));
    const_table.push(Value::Bool(true));

    let mut vm = VM::new(op_codes,const_table);
    vm.run();
}
