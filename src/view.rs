use std::{
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use crossterm::event::Event;
use ratatui::{Frame, buffer::Buffer, layout::Rect, widgets::Widget};

pub trait View: Widget {
    fn start(&mut self);

    fn update(&mut self, delta: Duration);

    // render
}

// pub struct DrawData {
//     area: Rect,
//     buf: Buffer,
// }

// pub enum DrawMode {
//     FPS(f32),
//     Draw(bool),
// }
