use std::rc::Rc;
use yew::prelude::*;
use std::cell::RefCell;

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub bridge: Rc<RefCell<CrossComponentBridge<Vec<u8>>>>,
}

pub struct CrossComponentBridge<T> {
    callback: Option<Callback<T>>,
}

impl<T> CrossComponentBridge<T> {
    pub fn new() -> Self {
        Self { callback: None }
    }
    pub fn send(&self, msg: T) {
        if let Some(cb) = &self.callback {
            cb.emit(msg)
        } else {
            stdweb::console!(log, "?");
        }
    }
    pub fn register_callback(&mut self, callback: Callback<T>) {
        self.callback = Some(callback)
    }
}
