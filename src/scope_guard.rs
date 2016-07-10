use std::boxed::FnBox;
use std::mem;

pub struct ScopeGuard<'a> {
    pub on_exit: Box<FnBox() + 'a>,
}

impl<'a> ScopeGuard<'a> {
    pub fn new<F>(on_exit: F) -> Self where F: 'a + FnOnce() {
        ScopeGuard {
            on_exit: Box::new(on_exit),
        }    
    }   
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        let mut on_exit: Box<FnBox() + 'a> = Box::new(||{});
        mem::swap(&mut self.on_exit, &mut on_exit);
        on_exit();
    }   
}
