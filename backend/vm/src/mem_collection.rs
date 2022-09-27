use std::borrow::Borrow;
use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::swap;
use std::ptr::{null, null_mut};
use super::util;
use util::data_structure::double_ll::List as DoubleRawList;
use util::data_structure::double_ll::NodeExt;
use crate::util::data_structure::double_ll::{List, Node, NodeExtraData, NodePtr};
use crate::util::data_structure::ptr::{DynPtr, DynPtrMut};

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



pub struct RcObject<T:?Sized> {
    count:usize,
    body:T,
}

pub struct RcReference<B:?Sized>{
    ptr:*mut RcObject<B>,
}
impl<B:?Sized> Drop for RcReference<B>{
    fn drop(&mut self) {
        unsafe{
            if self.ptr.count==1{
                self.ptr.drop_in_place();
                util::deallocate(self.ptr.cast())
            }else{
                self.ptr.count-=1;
            }
        }
    }
}

type GcObject<T> = RcObject<GcData<T>>;

pub trait GcBody{
    fn is_child_scanning(&self)->bool;
    fn enter_gc_scan(&mut self);
    fn next_child(&mut self)->Option<GcReference<dyn GcBody>>;
    fn leave_gc_scan(&mut self);
}
#[derive(Copy,Clone)]
pub enum GcStateType{
    Stable,
    Unstable,
    Reachable,
    //Deleted,
}
pub struct GcState{
    state:GcStateType,
    round:usize,
}
impl GcState{
    fn gc_state(&self,ctx:&GcContext)->GcStateType{
        if self.rounds!= ctx.round{
            GcStateType::Unstable
        }else{
            self.state
        }
    }
}

pub struct GcLLTag();

struct GcData<T:?Sized+GcBody> {
    gc_busy:bool,

    gc_root_count:usize,

    obj_state:GcState,

    gc:DynPtrMut<GarbageCollector>,

    gc_ll_data:NodeExtraData<GcLLTag,dyn GcBody>,

    body:T
}

impl<T:?Sized+GcBody> GcData<T>{
    fn is_root(&self)->bool{
        self.gc_root_count!=0
    }
}




impl<B:?Sized> Clone for RcReference<B>{
    fn clone(&self) -> Self {
        self.ptr.count+=1;
        Self{
            ptr:self.ptr
        }
    }
}


pub struct GcReference<B:?Sized>{
    is_gc_root:bool,
    rc_ptr:RcReference<GcInfo<B>>,
}


impl<B:?Sized> Drop for GcReference<B>{
    fn drop(&mut self) {

    }
}


impl<T:?Sized> Clone for GcReference<T>{
    fn clone(&self) -> Self {
        if self.is_gc_root{
            self.rc_ptr.ptr.body.gc_root_count+=1;
        }
        Self{
            is_gc_root: self.is_gc_root,
            rc_ptr: self.rc_ptr.clone(),
        }
    }
}


impl<T:?Sized> GcReference<T>{
    fn downgrade_clone(&self)->Self{
        Self{
            is_gc_root: false,
            rc_ptr: self.rc_ptr.clone(),
        }
    }

    fn upgrade_clone(&self)->Self{
        self.rc_ptr.ptr.body.gc_root_count+=1;
        if self.rc_ptr.ptr.body.gc_root_count == 1{
            self.rc_ptr.ptr.body.gc.move_to(GcState::ROOT,self.rc_ptr.ptr);
        }
        Self{
            is_gc_root: true,
            rc_ptr: self.rc_ptr.clone(),
        }
    }
}

pub struct GarbageCollector{
    root_list:          List<GcLLTag,dyn GcBody>,
    unstable_list:      List<GcLLTag,dyn GcBody>,

    reachable_list:     List<GcLLTag,dyn GcBody>,
    stable_list:        List<GcLLTag,dyn GcBody>,
    //delete_list:        List<GcLLTag,dyn GcBody>,

    context:            ScanContext,
}

pub struct GcContext{
    round:usize,
}
//一次移动root到reachable中
//遍历reachable，对于当前节点的节点的每个子节点，如果孩子为unstable则放入reachable，如果当前节点是root则放入到root中，如果不是root则放入stable中，
//直到reachable中的节点清空
//删除当前的unstable中的所有节点
//集体状态变化:
//一开始诠释unstable
//部分是root的unstable -> reachable
//
//stable    -> not visited
//unstable  ->
//reachable
impl GarbageCollector{
    pub fn new()->Self{
        Self{
            root_list:      List::new(),
            unstable_list:  List::new(),
            reachable_list: List::new(),
            stable_list:    List::new(),
            //delete_list:    List::new(),
            context:ScanContext{
                now: None
            }
        }
    }
    pub fn start_gc(&mut self){
        if self.context.now.is_none(){
            DoubleRawList::insert_front(&self.root_list,null_mut());

        }
    }
    pub fn gc_step(&mut self,steps:isize){

    }

}