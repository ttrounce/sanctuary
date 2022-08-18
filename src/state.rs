use std::{any::Any, collections::HashMap};

use crate::Page;

pub struct ProgramState {
    pub objects: HashMap<String, Box<dyn Any>>,
    pub current_page: Page,
    pub scroll_index: i32,
    pub scroll_index_max: i32,
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            objects: HashMap::new(),
            current_page: Page::CharacterSheet,
            scroll_index: 0,
            scroll_index_max: 0,
        }
    }
}
