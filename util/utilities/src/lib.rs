use std::cell::UnsafeCell;

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
    pub fn get(&self) -> & T{
        &*self.get_mut()
    }
}
