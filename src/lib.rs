pub mod gui;
pub mod model;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ImmutableRcWrapper<T> {
    impl_: Rc<RefCell<T>>,
}

impl<T> ImmutableRcWrapper<T> {
    pub fn from_rc(rc: Rc<RefCell<T>>) -> ImmutableRcWrapper<T> {
        ImmutableRcWrapper {impl_: rc}
    }

    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.impl_.borrow()
    }
}

impl<T> Clone for ImmutableRcWrapper<T> {
    fn clone(&self) -> ImmutableRcWrapper<T> {
        ImmutableRcWrapper { impl_: self.impl_.clone() }
    }
}
