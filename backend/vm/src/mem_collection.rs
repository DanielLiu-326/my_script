use std::borrow::Borrow;
use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::swap;
use std::ops::{Deref, DerefMut};
use std::ptr::{null, null_mut};
use super::util;
use util::data_structure::double_ll::List as DoubleRawList;
use util::data_structure::double_ll::NodeExt;
use crate::util::data_structure::double_ll::{List, Node, NodeExtraData, NodePtr};
use crate::util::data_structure::ptr::{Ptr, PtrMut};

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
    ptr:PtrMut<RcObject<B>>,
}

impl<B:?Sized> RcReference<B>{
    pub fn inner(&self)->&PtrMut<RcObject<B>>{
        &self.ptr
    }
    pub fn inner_mut(&mut self)->&mut PtrMut<RcObject<B>>{
        &mut self.ptr
    }
    pub fn body(&self)->&B{
        &self.ptr.deref().body
    }
    pub fn body_mut(&mut self) ->&mut B{
        &mut self.ptr.deref_mut().body
    }
    pub fn count(&self)->usize{
        self.ptr.count
    }
}
impl <B:?Sized> Deref for RcReference<B>{
    type Target = RcObject<B>;

    fn deref(&self) -> &Self::Target {
        self.ptr.deref()
    }
}
impl <B:?Sized> DerefMut for RcReference<B>{

    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ptr.deref_mut()
    }
}

impl<B:?Sized> Clone for RcReference<B>{
    fn clone(&self) -> Self {
        let mut ret =Self{
            ptr:self.ptr
        };
        ret.deref_mut().count+=1;
        ret
    }
}
impl<B:?Sized> Drop for RcReference<B>{
    fn drop(&mut self) {
        unsafe{
            self.ptr.deref_mut().count -= 1;
            if self.ptr.deref().count==0{
                <PtrMut<RcObject<B>> as Into<*mut RcObject<B>>>::into(self.ptr).drop_in_place();
            }
        }
    }
}



/********************************************************************************/

pub trait GcBody{
    fn is_child_scanning(&self)->bool;
    fn enter_gc_scan(&mut self);
    fn next_child(&mut self)->Option<GcReference<dyn GcBody>>;
    fn leave_gc_scan(&mut self);
}


type GcObject<T:?Sized+GcBody> = RcObject<GcData<T>>;


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
        if self.round!= ctx.round{
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

    gc:PtrMut<GarbageCollector>,

    gc_ll_data:NodeExtraData<GcLLTag,dyn GcBody>,

    body:T
}

impl<T:?Sized+GcBody> GcData<T>{
    fn is_root(&self)->bool{
        self.gc_root_count!=0
    }
}




pub struct GcReference<B:?Sized+GcBody>{
    is_gc_root:bool,
    rc_ptr:RcReference<GcData<B>>,
}


impl<B:?Sized+GcBody> Drop for GcReference<B>{
    fn drop(&mut self) {
        if self.is_gc_root{
            self.rc_ptr.body_mut().gc_root_count-=1;
        }
        //todo remove from
    }
}


impl<T:?Sized+GcBody> Clone for GcReference<T>{
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


impl<T:?Sized+GcBody> GcReference<T>{
    fn downgrade_clone(&self)->Self{
        Self{
            is_gc_root: false,
            rc_ptr: self.rc_ptr.clone(),
        }
    }

    fn upgrade_clone(&self)->Self{
        // self.rc_ptr.ptr.body.gc_root_count+=1;
        // if self.rc_ptr.ptr.body.gc_root_count == 1{
        //     self.rc_ptr.ptr.body.gc.move_to(GcState::ROOT,self.rc_ptr.ptr);
        // }
        // Self{
        //     is_gc_root: true,
        //     rc_ptr: self.rc_ptr.clone(),
        // }
        todo!()
    }
}

pub struct GarbageCollector{
    root_list:          List<GcLLTag,dyn GcBody>,
    unstable_list:      List<GcLLTag,dyn GcBody>,

    reachable_list:     List<GcLLTag,dyn GcBody>,
    stable_list:        List<GcLLTag,dyn GcBody>,
    //delete_list:        List<GcLLTag,dyn GcBody>,

    context:            GcContext,
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
            context:GcContext{
                round: 0,
            }
        }
    }
    pub fn start_gc(&mut self){
        // if self.context.now.is_none(){
        //     DoubleRawList::insert_front(&self.root_list,null_mut());
        //
        // }
        todo!()
    }
    pub fn gc_step(&mut self,steps:isize){

    }

}