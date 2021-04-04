use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub bridge: Rc<RefCell<CrossComponentBridge<Vec<u8>>>>,
}

pub struct CrossComponentBridge<T> {
    callback: Option<Callback<T>>,
}

impl<T> Default for CrossComponentBridge<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> CrossComponentBridge<T> {
    pub fn new() -> Self {
        Self { callback: None }
    }
    pub fn send(&self, msg: T) {
        if let Some(cb) = &self.callback {
            cb.emit(msg)
        }
    }
    pub fn register_callback(&mut self, callback: Callback<T>) {
        self.callback = Some(callback)
    }
}
