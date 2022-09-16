// use std::borrow::Borrow;
// use std::collections::HashMap;
// use std::ffi::c_void;
// use std::mem::swap;
// use std::ptr::{null, null_mut};
// use super::util;
// use util::data_structure::double_ll::List as DoubleRawList;
// use util::data_structure::double_ll::NodeExt;
// use crate::util::data_structure::double_ll::{List, Node};

//root                  //child change, self deleted, become not root
//unstable              //child change，self deleted，become root
//reachable             //child change，self deleted，become root
//stable                //child change，self deleted，become root
//deleted

//0. root become not root
//insert into stable
//1. unstable child change
//not care
//2. unstable self deleted
//not care
//3. unstable become root
//insert into reachable
//4. reachable child change
//new child insert into reachable
//5.reachable self deleted
//      not visited: not care
//      visiting:stopping visiting now and visit next and delete
//6.reachable become root
// not care
//7.stable/root child change
// new child into reachable
//8.stable/root self deleted
//not care

//让root中的节点移动到reachable
//遍历reachable，把每个节点的子节点中为unstable的节点放入到reachable中，当前节点如果是root则放入到root中，如果不是root则放入stable中
//直到reachable中的节点清空
//删除当前的unstable中的所有节点

//self.move_all_to(self.root,REACHABLE)
//for x in reachable{
//      for y in x.childs{
//          reachable.append(y);
//      }
//      if x.is_root(){
//          root.insert(x);
//      }else{
//          stable.insert(x);
//      }
// }



// pub struct RcObject<T:?Sized>{
//     count:usize,
//     body:T,
// }
//
// type GcObject<T> = RcObject<GcInfo<T>>;
//
// //HashMap<ChildNode{next,prev,ptr}>
// pub trait GcBody{
//     fn gc_scanning(&mut self)->bool;
//     fn enter_gc_scan(&mut self);
//     fn next_child(&mut self)->Option<GcReference<dyn GcBody>>;
//     fn leave_gc_scan(&mut self);
// }
//
// pub enum GcState{
//     Stable,
//     Unstable,
//     Reachable,
//     Deleted,
// }
//
// struct GcInfo<T:?Sized+GcBody>{
//     gc_busy:bool,
//     gc_root_count:usize,
//
//     gc_ll_prev:*mut GcObject<dyn GcBody>,
//     gc_ll_next:*mut GcObject<dyn GcBody>,
//
//     gc:*mut GarbageCollector,
//
//     body:T
// }
//
// pub struct GcLLTag();
// impl<T:GcBody+?Sized> Node<GcLLTag> for GcObject<T>{
//
//     fn next(&mut self) -> &mut *mut GcObject<dyn GcBody> {
//         return &mut self.body.gc_ll_next
//     }
//
//     fn prev(&mut self) -> &mut *mut GcObject<dyn GcBody> {
//         return &mut self.body.gc_ll_prev
//     }
// }
//
// pub struct RcReference<B:?Sized>{
//     ptr:*mut RcObject<B>,
// }
// impl<B:?Sized> Drop for RcReference<B>{
//     fn drop(&mut self) {
//         unsafe{
//             if self.ptr.count==1{
//                 self.ptr.drop_in_place();
//                 util::deallocate(self.ptr)
//             }else{
//                 self.ptr.count-=1;
//             }
//         }
//     }
// }
//
// impl<B:?Sized> Clone for RcReference<B>{
//     fn clone(&self) -> Self {
//         self.ptr.count+=1;
//         Self{
//             ptr:self.ptr
//         }
//     }
// }
//
//
// pub struct GcReference<B:?Sized>{
//     is_gc_root:bool,
//     rc_ptr:RcReference<GcInfo<B>>,
// }
//
//
// impl<B:?Sized> Drop for GcReference<B>{
//     fn drop(&mut self) {
//         if self.is_gc_root{
//             self.rc_ptr.ptr.body.gc_root_count-=1;
//             if self.rc_ptr.ptr.body.gc_root_count==0{
//                 self.rc_ptr.ptr.body.gc.move_to(GcState::UNSTABLE,self.rc_ptr.ptr)
//             }
//         }
//     }
// }
//
//
// impl<T:?Sized> Clone for GcReference<T>{
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
// impl<T:?Sized> GcReference<T>{
//     fn downgrade_clone(&self)->Self{
//         Self{
//             is_gc_root: false,
//             rc_ptr: self.rc_ptr.clone(),
//         }
//     }
//
//     fn upgrade_clone(&self)->Self{
//         self.rc_ptr.ptr.body.gc_root_count+=1;
//         if self.rc_ptr.ptr.body.gc_root_count == 1{
//             self.rc_ptr.ptr.body.gc.move_to(GcState::ROOT,self.rc_ptr.ptr);
//         }
//         Self{
//             is_gc_root: true,
//             rc_ptr: self.rc_ptr.clone(),
//         }
//     }
// }
//
// pub struct GarbageCollector{
//     root_list: List<GcLLTag>,
//     unstable_list: List<GcLLTag>,
//
//     reachable_list:List<GcLLTag>,
//     stable_list:List<GcLLTag>,
//     delete_list:List<GcLLTag>,
//
//     context:ScanContext,
// }
//
// pub struct ScanContext{
//     now:Option<GcReference<dyn GcBody>>,
// }
// //让root中的节点移动到reachable
// //遍历reachable，对于当前节点的节点的每个子节点，如果孩子为unstable则放入reachable，如果当前节点是root则放入到root中，如果不是root则放入stable中，
// //直到reachable中的节点清空
// //删除当前的unstable中的所有节点
// impl GarbageCollector{
//     pub fn new()->Self{
//         Self{
//             root_list: DoubleRawList::new(),
//             unstable_list: DoubleRawList::new(),
//             reachable_list: DoubleRawList::new(),
//             stable_list: DoubleRawList::new(),
//             delete_list: DoubleRawList::new(),
//             context: ScanContext{
//                 now: None
//             }
//         }
//     }
//     pub fn start_gc(&mut self){
//         if self.context.now.is_none(){
//
//             DoubleRawList::insert_front(&self.root_list,null_mut());
//
//         }
//     }
//     pub fn gc_step(&mut self,steps:isize){
//
//     }
//
// }

#[test]
fn test() {


}
