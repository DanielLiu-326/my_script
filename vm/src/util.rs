use std::alloc;
use std::alloc::Layout;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::NonNull;


#[inline(always)]
pub fn allocate_value<T>(val:T) -> NonNull<T>  {
    unsafe {
        let mut p_val = allocate::<T>();
        *p_val.as_mut() = val;
        return p_val;
    }
}

#[inline(always)]
pub fn allocate<T>() -> NonNull<T> {
    allocate_raw(size_of::<T>()).cast()
}

#[inline(always)]
pub fn allocate_raw(size: usize) -> NonNull<c_void> {unsafe{
    NonNull::new_unchecked(
        alloc::alloc(
            Layout::from_size_align_unchecked(size, size_of::<usize>())
        ).add(size_of::<usize>()).cast()
    )
}}


#[inline]
pub fn deallocate(ptr: NonNull<c_void>) {
    unsafe {
        let allocated = ptr.cast::<usize>().as_ptr().sub(1);
        alloc::dealloc(
            allocated.cast(),
            Layout::from_size_align_unchecked(*allocated, size_of::<usize>()),
        );
    }
}

pub struct UncheckMut<T>(UnsafeCell<T>);

impl<T:Default> Default for UncheckMut<T>{
    fn default() -> Self {
        Self(UnsafeCell::default())
    }
}

impl<T> UncheckMut<T>{
    #[inline(always)]
    pub fn new(val:T)->Self{
        Self(UnsafeCell::new(val))
    }

    #[inline(always)]
    pub fn get_mut(& self) -> & mut T{unsafe{
        &mut *self.0.get()
    }}

    #[inline(always)]
    pub fn get(& self) -> & T{unsafe{
        &*self.get_mut()
    }}
}

pub mod ptr{

    pub mod thin_dyn {
        use core::marker::PhantomData;
        use core::ops::Deref;
        use core::ops::DerefMut;

        /**
         * 类型
         */
        pub unsafe trait Implemented<Trait: ?Sized>: AsRef<Trait> {}

        #[repr(C)]
        pub struct Obj<T: Implemented<U> + Sized, U: ?Sized> {
            meta: <U as std::ptr::Pointee>::Metadata,
            data: T,
        }

        impl<T: Implemented<U> + Sized, U: ?Sized> From<T> for Obj<T, U> {
            fn from(data: T) -> Self {
                Self {
                    meta: std::ptr::metadata(data.as_ref() as *const U),
                    data,
                }
            }
        }

        impl<U: ?Sized, T: Implemented<U>> Obj<T, U> {
            pub fn meta(&self) -> <U as std::ptr::Pointee>::Metadata {
                self.meta
            }

            pub fn data_mut(&mut self) -> &mut T {
                &mut self.data
            }

            pub fn data(&self) -> &T {
                &self.data
            }

            pub fn trait_ptr<'a>(&'a self) -> ObjPtr<U> {
                ObjPtr {
                    obj: (&self.data as *const T).cast(),
                    phantom: Default::default()
                }
            }

            pub fn trait_ptr_mut<'a>(&'a mut self) -> ObjPtrMut<U> {
                ObjPtrMut {
                    obj: (&mut self.data as *mut T).cast(),
                    phantom: Default::default()
                }
            }
        }


        /**
         *  类型的引用
         */
        #[repr(C)]
        struct Dummy<Trait: ?Sized> {
            meta: <Trait as std::ptr::Pointee>::Metadata,
            x: (),
        }

        pub struct ObjRef<'a, ImplTrait: ?Sized> {
            obj: *const (),
            phantom: PhantomData<&'a ImplTrait>,
        }

        impl<'a, Trait: ?Sized> Deref for ObjRef<'a, Trait> {
            type Target = Trait;

            fn deref(&self) -> &'a Self::Target {
                let dummy: *const Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(
                        &(*dummy).x as *const (),
                        (*dummy).meta
                    )
                }
            }
        }

        pub struct ObjRefMut<'a, ImplTrait: ?Sized> {
            obj: *mut (),
            phantom: PhantomData<&'a ImplTrait>,
        }

        impl<'a, Trait: ?Sized> Deref for ObjRefMut<'a, Trait> {
            type Target = Trait;

            fn deref(&self) -> &Self::Target {
                let dummy: *const Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(
                        &(*dummy).x as *const (),
                        (*dummy).meta
                    )
                }
            }
        }

        impl<'a, Trait: ?Sized> DerefMut for ObjRefMut<'a, Trait> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                let dummy: *mut Dummy<Self::Target> = self.obj.cast();
                unsafe {
                    &mut *std::ptr::from_raw_parts_mut(
                        &mut (*dummy).x as *mut (),
                        (*dummy).meta
                    )
                }
            }
        }

        /**
         * 类型的指针
         */
        pub struct ObjPtr<ImplTrait: ?Sized> {
            obj: *const (),
            phantom: PhantomData<ImplTrait>,
        }

        impl<Trait> Clone for ObjPtr<Trait> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Trait> Copy for ObjPtr<Trait>{}

        impl<Trait: ?Sized> ObjPtr<Trait> {
            pub fn ref_const(&self) -> &Trait {
                let dummy: *const Dummy<Trait> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(
                        & (*dummy).x as *const (),
                        (*dummy).meta
                    )
                }
            }
        }
        impl<ImplTrait> Deref for ObjPtr<ImplTrait> {
            type Target = ImplTrait;

            fn deref(&self) -> &Self::Target {
                self.ref_const()
            }
        }

        pub struct ObjPtrMut<ImplTrait: ?Sized> {
            obj: *mut (),
            phantom: PhantomData<ImplTrait>,
        }

        impl<Trait: ?Sized> ObjPtrMut<Trait> {
            pub fn ref_mut(&self) -> &mut Trait {
                let dummy: *mut Dummy<Trait> = self.obj.cast();
                unsafe {
                    &mut *std::ptr::from_raw_parts_mut(
                        &mut (*dummy).x as *mut (),
                        (*dummy).meta
                    )
                }
            }

            pub fn ref_const(&self) -> &Trait {
                let dummy: *mut Dummy<Trait> = self.obj.cast();
                unsafe {
                    &*std::ptr::from_raw_parts(
                        & (*dummy).x as *const (),
                        (*dummy).meta
                    )
                }
            }
        }

        impl<Trait: ?Sized> Copy for ObjPtrMut<Trait> {}

        impl<Trait: ?Sized> Clone for ObjPtrMut<Trait> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<ImplTrait: ?Sized> Deref for ObjPtrMut<ImplTrait> {
            type Target = ImplTrait;

            fn deref(&self) -> &Self::Target {
                self.ref_const()
            }
        }

        impl<ImplTrait: ?Sized> DerefMut for ObjPtrMut<ImplTrait> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.ref_mut()
            }
        }
    }
}