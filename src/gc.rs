use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Gc<T: 'static>(Rc<RefCell<T>>);

impl<T: 'static> Gc<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Gc(Rc::new(RefCell::new(value)))
    }

    #[inline]
    pub fn borrow(&self) -> std::cell::Ref<'_, T> {
        self.0.borrow()
    }

    #[inline]
    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, T> {
        self.0.borrow_mut()
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

impl<T: 'static> fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gc<{}>(refs={})", std::any::type_name::<T>(), self.strong_count())
    }
}
