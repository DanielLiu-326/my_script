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
    pub mod double_ll {
        use std::alloc::{alloc, Layout};
        use std::borrow::Borrow;
        use std::collections::HashMap;
        use std::ffi::c_void;
        use std::fmt::{Debug, Formatter};
        use std::ptr::null_mut;


        pub trait Node<TAG> {
            type Trait: ?Sized;
            fn next(&mut self) -> &mut *mut dyn Node<TAG, Trait = Self::Trait>;
            fn prev(&mut self) -> &mut *mut dyn Node<TAG, Trait = Self::Trait>;
        }
        impl<TAG,Trait> Node<TAG> for *mut dyn Node<TAG,Trait=Trait> {
            type Trait = Trait;

            fn next(&mut self) -> &mut *mut dyn Node<TAG, Trait = Self::Trait> {
                (*self).next()
            }

            fn prev(&mut self) -> &mut *mut dyn Node<TAG, Trait = Self::Trait> {
                (*self).prev()
            }
        }
        pub trait NodeExt<TAG> :Node<TAG>+Borrow<Self::Trait>{
            fn remove(&mut self);
        }
        impl <TAG,T:Node<TAG>+Borrow<Self::Trait>> NodeExt<TAG> for T {
            #[inline]
            fn remove(&mut self) {
                unsafe{
                    **self.prev().next() = *self.next();
                    **self.next().prev() = *self.prev();
                }
            }
        }
        pub struct List<TAG,Trait> {
            head: *mut dyn Node<TAG,Trait=Trait>,
            rear: *mut dyn Node<TAG,Trait=Trait>,
        }
        impl<TAG, Trait> Drop for List<TAG, Trait> {
            fn drop(&mut self) {
                unsafe {
                    std::alloc::dealloc(
                        self.head.cast(),
                        Layout::new::<[HeadPlaceholderNode<TAG>; 2]>(),
                    );
                }
            }
        }
        struct HeadPlaceholderNode<TAG,Trait> {
            next: *mut dyn Node<TAG,Trait=Trait>,
            prev: *mut dyn Node<TAG,Trait=Trait>,
        }
        //此处的AsRef是不安全的。是为了满足TraitBound
        impl<TAG,Trait:?Sized> Node<TAG> for HeadPlaceholderNode<TAG,Trait> {
            type Trait = Trait;

            fn next(&mut self) -> &mut *mut dyn Node<T, Trait=Self::Trait> {
                return &mut self.next;
            }

            fn prev(&mut self) -> &mut *mut dyn Node<T, Trait=Self::Trait> {
                return &mut self.prev;
            }
        }

        impl<TAG, T: ?Sized> List<TAG, T> {
            pub fn new() -> Self {
                unsafe {
                    let help_nodes = alloc(Layout::new::<[HeadPlaceholderNode<TAG, T>; 2]>())
                        .cast::<[HeadPlaceholderNode<TAG, T>; 2]>();

                    *(*help_nodes)[0].prev() =
                        null_mut::<HeadPlaceholderNode<TAG, T>>() as *mut dyn Node<TAG, T>;
                    *(*help_nodes)[0].next() = &mut (*help_nodes)[1] as *mut dyn Node<TAG, T>;
                    *(*help_nodes)[1].prev() = &mut (*help_nodes)[0] as *mut dyn Node<TAG, T>;
                    *(*help_nodes)[1].next() =
                        null_mut::<HeadPlaceholderNode<TAG, T>>() as *mut dyn Node<TAG, T>;
                    Self {
                        head: (&mut (*help_nodes)[0] as *mut dyn Node<TAG, T>),
                        rear: (&mut (*help_nodes)[1] as *mut dyn Node<TAG, T>),
                    }
                }
            }

            pub fn insert_front<U: Node<TAG, T>>(&self, node: *mut U) {
                unsafe {
                    let head = self.head;
                    *(**(*head).next()).prev() = node;
                    *(*head).next() = node;
                    *(*node).next() = *(*head).next();
                    *(*node).prev() = head;
                }
            }

            pub fn insert_back<U: Node<TAG, T>>(&self, node: *mut U) {
                unsafe {
                    let rear = self.rear;
                    *(**(*rear).prev()).next() = node;
                    *(*rear).prev() = node;
                    *(*node).next() = rear;
                    *(*node).prev() = *(*rear).prev();
                }
            }

            pub fn concat_front(&self, other: &Self) {
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

            pub fn concat_back(&self, other: &mut Self) {
                unsafe {
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
            pub fn empty(&self) -> bool {
                unsafe {
                    let ptr = self.head;
                    (*(*ptr).next()).is_null()
                }
            }
        }
        #[test]
        fn test() {
            pub struct Test {
                prev_1: *mut dyn Node<Tag1, dyn Debug>,
                next_1: *mut dyn Node<Tag1, dyn Debug>,
                prev_2: *mut dyn Node<Tag2, dyn Debug>,
                next_2: *mut dyn Node<Tag2, dyn Debug>,
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
                        prev_1: null_mut::<Self>() as *mut dyn Node<Tag1, dyn Debug>,
                        next_1: null_mut::<Self>() as *mut dyn Node<Tag1, dyn Debug>,
                        prev_2: null_mut::<Self>() as *mut dyn Node<Tag2, dyn Debug>,
                        next_2: null_mut::<Self>() as *mut dyn Node<Tag2, dyn Debug>,
                        value,
                    }
                }
            }
            pub struct Tag1();
            pub struct Tag2();
            impl AsRef<dyn Debug> for Test {
                fn as_ref(&self) -> &(dyn Debug + 'static) {
                    return self;
                }
            }
            impl Node<Tag1, dyn Debug> for Test {
                fn next(&mut self) -> &mut *mut (dyn Node<Tag1, dyn Debug> + 'static) {
                    return &mut self.next_1;
                }

                fn prev(&mut self) -> &mut *mut (dyn Node<Tag1, dyn Debug> + 'static) {
                    return &mut self.prev_1;
                }
            }
            impl Node<Tag2, dyn Debug> for Test {
                fn next(&mut self) -> &mut *mut (dyn Node<Tag2, dyn Debug> + 'static) {
                    return &mut self.next_2;
                }

                fn prev(&mut self) -> &mut *mut (dyn Node<Tag2, dyn Debug> + 'static) {
                    return &mut self.prev_2;
                }
            }
            let mut test1 = Test::new(1);
            let mut test2 = Test::new(2);
            let mut test3 = Test::new(3);
            let test_list_1 = List::<Tag1, dyn Debug>::new();
            let test_list_2 = List::<Tag2, dyn Debug>::new();
            test_list_1.insert_back(&mut test3 as *mut Test);
            test_list_1.insert_back(&mut test1 as *mut Test);
            test_list_1.insert_back(&mut test2 as *mut Test);
            test_list_2.insert_back(&mut test1 as *mut Test);
            test_list_2.insert_back(&mut test2);
            test2.remove::<Tag1>();
            unsafe {
                let mut ptr = *(*test_list_1.head).next();
                while !(*ptr).next().is_null() {
                    println!("{:?}", (*ptr).as_ref());
                    ptr = *(*ptr).next();
                }
            }
        }
    }
}
