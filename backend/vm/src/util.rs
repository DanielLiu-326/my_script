use std::alloc;
use std::alloc::Layout;
use std::ffi::c_void;
use std::mem::size_of;

#[inline]
pub fn allocate(size: usize) -> *mut c_void {
    unsafe {
        let allocated = alloc::alloc(
            Layout::from_size_align(size + size_of::<usize>(), size_of::<usize>()).unwrap(),
        )
        .cast::<usize>();
        *(allocated as *mut usize) = size + size_of::<usize>();
        allocated.add(1).cast()
    }
}
#[inline]
pub fn deallocate(ptr: *mut c_void) {
    unsafe {
        let allocated = ptr.cast::<usize>().sub(1);
        alloc::dealloc(
            allocated.cast(),
            Layout::from_size_align(*allocated, size_of::<usize>()).unwrap(),
        );
    }
}

pub mod data_structure {

    pub mod ptr{
        use std::ops::{Deref, DerefMut};
        ///
        /// DynPtr:
        /// 胖指针，相对于primitive类型增加了null值，对null值解引用将出错
        ///
        pub struct DynPtr<Dyn: ?Sized> {
            ptr: *const (),
            meta: <Dyn as std::ptr::Pointee>::Metadata,
        }

        impl<Dyn:?Sized> Clone for DynPtr<Dyn> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<Dyn:?Sized> Copy for DynPtr<Dyn> {}

        impl<Dyn: ?Sized> DynPtr<Dyn> {
            fn new(ptr: &Dyn) -> Self {
                Self {
                    ptr: (ptr as *const Dyn).cast(),
                    meta: std::ptr::metadata(ptr as *const Dyn),
                }
            }
            fn null() -> Self {
                Self {
                    ptr: std::ptr::null(),
                    meta: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
                }
            }
            fn is_null(&self) -> bool {
                self.ptr.is_null()
            }
            fn thin(&self) -> *const () {
                return self.ptr;
            }
            fn metadata(&self) -> <Dyn as std::ptr::Pointee>::Metadata {
                self.meta
            }
        }
        impl<Dyn: ?Sized> From<&Dyn> for DynPtr<Dyn> {
            fn from(f: &Dyn) -> Self {
                DynPtr::new(f)
            }
        }
        impl<Dyn: ?Sized> Deref for DynPtr<Dyn> {
            type Target = Dyn;

            fn deref(&self) -> &Self::Target {
                unsafe{
                    &*std::ptr::from_raw_parts(self.ptr, self.meta)
                }
            }
        }
        ///
        /// DynPtrMut:
        /// 可变胖指针，相对于primitive类型增加了null值，对null值解引用将出错
        ///
        pub struct DynPtrMut<Dyn: ?Sized> {
            ptr: *mut (),
            meta: <Dyn as std::ptr::Pointee>::Metadata,
        }

        impl<Dyn:?Sized> Clone for DynPtrMut<Dyn> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Dyn:?Sized> Copy for DynPtrMut<Dyn> {}

        impl<Dyn: ?Sized> DynPtrMut<Dyn> {
            pub fn new(ptr: &mut Dyn) -> Self {
                Self {
                    ptr: (ptr as *mut Dyn).cast(),
                    meta: std::ptr::metadata(ptr as *mut Dyn),
                }
            }
            pub fn null() -> Self {
                Self {
                    ptr: std::ptr::null_mut(),
                    meta: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
                }
            }
            pub fn is_null(&self) -> bool {
                self.ptr.is_null()
            }
            pub fn thin(&self) -> *mut () {
                self.ptr
            }
            pub fn metadata(&self) -> <Dyn as std::ptr::Pointee>::Metadata {
                self.meta
            }
        }
        impl<Dyn: ?Sized+'static> From<&mut Dyn> for DynPtrMut<Dyn> {
            fn from(f: &mut Dyn) -> Self {
                DynPtrMut::new(f)
            }
        }
        impl<Dyn: ?Sized> Deref for DynPtrMut<Dyn> {
            type Target = Dyn;

            fn deref(&self) -> &Self::Target {
                unsafe{
                    &*std::ptr::from_raw_parts(self.ptr, self.meta)
                }
            }
        }

        impl<Dyn: ?Sized> DerefMut for DynPtrMut<Dyn> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe {
                    &mut *std::ptr::from_raw_parts_mut(self.ptr, self.meta)
                }
            }
        }
    }

    pub mod double_ll {
        use std::alloc::{alloc, Layout};
        use std::borrow::{Borrow, BorrowMut};
        use std::cell::Cell;
        use std::fmt::{Debug, Formatter};
        use std::ops::{Deref, DerefMut};
        use crate::util::data_structure::ptr::DynPtrMut;

        pub type NodePtr<TAG,Trait:?Sized> = DynPtrMut<dyn NodeExt<TAG, Trait>>;

        pub struct NodeExtraData<TAG,Trait:?Sized> {
            next:Cell<NodePtr<TAG,Trait>>,
            prev:Cell<NodePtr<TAG,Trait>>,
        }

        impl<TAG,Trait:?Sized> Default for NodeExtraData<TAG, Trait>{
            fn default() -> Self {
                Self{
                    next: Cell::new(NodePtr::null()),
                    prev: Cell::new(NodePtr::null())
                }
            }
        }

        impl<TAG,Trait:?Sized> NodeExtraData<TAG, Trait> {
            #[inline]
            pub fn get_next(&self)->NodePtr<TAG,Trait>{
                self.next.get()
            }
            #[inline]
            pub fn set_next(&self,next:NodePtr<TAG,Trait>){
                self.next.set(next)
            }
            #[inline]
            pub fn get_prev(&self)->NodePtr<TAG,Trait>{
                self.prev.get()
            }
            #[inline]
            pub fn set_prev(&self,prev:NodePtr<TAG,Trait>){
                self.prev.set(prev)
            }
        }

        pub trait Node<TAG> {
            type Trait: ?Sized;
            fn extra_data(&self)->&NodeExtraData<TAG,Self::Trait>;
        }

        pub trait NodeExt<TAG,Trait:?Sized> :Node<TAG,Trait=Trait>+BorrowMut<Trait>{
            fn remove(&self){
                self.extra_data().set_next(
                    self.extra_data()
                    .get_next()
                    .deref()
                    .extra_data()
                    .get_next()
                )
            }
        }

        impl <TAG,Trait:?Sized,T> NodeExt<TAG,Trait> for T
            where T:Node<TAG,Trait=Trait>+BorrowMut<Trait>,
        {}

        struct HeadNode<TAG,Trait:?Sized>{
            data:NodeExtraData<TAG,Trait>,
        }

        impl<TAG,Trait:?Sized> Node<TAG> for HeadNode<TAG,Trait>{
            type Trait = Trait;

            fn extra_data(&self) -> &NodeExtraData<TAG, Self::Trait> {
                &self.data
            }
        }

        impl<TAG, Trait: ?Sized> Borrow<Trait> for HeadNode<TAG, Trait> {
            fn borrow(&self) -> &Trait {
                panic!("calling Borrow<Trait>() on head node")
            }
        }

        impl<TAG, Trait: ?Sized> BorrowMut<Trait> for HeadNode<TAG, Trait> {
            fn borrow_mut(&mut self) -> &mut Trait {
                panic!("call BorrowMut<Trait>() on head node")
            }
        }

        pub struct List<TAG,Trait:?Sized> {
            head: NodePtr<TAG,Trait>,
            rear: NodePtr<TAG,Trait>,
        }
        impl<TAG, Trait:?Sized> Drop for List<TAG, Trait> {
            fn drop(&mut self) { unsafe {
                std::alloc::dealloc(
                    self.head.thin().cast(),
                    Layout::new::<[HeadNode<TAG,Trait>; 2]>(),
                );
            }}
        }


        impl<TAG:'static, Trait: ?Sized+'static> List<TAG, Trait> {
            pub fn new() -> Self {
                unsafe {
                    let head_nodes = alloc(Layout::new::<[HeadNode<TAG, Trait>; 2]>())
                        .cast::<[HeadNode<TAG, Trait>; 2]>();
                    let head=NodePtr::new(&mut (*head_nodes)[0]);
                    let rear= NodePtr::new(& mut (*head_nodes)[1]);
                    head.deref().extra_data()
                        .set_prev(NodePtr::null());
                    head.deref().extra_data()
                        .set_next(rear);
                    rear.deref().extra_data()
                        .set_prev(head);
                    rear.deref().extra_data()
                        .set_next(NodePtr::null());
                    Self {
                        head: NodePtr::from(head),
                        rear: NodePtr::from(rear),
                    }
                }
            }

            pub fn insert_front<T:NodeExt<TAG,Trait> + 'static,U:DerefMut<Target=T>>(&self, mut node:U) {
                let head = self.head;
                let node = NodePtr::new(node.deref_mut());
                head.deref().extra_data()
                    .get_next().deref().extra_data()
                    .set_prev(node);
                head.deref().extra_data()
                    .set_next(node);
                node.extra_data()
                    .set_next(head.deref().extra_data().get_next());
                node.extra_data()
                    .set_prev(head);
            }

            pub fn insert_back(&self, mut node: NodePtr<TAG,Trait>) {
                let rear = self.rear;
                let node = NodePtr::from(node.deref_mut());
                rear.deref().extra_data()
                    .get_prev().extra_data()
                    .set_next(node);
                rear.deref().extra_data()
                    .set_prev(node);
                node.deref().extra_data()
                    .set_next(rear);
                node.deref().extra_data()
                    .set_prev(rear.extra_data().get_prev());
            }

            pub fn concat_front(&self, other: &Self) {
                //remove the nodes from other list
                let other_first = other.head.deref().extra_data().get_next();
                let other_last = other.rear.deref().extra_data().get_prev();
                other.head.deref().extra_data().set_next(other.rear);
                other.rear.deref().extra_data().set_prev(other.head);
                //concat those nodes to the front of self
                other_last.deref().extra_data()
                    .set_next(self.head.deref().extra_data().get_next());
                self.head.deref().extra_data()
                    .get_next().deref().extra_data()
                    .set_prev(other_last);

                other_first.deref().extra_data()
                    .set_prev(self.head);
                self.head.deref().extra_data()
                    .set_next(other_last);
            }

            pub fn concat_back(&self, other: &mut Self) {
                //remove the nodes from other list
                let other_first = other.head.deref().extra_data().get_next();
                let other_last = other.rear.deref().extra_data().get_prev();
                other.head.deref().extra_data().set_next(other.rear);
                other.rear.deref().extra_data().set_prev(other.head);
                //concat those nodes to the rear of self
                other_first.deref().extra_data()
                    .set_prev(self.rear.deref().extra_data().get_prev());
                self.rear.deref().extra_data()
                    .get_prev().deref().extra_data()
                    .set_next(other_first);

                other_last.deref().extra_data()
                    .set_next(self.rear);
                self.rear.extra_data()
                    .set_prev(other_last);
            }
            pub fn empty(&self) -> bool {
                self.head.extra_data().get_next().is_null()
            }
        }
        #[test]
        fn test() {
            pub struct Test {
                ext1: NodeExtraData<Tag1, dyn Debug>,
                ext2: NodeExtraData<Tag2, dyn Debug>,
                value: usize,
            }
            impl Debug for Test {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{},", self.value)?;
                    return Ok(());
                }
            }
            impl Test {
                pub fn new(value: usize) -> Self {
                    Self {
                        ext1: Default::default(),
                        ext2: Default::default(),
                        value,
                    }
                }
            }
            impl Borrow<dyn Debug> for Test {
                fn borrow(&self) -> &(dyn Debug + 'static) {
                    self
                }
            }
            impl BorrowMut<dyn Debug> for Test {
                fn borrow_mut(&mut self) -> &mut (dyn Debug + 'static) {
                    self
                }
            }

            pub struct Tag1();
            pub struct Tag2();
            impl Node<Tag1> for Test {
                type Trait = dyn Debug;

                fn extra_data(&self) -> &NodeExtraData<Tag1, dyn Debug> {
                    &self.ext1
                }
            }

            impl Node<Tag2> for Test {
                type Trait = dyn Debug;

                fn extra_data(&self) -> &NodeExtraData<Tag2, dyn Debug> {
                    &self.ext2
                }
            }

            let mut test1 = Test::new(1);
            let mut test2 = Test::new(2);
            let mut test3 = Test::new(3);
            let test_list_1 = List::<Tag1, dyn Debug>::new();
            let test_list_2 = List::<Tag2, dyn Debug>::new();
            test_list_1.insert_back(NodePtr::new(&mut test3));
            test_list_1.insert_back(NodePtr::new(&mut test1));
            test_list_1.insert_back(NodePtr::new(&mut test2));
            test_list_2.insert_back(NodePtr::new(&mut test1));
            test_list_2.insert_back(NodePtr::new(&mut test2));
            <Test as NodeExt<Tag1, dyn Debug>>::remove(&test2);
            let mut ptr = test_list_1.head.deref().extra_data().get_next();
            while !ptr.extra_data().get_next().is_null() {
                println!("{:?}", ptr.deref().borrow());
                ptr = ptr.deref_mut().extra_data().get_next();
            }
        }
    }
}
