use std::{any::Any, collections::HashMap};

pub struct State {
    pub objects: HashMap<String, Box<dyn Any>>,
}

impl State {

    pub(crate) fn store<T: Any>(&mut self, key: &str, value: T) {
        self.objects.insert(key.to_owned(), Box::new(value));
    }
    
    pub(crate) fn retrieve<'a, T: Any>(&'a mut self, key: &str) -> Option<&'a T> {
        if let Some(boxed) = &self.objects.get(&key.to_owned()) {
            let value = boxed.downcast_ref::<T>();
            return value;
        }
        None
    }
    
    pub(crate) fn retrieve_or<'a, T: Any>(&'a mut self, key: &str, default: &'a T) -> &'a T {
        if let Some(boxed) = &self.objects.get(&key.to_owned()) {
            if let Some(v) = boxed.downcast_ref::<T>() {
                return v;
            }
        }
        default
    }
    
    pub(crate) fn take_or<T: Any>(&mut self, key: &str,  default: T) -> T {
        if let Some(boxed) = self.objects.remove(key) {
            if let Ok(v) = boxed.downcast::<T>() {
                return *v;
            }
        }
        default
    }

    pub(crate) fn take<T: Any>(&mut self, key: &str) -> Option<T> {
        if let Some(boxed) = self.objects.remove(key) {
            if let Ok(v) = boxed.downcast::<T>() {
                return Some(*v);
            }
        }
        None
    }
}
