use std::alloc;
use std::alloc::Layout;
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::mem::size_of;
use crate::util::data_structure::double_ll::{Implemented, List, ListTag, Node, NodeExt, NodeExtraData, NodePtr};

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
        impl<ImplTrait:?Sized> ObjPtrMut<ImplTrait>{
            fn new()->Self{

            }
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
        #[inline]
        pub fn null() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
                meta: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            }
        }
        #[inline]
        pub fn is_null(&self) -> bool {
            self.ptr.is_null()
        }
        #[inline]
        pub fn thin(&self) -> *mut () {
            self.ptr
        }
        #[inline]
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
        use std::alloc::{alloc, Layout};
        use std::borrow::{Borrow, BorrowMut};
        use std::cell::Cell;
        use std::fmt::{Debug, Formatter};
        use std::io::Take;
        use std::ops::{Deref, DerefMut};
        use crate::util::ptr::{Ptr, PtrMut};

        /// 一个类型实现了Implemented<Trait>，就看作它实现了Trait，这里用TraitObject代替Dyn
        pub trait Implemented<Trait:?Sized>:AsRef<Trait>+AsMut<Trait>{}

        /// ListTag是Tag实现的类型，Trait代表该List每个元素都实现的Trait，Tag用于区分不同的List
        pub trait ListTag:'static{
            type Trait:?Sized;
        }

        ///
        /// 节点的前向/后向指针类型
        ///
        pub type NodePtr<TAG> = PtrMut<dyn Node<TAG>>;

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

        pub trait Node<TAG:ListTag>:Implemented<TAG::Trait> {
            /// 返回节点的附加信息
            fn extra_data(&self)->&NodeExtraData<TAG>;
        }

        ///
        /// NodeExt是对Node的补充,增加了实用方法
        ///
        pub trait NodeExt<TAG:ListTag>{
            fn remove(&self);
            fn as_trait(&self)->&TAG::Trait;
            fn as_trait_mut(&mut self)->&TAG::Trait;
            fn get_next(&self)->NodePtr<TAG>;
            fn set_next(&self,next:NodePtr<TAG>);
            fn get_prev(&self)->NodePtr<TAG>;
            fn set_prev(&self,prev:NodePtr<TAG>);
        }
        impl <TAG:ListTag,T:Node<TAG>+?Sized> NodeExt<TAG> for T {
            #[inline]
            fn remove(&self){
                self.get_prev().set_next(self.get_next());
                self.get_next().set_prev(self.get_prev());
            }
            #[inline]
            fn as_trait(&self) -> &TAG::Trait {
                return self.as_ref()
            }
            #[inline]
            fn as_trait_mut(&mut self) -> &TAG::Trait {
                return self.as_mut()
            }
            #[inline]
            fn get_next(&self) -> NodePtr<TAG> {
                self.extra_data().get_next()
            }
            #[inline]
            fn set_next(&self, next: NodePtr<TAG>) {
                self.extra_data().set_next(next)
            }
            #[inline]
            fn get_prev(&self) -> NodePtr<TAG> {
                self.extra_data().get_prev()
            }
            #[inline]
            fn set_prev(&self, prev: NodePtr<TAG>) {
                self.extra_data().set_prev(prev)
            }
        }

        ///
        /// 头节点实现。
        ///
        struct HeadNode<TAG:ListTag>{
            extra:NodeExtraData<TAG>,
        }

        impl<TAG: ListTag> Implemented<TAG::Trait> for HeadNode<TAG> {}

        impl<TAG: ListTag> AsRef<TAG::Trait> for HeadNode<TAG> {
            fn as_ref(&self) -> &TAG::Trait {
                panic!("called AsRef::as_ref() on head node")
            }
        }

        impl<TAG: ListTag> AsMut<TAG::Trait> for HeadNode<TAG> {
            fn as_mut(&mut self) -> &mut TAG::Trait {
                panic!("called AsMut::as_mut() on head node")
            }
        }

        impl<TAG:ListTag> Node<TAG> for HeadNode<TAG>{
            fn extra_data(&self) -> &NodeExtraData<TAG> {
                &self.extra
            }
        }

        ///
        /// 双向链表类型，TAG用于区分不同的类型，并通过实现ListNode中的Trait关联类型来确定链表每个元素都有的Trait类型。
        /// 
        /// ```rust
        ///     pub struct Tag1();
        ///     impl ListTag for Tag1{
        ///         type Trait =dyn Debug;
        ///     }
        ///     pub struct Tag2();
        ///     impl ListTag for Tag2{
        ///         type Trait = dyn Debug;
        ///     }
        ///     pub struct Test {
        ///         ext1: NodeExtraData<Tag1>,
        ///         ext2: NodeExtraData<Tag2>,
        ///         value: usize,
        ///     }
        ///     impl Debug for Test {
        ///         fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ///             write!(f, "{},", self.value)?;
        ///             return Ok(());
        ///         }
        ///     }
        ///     impl Test {
        ///         pub fn new(value: usize) -> Self {
        ///             Self {
        ///                 ext1: Default::default(),
        ///                 ext2: Default::default(),
        ///                 value,
        ///             }
        ///         }
        ///     }
        /// 
        ///     impl AsRef<<Tag1 as ListTag>::Trait> for Test {
        ///         fn as_ref(&self) -> &<Tag1 as ListTag>::Trait {
        ///             self
        ///         }
        ///     }
        ///     impl AsMut<<Tag1 as ListTag>::Trait> for Test {
        ///         fn as_mut(&mut self) -> &mut <Tag1 as ListTag>::Trait {
        ///             self
        ///         }
        ///     }
        ///     impl Implemented<<Tag1 as ListTag>::Trait> for Test {}
        ///     impl Node<Tag1> for Test {
        ///         fn extra_data(&self) -> &NodeExtraData<Tag1> {
        ///             &self.ext1
        ///         }
        ///     }
        /// 
        ///     impl Node<Tag2> for Test {
        ///         fn extra_data(&self) -> &NodeExtraData<Tag2> {
        ///             &self.ext2
        ///         }
        ///     }
        /// 
        ///     let mut test1 = Test::new(1);
        ///     let mut test2 = Test::new(2);
        ///     let mut test3 = Test::new(3);
        ///     let test_list_1 = List::<Tag1>::new();
        ///     let test_list_2 = List::<Tag2>::new();
        ///     test_list_1.insert_back(NodePtr::new(&mut test3));
        ///     test_list_1.insert_back(NodePtr::new(&mut test1));
        ///     test_list_1.insert_back(NodePtr::new(&mut test2));
        ///     test_list_2.insert_back(NodePtr::new(&mut test1));
        ///     test_list_2.insert_back(NodePtr::new(&mut test2));
        ///     <Test as NodeExt<Tag1>>::remove(&test2);
        ///     let mut ptr = test_list_1.head_next();
        ///     while !ptr.get_next().is_null() {
        ///         println!("{:?}", ptr.as_ref());
        ///         ptr = ptr.get_next();
        ///     }
        ///     println!(" ");
        ///     let mut ptr = test_list_2.head_next();
        ///     while !ptr.get_next().is_null() {
        ///         println!("{:?}", ptr.as_mut());
        ///         ptr = ptr.get_next();
        ///     }
        /// ```
        ///
        pub struct List<TAG:ListTag> {
            head: NodePtr<TAG>,
            rear: NodePtr<TAG>,
        }
        impl<TAG:ListTag> Drop for List<TAG> {
            fn drop(&mut self) { unsafe {
                std::alloc::dealloc(
                    self.head.thin().cast(),
                    Layout::new::<[HeadNode<TAG>; 2]>(),
                );
            }}
        }


        impl<TAG:ListTag> List<TAG> {
            pub fn new() -> Self {
                unsafe {
                    /**
                     * 分配节点
                     */
                    let head_nodes = alloc(Layout::new::<[HeadNode<TAG>; 2]>())
                        .cast::<[HeadNode<TAG>; 2]>();
                    let head= NodePtr::new(&mut (*head_nodes)[0]);
                    let rear= NodePtr::new(&mut (*head_nodes)[1]);
                    /**
                     * 连接头尾节点。
                     */
                    head.set_prev(NodePtr::null());
                    head.set_next(rear);
                    rear.set_prev(head);
                    rear.set_next(NodePtr::null());
                    Self {
                        head,
                        rear,
                    }
                }
            }

            /// head_next:
            ///
            /// 返回头节点的下一个节点，如果链表为空返回尾节点
            pub fn head_next(&self)->NodePtr<TAG>{
                self.head.get_next()
            }

            /// 头插节点
            pub fn insert_front<U:DerefMut<Target=dyn Node<TAG>>>(&self, mut node:U) {
                let head = self.head;
                let node = NodePtr::new(node.deref_mut());
                head.get_next().set_prev(node);
                head.set_next(node);
                node.set_next(head.get_next());
                node.set_prev(head);
            }

            ///尾插节点
            pub fn insert_back<U:DerefMut<Target=dyn Node<TAG>>>(&self, mut node: U) {
                let rear = self.rear;
                let node = NodePtr::new(node.deref_mut());
                rear.get_prev().set_next(node);
                rear.set_prev(node);
                node.set_next(rear);
                node.set_prev(rear.get_prev());
            }

            /// 将另一个链表的全部节点头插到本链表的前面，时间复杂度O(1)
            pub fn concat_front(&self, other: &Self) {
                //remove the nodes from other list
                let other_first = other.head.get_next();
                let other_last = other.rear.get_prev();
                other.head.set_next(other.rear);
                other.rear.set_prev(other.head);
                //concat those nodes to the front of self
                other_last.set_next(self.head.get_next());
                self.head.get_next().set_prev(other_last);

                other_first.set_prev(self.head);
                self.head.set_next(other_last);
            }

            /// 将另一个链表的全部节点尾插到本链表的前面，时间复杂度O(1)
            pub fn concat_back(&self, other: &mut Self) {
                //remove the nodes from other list
                let other_first = other.head.get_next();
                let other_last = other.rear.get_prev();
                other.head.set_next(other.rear);
                other.rear.set_prev(other.head);
                //concat those nodes to the rear of self
                other_first.set_prev(self.rear.get_prev());
                self.rear.get_prev().set_next(other_first);

                other_last.set_next(self.rear);
                self.rear.set_prev(other_last);
            }
            /// 链表是否为空
            pub fn empty(&self) -> bool {
                self.head.get_next().is_null()
            }
        }
    }
}
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

    impl AsRef<<Tag1 as ListTag>::Trait> for Test {
        fn as_ref(&self) -> &<Tag1 as ListTag>::Trait {
            self
        }
    }
    impl AsMut<<Tag1 as ListTag>::Trait> for Test {
        fn as_mut(&mut self) -> &mut <Tag1 as ListTag>::Trait {
            self
        }
    }
    impl Implemented<<Tag1 as ListTag>::Trait> for Test {}
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
    let mut ptr = test_list_1.head_next();
    while !ptr.get_next().is_null() {
        println!("{:?}", ptr.as_ref());
        ptr = ptr.get_next();
    }
    println!(" ");
    let mut ptr = test_list_2.head_next();
    while !ptr.get_next().is_null() {
        println!("{:?}", ptr.as_mut());
        ptr = ptr.get_next();
    }

}