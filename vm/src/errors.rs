use std::fmt::{Debug, Formatter};

pub struct MutabilityError(bool);
impl MutabilityError{
    pub fn new(mutable:bool)->Self{
        Self(mutable)
    }
}
impl Debug for MutabilityError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0{
            write!(f,"Can't access object mutably with the reference")
        }else{
            write!(f,"Can't access object constant with the reference")
        }
    }
}
impl Into<Error> for MutabilityError{
    fn into(self) -> Error {
        Error::MutabilityError(self)
    }
}

pub enum Error{
    MutabilityError(MutabilityError),

}

pub(crate) type Result<T> = std::result::Result<T,Error>;
