use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};
use std::simd::u16x32;
use super::util;
use util::ptr::thin_dyn::{Obj,Implemented};
use crate::util::ptr::thin_dyn::ObjPtrMut;


// root                  //child change, self deleted, become not root
// unstable              //child change，self deleted，become root
// reachable             //child change，self deleted，become root
// stable                //child change，self deleted，become root
// deleted
//
// 0. root become not root          //todo
//      insert into stable
// 1. unstable child change
//      not care
// 2. unstable self deleted
//      not care
// 3. unstable become root          //todo
//      insert into reachable
// 4. reachable child change        //todo
//      new child insert into reachable
// 5.reachable self deleted         //todo
//      not visited: not care
//      visiting:stopping visiting now and visit next and delete
// 6.reachable become root
//      not care
// 7.stable/root child change       //todo
//      new child into reachable
// 8.stable/root self deleted       //todo
//      not care
//
// 让root中的节点移动到reachable
// 遍历reachable，把每个节点的子节点中为unstable的节点放入到reachable中，当前节点如果是root则放入到root中，如果不是root则放入stable中
// 直到reachable中的节点清空

// 删除当前的unstable中的所有节点

// self.move_all_to(self.root,REACHABLE)
// for x in reachable{
//      for y in x.childs{
//          reachable.append(y);
//      }
//      if x.is_root(){
//          root.insert(x);
//      }else{
//          stable.insert(x);
//      }
// }

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
/// const func = fn(const a,const b){
///     return a + b;
/// }
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


type OpReg             = u8;    //寄存器编号类型
type ConstAddr         = u16;   //常量指针类型
type RelativeAddr      = u16;   //相对地址类型
pub struct SegAddr(u8,u8,u8);   //代码段指针类型

pub enum OpCode {
    /// 运算指令(结果,左操作数,右操作数)
    Or(OpReg,OpReg,OpReg),
    And(OpReg,OpReg,OpReg),
    BitOr(OpReg,OpReg,OpReg),
    BitXor(OpReg,OpReg,OpReg),
    BitAnd(OpReg,OpReg,OpReg),
    NE(OpReg,OpReg,OpReg),
    EQ(OpReg,OpReg,OpReg),
    RefEQ(OpReg,OpReg,OpReg),
    RefNE(OpReg,OpReg,OpReg),
    LT(OpReg,OpReg,OpReg),
    GT(OpReg,OpReg,OpReg),
    LE(OpReg,OpReg,OpReg),
    GE(OpReg,OpReg,OpReg),
    RefLT(OpReg,OpReg,OpReg),
    RefGT(OpReg,OpReg,OpReg),
    RefLE(OpReg,OpReg,OpReg),
    RefGE(OpReg,OpReg,OpReg),
    LMov(OpReg,OpReg,OpReg),
    RMov(OpReg,OpReg,OpReg),
    Add(OpReg,OpReg,OpReg),
    Sub(OpReg,OpReg,OpReg),
    Mul(OpReg,OpReg,OpReg),
    Div(OpReg,OpReg,OpReg),
    Mod(OpReg,OpReg,OpReg),
    Fact(OpReg,OpReg,OpReg),

    BitNot(OpReg,OpReg),
    Not(OpReg,OpReg),
    Neg(OpReg,OpReg),
    Pos(OpReg,OpReg),

    ///数组操作
    IndexVisit(OpReg,OpReg),

    ///结构体成员访问
    MemberVisit(OpReg,OpReg,OpReg),

    ///
    /// 变量创建
    ///


    //加载常量,String,Integer,Function
    MovConst0(OpReg,ConstAddr), //从常量区0加载数据
    MovConst1(OpReg,ConstAddr), //从常量区1加载数据
    MovConst2(OpReg,ConstAddr), //从常量区2加载数据
    MovConst3(OpReg,ConstAddr), //从常量区3加载数据

    //创建内嵌式Bool
    LoadTrue(OpReg),
    LoadFalse(OpReg),

    //创建内嵌式整数 -65535~65535
    LoadPosShort(OpReg,u16),
    LoadNegShort(OpReg,u16),

    //创建数组
    LoadNewArray(OpReg,u16),	//存储寄存器，初始大小

    //创建结构体
    LoadStruct(OpReg),

    //创建闭包，添加捕获变量
    CapVariable(OpReg,OpReg),

    //创建Nil
    LoadNil(OpReg),

    //相对跳跃，一次最多跳524280条指令,一个段最多有6553500条指令,最多有6553500个段
    JmpIfPrev0(OpReg,u16),
    JmpIfPrev1(OpReg,u16),
    JmpIfPrev2(OpReg,u16),
    JmpIfPrev3(OpReg,u16),

    JmpIfPost0(OpReg,u16),
    JmpIfPost1(OpReg,u16),
    JmpIfPost2(OpReg,u16),
    JmpIfPost3(OpReg,u16),




    Push(u16),                   //压入n个值
    Pop(u16),                    //弹出n个值


    ///函数调用
    /// - 压入PC寄存器
    /// - 压入基地址寄存器
    /// - 将基地址设置为栈顶-4
    Call(u8),
    CallConst0(ConstAddr),      //调用Const函数

    Ret,                         //弹出到基地址寄存器，弹出到程序计数器


}


impl OpCode{
    #[inline(always)]
    pub fn get_u24(&self) ->u32 {
        unsafe {
            *(self as *const u32) & 0x00_ff_ff_ff
        }
    }

    #[inline(always)]
    pub fn get_uncut(&self) ->u32 {
        unsafe{
            *(self as *const u32)
        }
    }

    #[inline(always)]
    pub fn set_u24(&mut self,val:u32){
        unsafe{
            *(self as *mut u32) |= val;
        }
    }

}


///所有可能出现的类型组合
pub enum RegValue{
    ///未装箱的基本类型
    InlineInteger(i64),
    InlineBool(bool),
    InlineFloat(f64),

    ///已经装箱的基本类型
    RefInteger(Ptr),
    RefBool(Ptr),
    RefFloat(Ptr),

    ConstRefInteger(Ptr),
    ConstRefBool(Ptr),
    ConstRefFloat(Ptr),

    ///对象类型
    RefArray(Ptr),
    RefDict(Ptr),
    RefStruct(Ptr),
    RefFunction(Ptr),

    ConstRefArray(Ptr),
    ConstRefDict(Ptr),
    ConstRefStruct(Ptr),
    ConstRefFunction(Ptr),

    ///Load const指令加载的常量类型
    ConstInteger(i64),
    ConstBool(bool),
    ConstFloat(i64),
    ConstString(Ptr),

    RefNil,
    ConstRefNil,

}





































//
// pub trait GcObj{
//     type Info;
//     fn try_drop(&mut self)->Self::Info;
// }
// pub trait GC{
//     fn start_gc(&self);
//
//     fn gc_steps(&self,steps:isize)->Option<usize>;
// }
//
// pub struct RcInfo {
//     count:usize,
//     body:T,
// }
//
// pub struct RcReference<B:?Sized>{
//     ptr:PtrMut<RcObject<B>>,
// }
//
// impl<B:?Sized> RcReference<B>{
//     pub fn inner(&self)->&PtrMut<RcObject<B>>{
//         &self.ptr
//     }
//     pub fn inner_mut(&mut self)->&mut PtrMut<RcObject<B>>{
//         &mut self.ptr
//     }
//     pub fn body(&self)->&B{
//         &self.ptr.deref().body
//     }
//     pub fn body_mut(&mut self) ->&mut B{
//         &mut self.ptr.deref_mut().body
//     }
//     pub fn count(&self)->usize{
//         self.ptr.count
//     }
// }
// impl <B:?Sized> Deref for RcReference<B>{
//     type Target = RcObject<B>;
//
//     fn deref(&self) -> &Self::Target {
//         self.ptr.deref()
//     }
// }
// impl <B:?Sized> DerefMut for RcReference<B>{
//
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.ptr.deref_mut()
//     }
// }
//
// impl<B:?Sized> Clone for RcReference<B>{
//     fn clone(&self) -> Self {
//         let mut ret =Self{
//             ptr:self.ptr
//         };
//         ret.deref_mut().count+=1;
//         ret
//     }
// }
// impl<B:?Sized> Drop for RcReference<B>{
//     fn drop(&mut self) {
//         unsafe{
//             self.ptr.deref_mut().count -= 1;
//             if self.ptr.deref().count==0{
//                 <PtrMut<RcObject<B>> as Into<*mut RcObject<B>>>::into(self.ptr).drop_in_place();
//             }
//         }
//     }
// }
//
//
//
// /********************************************************************************/
//
// pub trait GcBody{
//     fn is_child_scanning(&self)->bool;
//     fn enter_gc_scan(&mut self);
//     fn next_child(&mut self)->Option<GcReference<dyn GcBody>>;
//     fn leave_gc_scan(&mut self);
// }
//
//
// type GcObject<T:?Sized+GcBody> = RcObject<GcData<T>>;
//
//
// #[derive(Copy,Clone)]
// pub enum GcStateType{
//     Stable,
//     Unstable,
//     Reachable,
//     //Deleted,
// }
// pub struct GcState{
//     state:GcStateType,
//     round:usize,
// }
// impl GcState{
//     fn gc_state(&self,ctx:&GcContext)->GcStateType{
//         if self.round!= ctx.round{
//             GcStateType::Unstable
//         }else{
//             self.state
//         }
//     }
// }
//
// pub struct GcLLTag();
//
// struct GcData<T:?Sized+GcBody> {
//     gc_busy:bool,
//
//     gc_root_count:usize,
//
//     obj_state:GcState,
//
//     gc:PtrMut<GarbageCollector>,
//
//     gc_ll_data:NodeExtraData<GcLLTag,dyn GcBody>,
//
//     body:T
// }
//
// impl<T:?Sized+GcBody> GcData<T>{
//     fn is_root(&self)->bool{
//         self.gc_root_count!=0
//     }
// }
//
//
//
//
// pub struct GcReference<B:?Sized+GcBody>{
//     is_gc_root:bool,
//     rc_ptr:RcReference<GcData<B>>,
// }
//
//
// impl<B:?Sized+GcBody> Drop for GcReference<B>{
//     fn drop(&mut self) {
//         if self.is_gc_root{
//             self.rc_ptr.body_mut().gc_root_count-=1;
//         }
//         //todo remove from
//     }
// }
//
//
// impl<T:?Sized+GcBody> Clone for GcReference<T>{
//     fn clone(&self) -> Self {
//         if self.is_gc_root{
//             self.rc_ptr.ptr.body.gc_root_count+=1;
//         }
//         Self{
//             is_gc_root: self.is_gc_root,
//             rc_ptr: self.rc_ptr.clone(),
//         }
//     }
// }
//
//
// impl<T:?Sized+GcBody> GcReference<T>{
//     fn downgrade_clone(&self)->Self{
//         Self{
//             is_gc_root: false,
//             rc_ptr: self.rc_ptr.clone(),
//         }
//     }
//
//     fn upgrade_clone(&self)->Self{
//         // self.rc_ptr.ptr.body.gc_root_count+=1;
//         // if self.rc_ptr.ptr.body.gc_root_count == 1{
//         //     self.rc_ptr.ptr.body.gc.move_to(GcState::ROOT,self.rc_ptr.ptr);
//         // }
//         // Self{
//         //     is_gc_root: true,
//         //     rc_ptr: self.rc_ptr.clone(),
//         // }
//         todo!()
//     }
// }
//
// pub struct GarbageCollector{
//     root_list:          List<GcLLTag,dyn GcBody>,
//     unstable_list:      List<GcLLTag,dyn GcBody>,
//
//     reachable_list:     List<GcLLTag,dyn GcBody>,
//     stable_list:        List<GcLLTag,dyn GcBody>,
//     //delete_list:        List<GcLLTag,dyn GcBody>,
//
//     context:            GcContext,
// }
//
// pub struct GcContext{
//     round:usize,
// }
// //一次移动root到reachable中
// //遍历reachable，对于当前节点的节点的每个子节点，如果孩子为unstable则放入reachable，如果当前节点是root则放入到root中，如果不是root则放入stable中，
// //直到reachable中的节点清空
// //删除当前的unstable中的所有节点
// //集体状态变化:
// //一开始诠释unstable
// //部分是root的unstable -> reachable
// //
// //stable    -> not visited
// //unstable  ->
// //reachable
// impl GarbageCollector{
//     pub fn new()->Self{
//         Self{
//             root_list:      List::new(),
//             unstable_list:  List::new(),
//             reachable_list: List::new(),
//             stable_list:    List::new(),
//             //delete_list:    List::new(),
//             context:GcContext{
//                 round: 0,
//             }
//         }
//     }
//     pub fn start_gc(&mut self){
//         // if self.context.now.is_none(){
//         //     DoubleRawList::insert_front(&self.root_list,null_mut());
//         //
//         // }
//         todo!()
//     }
//     pub fn gc_step(&mut self,steps:isize){
//
//     }
//
// }

#[test]
fn test(){

}