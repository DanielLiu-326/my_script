use std::any::{Any, TypeId};
use std::cell::{Cell, UnsafeCell};
use std::collections::hash_map::Values;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use super::util;
use util::ptr::thin_dyn::{Obj,Implemented};
use crate::util::{allocate, allocate_value};
use crate::util::ptr::thin_dyn::ObjPtrMut;

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



pub struct RefCountObj<T:Clone> {
    count: usize,
    val: T,
}
impl<T:Clone> RefCountObj<T>{
    pub fn new(val:T,count:usize)->Self{
        Self{
            count,
            val,
        }
    }

    #[inline(always)]
    pub fn unbox(&self)->T{
        unsafe{
            self.val.clone()
        }
    }

    #[inline(always)]
    pub fn as_mut(&mut self)->&mut T{
        return &mut self.val;
    }

    #[inline(always)]
    pub fn as_ref(&self)->& T{
        return &self.val;
    }
}

pub struct RefCount<T:Clone> {
    obj:NonNull<RefCountObj<T>>,
}

impl<T:Clone> RefCount<T>{

    #[inline(always)]
    pub fn new(obj:NonNull<RefCountObj<T>>)->Self{
        Self{
            obj
        }
    }

    #[inline(always)]
    pub fn box_val(&self,val:T)->Self{
        Self{
            obj:NonNull::new(
                allocate_value(RefCountObj::new(val,1))
            ).unwrap()
        }
    }
}

impl<T:Clone> Deref for RefCount<T>{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.obj.as_ref().as_ref()
        }
    }
}

impl<T:Clone> DerefMut for RefCount<T>{

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.obj.as_mut().as_mut()
        }
    }
}

impl<T:Clone> Drop for RefCount<T>{
    fn drop(&mut self) {
        unsafe{
            if self.obj.as_mut().count == 1{
                self.obj.as_ptr().drop_in_place();
            }else{
                self.obj.as_mut().count -= 1;
            }
        }
    }
}

///
/// 常量：
/// 常量的生死存亡不由垃圾收集器管理，它只引用常量表中的项目，常量随着常量表变化
///
pub struct RefConst<T>{
    val:NonNull<T>,
}

impl<T> RefConst<T>{
    pub fn new(val:NonNull<T>)->Self{
        Self{
            val,
        }
    }
}

impl<T> Deref for RefConst<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{
            self.val.as_ref()
        }
    }
}


#[test]
fn test(){

}