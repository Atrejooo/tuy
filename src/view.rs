use std::{
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use crossterm::event::Event;
use ratatui::{Frame, buffer::Buffer, layout::Rect, widgets::Widget};

pub trait View<A>: Send + Sync {
    fn layer(&self) -> i32;

    fn start(&mut self);

    fn update(&mut self, time: &Time) -> Awnser<A>;

    fn after(&mut self, time: &Time) -> Awnser<A> {
        Awnser::Idle
    }

    fn death(&mut self) {}

    fn draw(&self, frame: &mut Frame, area: Rect, assets: &A, time: &Time);
}

pub enum Awnser<A> {
    Idle,
    Draw,
    Transition(Box<dyn View<A>>),
    Kill,
    Stop,
}

#[derive(Default)]
pub struct Time {
    pub time: Duration,
    pub delta: Duration,
}

impl Time {}

// pub struct DrawData {
//     area: Rect,
//     buf: Buffer,
// }

// pub enum DrawMode {
//     FPS(f32),
//     Draw(bool),
// }
