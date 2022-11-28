use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use crate::util::{allocate_value};

// TODO:
//     Garbage Collector

// TODO:
//     Copy On Write Optimize

pub type RefAddr = usize;

pub trait Addr{
    fn ref_addr(&self)-> RefAddr;
}

///
/// Reference Object
///

pub struct RefCountObj<T:Clone> {
    count: usize,
    val: T,
}
impl<T:Clone> RefCountObj<T>{

    #[inline(always)]
    pub fn new(val:T,count:usize)->Self{
        Self{
            count,
            val,
        }
    }

    #[inline(always)]
    pub fn as_mut(&mut self) -> &mut T {
        return &mut self.val;
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        return &self.val;
    }
}

///
/// Reference of reference count object
///

pub struct RefCount<T:Clone> {
    obj:NonNull<RefCountObj<T>>,
}

impl<T:Clone> RefCount<T> {

    #[inline(always)]
    pub fn new(obj:NonNull<RefCountObj<T>>) -> Self {
        Self{
            obj
        }
    }

    #[inline(always)]
    pub fn box_val(&self,val:T) -> Self {
        Self{
            obj:allocate_value(RefCountObj::new(val,1))
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

impl<T:Clone> Drop for RefCount<T> {

    #[inline(always)]
    fn drop(&mut self) {
        unsafe{
            if self.obj.as_mut().count == 1 {
                self.obj.as_ptr().drop_in_place();
            } else {
                self.obj.as_mut().count -= 1;
            }
        }
    }
}

impl<T:Clone> Addr for RefCount<T>{
    fn ref_addr(&self) -> RefAddr {
        self.obj.as_ptr() as RefAddr
    }
}

///
/// Constants：
/// 常量的生死存亡不由垃圾收集器管理，它只引用常量表中的项目，常量随着常量表变化
///
#[derive(Clone)]
pub struct RefConst<T>{
    val:NonNull<T>,
}

impl<T> From<NonNull<T>> for RefConst<T> {
    fn from(val: NonNull<T>) -> Self {
        Self{
            val,
        }
    }
}

impl<T> RefConst<T>{

    #[inline(always)]
    pub fn new(val:NonNull<T>)->Self{
        Self{
            val,
        }
    }
}


impl<T> Deref for RefConst<T>{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe{
            self.val.as_ref()
        }
    }
}

impl<T> Addr for RefConst<T> {
    fn ref_addr(&self) -> RefAddr {
        self.val.as_ptr() as RefAddr
    }
}