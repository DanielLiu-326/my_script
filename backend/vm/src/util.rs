use std::alloc;
use std::alloc::Layout;
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::mem::size_of;
use crate::util::data_structure::double_ll::{List, ListTag, Node, NodeExt, NodeExtraData, NodePtr};

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
pub mod ptr{

    pub mod thin_dyn{
        use core::marker::PhantomData;
        use core::ops::Deref;
        use core::ops::DerefMut;
        /**
         * 类型
         */
        pub unsafe trait Implemented:AsRef<Self::ImplTrait>{
            type ImplTrait:?Sized;
        }

        #[repr(C)]
        pub struct Obj<T:Implemented+Sized> {
            meta: <T::ImplTrait as std::ptr::Pointee>::Metadata,
            data: T,
        }
        impl<T:Implemented+Sized> From<T> for Obj<T>{
            fn from(data: T) -> Self {
                Self{
                    meta: std::ptr::metadata(data.as_ref() as *const T::ImplTrait),
                    data: data,
                }
            }
        }
        impl<T:Implemented+Sized> Obj<T> {
            pub fn meta(&self)-><T::ImplTrait as std::ptr::Pointee>::Metadata{
                self.meta
            }
            pub fn data_mut(&mut self)->&mut T{
                &mut self.data
            }
            pub fn data(&self)->&T{
                &self.data
            }
            pub fn trait_ptr<'a>(&'a self) ->ObjPtr<T::ImplTrait>{
                ObjPtr{
                    obj:(&self.data as *const T).cast(),
                    phantom: Default::default()
                }
            }

            pub fn trait_ptr_mut<'a>(&'a mut self) -> ObjPtrMut<T::ImplTrait>{
                ObjPtrMut{
                    obj: (&mut self.data as *mut T).cast(),
                    phantom: Default::default()
                }
            }
        }


        /**
         *  类型的引用
         */


        #[repr(C)]
        struct Dummy<Trait:?Sized>{
            meta:<Trait as std::ptr::Pointee>::Metadata,
            x:(),
        }

        pub struct ObjRef<'a,ImplTrait:?Sized> {
            obj:*const (),
            phantom:PhantomData<&'a ImplTrait>,
        }

        impl<'a ,Trait:?Sized> Deref for ObjRef<'a,Trait>{
            type Target = Trait;

            fn deref(&self) -> &'a Self::Target {
                let dummy:*const Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(&(*dummy).x as *const (),(*dummy).meta)
                }
            }
        }

        pub struct ObjRefMut<'a,ImplTrait:?Sized> {
            obj:*mut (),
            phantom:PhantomData<&'a ImplTrait>,
        }

        impl<'a,Trait: ?Sized> Deref for ObjRefMut<'a, Trait> {
            type Target = Trait;

            fn deref(&self) -> &Self::Target {
                let dummy:*const Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(&(*dummy).x as *const (),(*dummy).meta)
                }
            }
        }

        impl<'a,Trait:?Sized> DerefMut for ObjRefMut<'a,Trait>{
            fn deref_mut(&mut self) -> &mut Self::Target {
                let dummy:*mut Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &mut *std::ptr::from_raw_parts_mut(&mut (*dummy).x as *mut (),(*dummy).meta)
                }
            }
        }

        /**
         * 类型的指针
         */

        pub struct ObjPtr<ImplTrait:?Sized>{
            obj:*const (),
            phantom:PhantomData<ImplTrait>,
        }
        impl<ImplTrait> Deref for ObjPtr<ImplTrait> {
            type Target = ImplTrait;

            fn deref(&self) -> &Self::Target {
                let dummy:*const Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(&(*dummy).x as *const (),(*dummy).meta)
                }
            }
        }

        pub struct ObjPtrMut<ImplTrait:?Sized>{
            obj:*mut (),
            phantom:PhantomData<ImplTrait>,
        }


        impl<ImplTrait: ?Sized> Deref for ObjPtrMut<ImplTrait> {
            type Target = ();

            fn deref(&self) -> &Self::Target {
                let dummy:*const Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(&(*dummy).x as *const (),(*dummy).meta)
                }
            }
        }

        impl<ImplTrait:?Sized> DerefMut for ObjPtrMut<ImplTrait>{
            fn deref_mut(&mut self) -> &mut Self::Target {
                let dummy:*mut Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &mut *std::ptr::from_raw_parts_mut(&mut (*dummy).x as *mut (),(*dummy).meta)
                }
            }
        }

    }






    use std::ops::{Deref, DerefMut};
    ///
    /// 指针类型，指向普通类型时大小为usize大小，指向非Thin类型时，大小根据为usize+std::ptr::Metadata大小
    /// 支持了dyn类型的null值，对null值解引用是未定义的行为
    ///
    pub struct Ptr<Dyn: ?Sized> {
        ptr: *const (),
        meta: <Dyn as std::ptr::Pointee>::Metadata,
    }

    impl<Dyn:?Sized> Clone for Ptr<Dyn> {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<Dyn:?Sized> Copy for Ptr<Dyn> {}

    impl<Dyn: ?Sized> Ptr<Dyn> {
        pub fn new(ptr: &Dyn) -> Self {
            Self {
                ptr: (ptr as *const Dyn).cast(),
                meta: std::ptr::metadata(ptr as *const Dyn),
            }
        }
        pub fn null() -> Self {
            Self {
                ptr: std::ptr::null(),
                meta: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            }
        }
        pub fn is_null(&self) -> bool {
            self.ptr.is_null()
        }
        pub fn thin(&self) -> *const () {
            return self.ptr;
        }
        pub fn metadata(&self) -> <Dyn as std::ptr::Pointee>::Metadata {
            self.meta
        }
    }
    impl<Dyn:?Sized> Into<*const Dyn> for Ptr<Dyn> {
        fn into(self) -> *const Dyn {
            std::ptr::from_raw_parts(self.ptr,self.meta)
        }
    }
    impl<Dyn: ?Sized> From<&Dyn> for Ptr<Dyn> {
        fn from(f: &Dyn) -> Self {
            Ptr::new(f)
        }
    }
    impl<Dyn: ?Sized> Deref for Ptr<Dyn> {
        type Target = Dyn;

        fn deref(&self) -> &Self::Target {
            unsafe{
                &*std::ptr::from_raw_parts(self.ptr, self.meta)
            }
        }
    }
    ///
    /// DynPtrMut:
    /// 可变胖/瘦指针，相对于primitive类型增加了null值，对null值解引用将出错
    ///
    pub struct PtrMut<Dyn: ?Sized> {
        ptr: *mut (),
        meta: <Dyn as std::ptr::Pointee>::Metadata,
    }

    impl<Dyn:?Sized> Clone for PtrMut<Dyn> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<Dyn:?Sized> Copy for PtrMut<Dyn> {}

    impl<Dyn: ?Sized> PtrMut<Dyn> {
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

    impl<Dyn:?Sized> Into<*mut Dyn> for PtrMut<Dyn> {
        fn into(self) -> *mut Dyn {
            std::ptr::from_raw_parts_mut(self.ptr,self.meta)
        }
    }
    impl<Dyn:?Sized> Into<*const Dyn> for PtrMut<Dyn> {
        fn into(self) -> *const Dyn {
            std::ptr::from_raw_parts(self.ptr,self.meta)
        }
    }

    impl<Dyn: ?Sized+'static> From<&mut Dyn> for PtrMut<Dyn> {
        fn from(f: &mut Dyn) -> Self {
            PtrMut::new(f)
        }
    }
    impl<Dyn: ?Sized> Deref for PtrMut<Dyn> {
        type Target = Dyn;

        fn deref(&self) -> &Self::Target {
            unsafe{
                &*std::ptr::from_raw_parts(self.ptr, self.meta)
            }
        }
    }

    impl<Dyn: ?Sized> DerefMut for PtrMut<Dyn> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *std::ptr::from_raw_parts_mut(self.ptr, self.meta)
            }
        }
    }
}
pub mod data_structure {
    pub mod double_ll {
        use core::panicking::panic;
        use std::alloc::{alloc, Layout};
        use std::borrow::{Borrow, BorrowMut};
        use std::cell::Cell;
        use std::fmt::{Debug, Formatter};
        use std::io::Take;
        use std::ops::{Deref, DerefMut};
        use crate::util::ptr::PtrMut;

        pub trait ListTag{
            type Trait:?Sized;
        }

        ///
        /// 节点的前向/后向指针类型
        ///
        pub type NodePtr<TAG:ListTag> = PtrMut<dyn NodeExt<TAG>>;

        ///
        /// 一个类型作为节点所需要的指针信息
        ///
        pub struct NodeExtraData<TAG:ListTag> {
            next:Cell<NodePtr<TAG>>,
            prev:Cell<NodePtr<TAG>>,
        }

        impl<TAG:ListTag> Default for NodeExtraData<TAG>{
            fn default() -> Self {
                Self{
                    next: Cell::new(NodePtr::null()),
                    prev: Cell::new(NodePtr::null())
                }
            }
        }

        impl<TAG:ListTag> NodeExtraData<TAG> {
            #[inline]
            pub fn get_next(&self)->NodePtr<TAG>{
                self.next.get()
            }
            #[inline]
            pub fn set_next(&self,next:NodePtr<TAG>){
                self.next.set(next)
            }
            #[inline]
            pub fn get_prev(&self)->NodePtr<TAG>{
                self.prev.get()
            }
            #[inline]
            pub fn set_prev(&self,prev:NodePtr<TAG>){
                self.prev.set(prev)
            }
        }


        ///
        /// 一个类型想要当作List的节点就必须实现Node
        ///
        /// **泛型参数:**
        ///
        /// - TAG:同一个节点可以在不同的List中，节点要存储多个List的指针信息，用0大小的类型当作Tag来区分他们
        ///

        pub trait Node<TAG:ListTag>:BorrowMut<TAG::Trait> {
            /// 返回节点的附加信息
            fn extra_data(&self)->&NodeExtraData<TAG>;
        }

        ///
        /// 类型想要作为List节点的第二个限制，必须实现过NodeExt
        ///
        /// - NodeExt是对Node的补充，Node的Node::Trait并不能约束实现了Node类型的一定实现过Trait类型。
        /// - NodeExt将会为所有实现过BorrowMut<Trait>和Node的类型自动实现，不需要手动实现
        /// - NodeExt对外的作用是可以通过对象本身来从它所在的List中移除
        ///
        pub trait NodeExt<TAG:ListTag>{
            fn remove(&self);
            fn as_trait(&self)->&TAG::Trait;
            fn as_trait_mut(&self)->&TAG::Trait;
            fn get_next(&self)->NodePtr<TAG>;
            fn set_next(&self,next:NodePtr<TAG>);
            fn get_prev(&self)->NodePtr<TAG>;
            fn set_prev(&self,prev:NodePtr<TAG>);
        }

        impl <TAG:ListTag,T> NodeExt<TAG> for T
            where T:Node<TAG>, {
            fn remove(&self){
                self.extra_data().set_next(
                    self.extra_data()
                        .get_next()
                        .deref()
                        .extra_data()
                        .get_next()
                )
            }
            fn as_trait(&self) -> &TAG::Trait {
                return self.borrow()
            }
            fn as_trait_mut(&self) -> &TAG::Trait {
                return self.borrow_mut()
            }
            fn get_next(&self) -> NodePtr<TAG> {
                self.extra_data().get_next()
            }
            fn set_next(&self, next: NodePtr<TAG>) {
                self.extra_data().set_next(next)
            }
            fn get_prev(&self) -> NodePtr<TAG> {
                self.extra_data().get_prev()
            }
            fn set_prev(&self, prev: NodePtr<TAG>) {
                self.extra_data().set_prev(prev)
            }
        }
        struct HeadNode<TAG:ListTag>{
            data:NodeExtraData<TAG>,
        }
        impl<TAG:ListTag> NodeExt<TAG> for HeadNode<TAG>{
            fn remove(&self) {
                self.data.set_next(
                    self.extra_data()
                        .get_next()
                        .deref()
                        .extra_data()
                        .get_next()
                )
            }
            fn as_trait(&self) -> &TAG::Trait {
                panic!("error:calling Borrow<TAG::Trait>::borrow on HeaderNode")
            }
            fn as_trait_mut(&self) -> &TAG::Trait {
                panic!("error:calling BorrowMut<TAG::Trait>::borrow_mut on HeaderNode")
            }
            fn get_next(&self) -> NodePtr<TAG> {
                self.data.get_next()
            }
            fn set_next(&self, next: NodePtr<TAG>) {
                self.data.set_next(next)
            }
            fn get_prev(&self) -> NodePtr<TAG> {
                self.data.get_prev()
            }
            fn set_prev(&self, prev: NodePtr<TAG>) {
                self.data.set_prev(prev)
            }

        }

        /// deprecated cuased by:
        ///
        /// error[E0119]: conflicting implementations of trait `std::borrow::BorrowMut<util::data_structure::double_ll::HeadNode<_>>` for type `util::data_structure::double_ll::HeadNode<_>`
        ///    --> src\util.rs:414:9
        ///     |
        /// 414 |         impl<TAG: ListTag> BorrowMut<TAG::Trait> for HeadNode<TAG> {
        ///     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        ///     |
        ///     = note: conflicting implementation in crate `core`:
        ///             - impl<T> BorrowMut<T> for T
        ///               where T: ?Sized;
        ///
        /// reference: https://github.com/rust-lang/rust/issues/56804
        /// wait for   https://github.com/rust-lang/rfcs/pull/1672 to be stablized
        ///
        ///
        // struct HeadNode{
        //     data:NodeExtraData<HeadTag>,
        // }
        // impl<TAG: ListTag> BorrowMut<TAG::Trait> for HeadNode<TAG> {
        //     fn borrow_mut(&mut self) -> &mut TAG::Trait {
        //         panic!("error:calling BorrowMut<TAG::Trait>::borrow_mut on HeaderNode")
        //     }
        // }
        //
        // impl<TAG: ListTag> Borrow<TAG::Trait> for HeadNode<TAG> {
        //     fn borrow(&self) -> &TAG::Trait {
        //         panic!("error:calling Borrow<TAG::Trait>::borrow on HeaderNode")
        //     }
        // }
        //
        // impl<TAG:ListTag> Node<TAG> for HeadNode<TAG>{
        //     fn extra_data(&self) -> &NodeExtraData<TAG> {
        //         self.data
        //     }
        // }

        pub struct List<TAG:ListTag> {
            head: NodePtr<TAG>,
            rear: NodePtr<TAG>,
        }
        impl<TAG:ListTag> Drop for List<TAG> {
            fn drop(&mut self) { unsafe {
                std::alloc::dealloc(
                    self.head.thin().cast(),
                    Layout::new::<[HeadNode; 2]>(),
                );
            }}
        }


        impl<TAG:ListTag> List<TAG> {
            pub fn new() -> Self {
                unsafe {
                    let head_nodes = alloc(Layout::new::<[HeadNode; 2]>())
                        .cast::<[HeadNode; 2]>();
                    let head= NodePtr::new(&mut (*head_nodes)[0]);
                    let rear= NodePtr::new(&mut (*head_nodes)[1]);
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

            pub fn insert_front<T:Node<TAG>,U:Deref<Target=T>>(&self, node:U) {
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

            pub fn insert_back<T:Node<TAG>,U:Deref<Target=T>>(&self, node: U) {
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
    }
}
use core::borrow::Borrow;
use core::borrow::BorrowMut;
#[test]
fn test_double_ll() {
    pub struct Tag1();
    impl ListTag for Tag1{
        type Trait =dyn Debug;
    }
    pub struct Tag2();
    impl ListTag for Tag2{
        type Trait = dyn Debug;
    }
    pub struct Test {
        ext1: NodeExtraData<Tag1>,
        ext2: NodeExtraData<Tag2>,
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
        fn borrow(&self) -> &(dyn Debug+'static) {
            self
        }
    }
    impl BorrowMut<dyn Debug> for Test {
        fn borrow_mut(&mut self) -> &mut (dyn Debug+'static) {
            self
        }
    }

    impl Node<Tag1> for Test {

        fn extra_data(&self) -> &NodeExtraData<Tag1> {
            &self.ext1
        }
    }

    impl Node<Tag2> for Test {

        fn extra_data(&self) -> &NodeExtraData<Tag2> {
            &self.ext2
        }
    }

    let mut test1 = Test::new(1);
    let mut test2 = Test::new(2);
    let mut test3 = Test::new(3);
    let test_list_1 = List::<Tag1>::new();
    let test_list_2 = List::<Tag2>::new();
    test_list_1.insert_back(NodePtr::new(&mut test3));
    test_list_1.insert_back(NodePtr::new(&mut test1));
    test_list_1.insert_back(NodePtr::new(&mut test2));
    test_list_2.insert_back(NodePtr::new(&mut test1));
    test_list_2.insert_back(NodePtr::new(&mut test2));
    <Test as NodeExt<Tag1>>::remove(&test2);
    let mut ptr = test_list_1.head.deref().extra_data().get_next();
    while !ptr.extra_data().get_next().is_null() {
        println!("{:?}", ptr.deref().borrow());
        ptr = ptr.deref_mut().extra_data().get_next();
    }
}