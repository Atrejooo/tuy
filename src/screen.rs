use std::any::Any;

use crate::view::{Draw, View};

pub struct Screen {
    views: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn register_draw(&mut self, draw: Box<dyn Draw>) {}
}
