#![feature(unboxed_closures,fn_traits,ptr_metadata,auto_traits,negative_impls)]
#![feature(specialization)]
#![feature(adt_const_params)]
#![feature(decl_macro)]
//
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






mod opcode;
mod errors;
mod mem_collection;
mod util;
mod vm;
mod stack;
mod const_table;
mod types;


use crate::types::Value;

fn main() {

    let mut const_table = Vec::new();
    let mut op_codes = Vec::new();

    op_codes.push(OpCode::LoadAsMutRef(1,0));
    op_codes.push(OpCode::LoadAsConstRef(2,1));
    op_codes.push(OpCode::LoadAsConstRef(3,2));

    op_codes.push(OpCode::EQ(0,2,1));
    op_codes.push(OpCode::Chk(0));
    op_codes.push(OpCode::JmpPost(0,0,3));
    op_codes.push(OpCode::Add(1,1,3));
    op_codes.push(OpCode::JmpPrev(0,0,4));

    const_table.push(Value::Integer(0));
    const_table.push(Value::Integer(1_000_000_000));
    const_table.push(Value::Integer(1));

    let mut vm = VM::new(op_codes,const_table);
    vm.run();
}
