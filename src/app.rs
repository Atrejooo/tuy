use std::any::Any;

use crate::view::View;

pub struct App {
    views: Vec<Box<dyn View>>,
}

impl App {}
