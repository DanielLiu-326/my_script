use std::alloc;
use std::alloc::{Layout};
use std::collections::HashMap;
use std::ffi::c_void;
use std::fs::read;
use std::mem::size_of;
use std::ptr::NonNull;

#[inline]
pub fn allocate(size:usize)->* mut c_void{
    unsafe{
        let mut allocated =
            alloc::alloc(Layout::from_size_align(size+size_of::<usize>(),size_of::<usize>()).unwrap())
                             .cast::<usize>();
        *(allocated as *mut usize) = size;
        return allocated.add(1).cast();
    }
}
#[inline]
pub fn deallocate(ptr:*mut c_void){
    unsafe{
        let allocated = ptr.cast::<usize>().sub(1);
        alloc::dealloc(
                allocated.cast(),
                Layout::from_size_align(
                    *allocated+size_of::<usize>(),
                    size_of::<usize>()
                ).unwrap()
            );
    }
}


pub mod data_structure{
    pub mod double_ll{
        use std::alloc::{alloc, Layout};
        use std::collections::LinkedList;
        use std::fmt::{Debug, Display, Formatter};
        use std::os::unix::net::UnixDatagram;
        use std::ptr::{null, null_mut};
        use crate::util::{allocate, deallocate};

        pub trait Node<TAG,T:?Sized>:AsRef<T>{
            #[inline]
            fn next(&mut self)->&mut *mut (dyn Node<TAG,T>+AsRef<T>);
            #[inline]
            fn prev(&mut self)->&mut *mut (dyn Node<TAG,T>+AsRef<T>);
        }
        pub trait NodeExt{
            #[inline]
            fn remove(&mut self);
        }
        // impl <TAG,T:Node<TAG>> NodeExt for T {
        //     #[inline]
        //     fn remove(&mut self) {
        //         self.prev().next() = self.next();
        //         self.next().prev() = self.prev();
        //     }
        // }
        pub struct List<TAG,T:?Sized>{
            head:*mut dyn Node<TAG,T>,
            rear:*mut dyn Node<TAG,T>,
        }
        impl<TAG,T> Drop for List<TAG,T>{
            fn drop(&mut self) {
                deallocate(self.head.cast());
            }
        }
        struct HeadPlaceholderNode<TAG,T>{
            next :*mut dyn Node<TAG,T>,
            prev :*mut dyn Node<TAG,T>,
        }
        //此处的AsRef是不安全的。是为了满足TraitBound
        impl<TAG,T:?Sized> AsRef<T> for HeadPlaceholderNode<TAG,T>{
            fn as_ref(&self) -> &T {
                unsafe {
                    return (*Self.next).as_ref();
                }
            }
        }
        impl<TAG,T:?Sized> Node<TAG,T> for HeadPlaceholderNode<TAG,T>{
            fn next(&mut self) -> &mut *mut dyn Node<TAG,T> {
                return &mut self.next
            }
            fn prev(&mut self) -> &mut *mut dyn Node<TAG,T> {
                &mut self.prev
            }
        }
        impl<TAG,T:?Sized> List<TAG,T>{
            pub fn new()->Self {
                unsafe{
                    let help_nodes = alloc(Layout::new::<[HeadPlaceholderNode<TAG,T>; 2]>())
                        .cast::<[HeadPlaceholderNode<TAG,T>; 2]>();

                    *(*help_nodes)[0].prev() = null_mut::<HeadPlaceholderNode<TAG,T>>() as *mut dyn Node<TAG,T>;
                    *(*help_nodes)[0].next() = &mut (*help_nodes)[1] as *mut dyn Node<TAG,T>;
                    *(*help_nodes)[1].prev() = &mut (*help_nodes)[0] as *mut dyn Node<TAG,T>;
                    *(*help_nodes)[1].next() = null_mut::<HeadPlaceholderNode<TAG,T>>() as *mut dyn Node<TAG,T>;
                    Self {
                        head: (&mut (*help_nodes)[0] as *mut dyn Node<TAG,T>),
                        rear: (&mut (*help_nodes)[1] as *mut dyn Node<TAG,T>),
                    }
                }
            }
            pub fn insert_front<U:Node<TAG,T>>(&self, node:*mut U){
                unsafe{
                    let mut head = self.head;
                    *(**(*head).next()).prev() = node;
                    *(*head).next() = node;
                    *(*node).next() = *(*head).next();
                    *(*node).prev() = head;
                }
            }
            pub fn insert_back<U:Node<TAG,T>>(&self,node:*mut U){
                unsafe{
                    let mut rear = self.rear;
                    *(**(*rear).prev()).next() = node;
                    *(*rear).prev() = node;
                    *(*node).next() = rear;
                    *(*node).prev() = *(*rear).prev();
                }
            }
            pub fn concat_front(&self,other:&mut Self){
                unsafe {
                    //remove the nodes from other list
                    let other_list_first = *(*other.head).next();
                    let other_list_last = *(*other.rear).prev();
                    *(*other.head).next() = other.rear;
                    *(*other.rear).prev() = other.head;
                    //concat those nodes to the front of self
                    *(*other_list_last).next() = *(*self.head).next();
                    *(**(*self.head).next()).prev() = other_list_last;

                    *(*other_list_first).prev() = self.head;
                    *(*self.head).next() = other_list_first;
                }
            }
            pub fn concat_back(&self,other:&mut Self){
                unsafe{
                    //remove the nodes from other list
                    let other_list_first = *(*other.head).next();
                    let other_list_last = *(*other.rear).prev();
                    *(*other.head).next() = other.rear;
                    *(*other.rear).prev() = other.head;
                    //concat those nodes to the rear of self
                    *(*other_list_first).prev() = *(*self.rear).prev();
                    *(**(*self.rear).prev()).next() = other_list_first;

                    *(*other_list_last).next() = self.rear;
                    *(*self.rear).prev() = other_list_last;
                }
            }
            pub fn empty(&self)->bool{
                unsafe{
                    let mut ptr = self.head;
                    (*(*ptr).next()).is_null()
                }
            }
        }
        #[test]
        fn test(){
            pub struct Test{
                prev_1:*mut dyn Node<Tag1,dyn Debug>,
                next_1:*mut dyn Node<Tag1,dyn Debug>,
                prev_2:*mut dyn Node<Tag2,dyn Debug>,
                next_2:*mut dyn Node<Tag2,dyn Debug>,
                value :usize,
            }
            impl Debug for Test{
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f,"{},",self.value)?;
                    return Ok(());
                }
            }
            impl Test{
                pub fn new(value:usize)->Self{
                    Self{
                        prev_1: Default::default(),
                        next_1: Default::default(),
                        prev_2: Default::default(),
                        next_2: Default::default(),
                        value,
                    }
                }
            }
            pub struct Tag1();
            pub struct Tag2();
            impl AsRef<dyn Debug> for Test {
                fn as_ref(&self) -> &dyn Debug {
                    return self;
                }
            }
            impl Node<Tag1,dyn Debug> for Test{
                fn next(&mut self) -> &mut *mut dyn Node<Tag1,dyn Debug> {
                    return &mut self.next_1;
                }

                fn prev(&mut self) -> &mut *mut dyn Node<Tag1,dyn Debug> {
                    return &mut self.prev_1;
                }
            }
            impl Node<Tag2, dyn Debug> for Test{
                fn next(&mut self) -> &mut *mut dyn Node<Tag2,dyn Debug> {
                    return &mut self.next_2;
                }

                fn prev(&mut self) -> &mut *mut dyn Node<Tag2,dyn Debug> {
                    return &mut self.prev_2;
                }
            }
            let mut test1 = Test::new(1);
            let mut test2 = Test::new(2);
            let mut test3 = Test::new(3);
            let mut test_list_1 = List::<Tag1,dyn Debug>::new();
            let mut test_list_2 = List::<Tag2,dyn Debug>::new();
            test_list_1.insert_back(&mut test1 as *mut Test);
            test_list_2.insert_back(&mut test1 as *mut Test);
            test_list_1.insert_back(&mut test2 as *mut Test);
            test_list_2.insert_back(&mut test2);


        }
    }

}