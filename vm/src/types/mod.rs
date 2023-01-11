use concept::types::*;
use crate::util::UncheckMut;
use super::errors::*;

// Register Type Definitions

pub trait LoadVariableFrom<Val>{
    fn load_variable(val:Val) -> Self;
}

pub trait RegisterType{
    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue;
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue>;
}

pub struct Inline<const MUTABLE:bool,T>(UncheckMut<T>);

impl<const MUTABLE:bool,T> Inline<MUTABLE,T> {
    pub fn new(val:Bool)->Self{
        Self(UncheckMut::new(val))
    }
}

impl<const MUTABLE:bool,T> RegTy for InlineBool<MUTABLE,T>{
    #[inline(always)]
    fn unbox_const(&self) -> RefConstValue {
        self.0.get().into()
    }
    #[inline(always)]
    fn unbox_mut(&self) -> Result<RefMutValue, E> {
        if MUTABLE{
            Ok(self.0.get_mut().into())
        }else{
            Err(MutabilityError::new().into())
        }
    }
}

impl Val for Bool{
    #[inline(always)]
    fn load_variable(&self, mutable: bool) -> RegType {
        if mutable{
            RegType::InlineBool(InlineBool::new(*self))
        }else{
            RegType::ConstInlineBool(InlineBool::new(*self))
        }
    }
}

