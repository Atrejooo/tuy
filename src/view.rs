use ratatui::Frame;

pub trait View: Draw {
    type Data;
    fn new(data: Self::Data) -> Self
    where
        Self: Sized;
    fn update(&mut self, data: Self::Data);
}

pub trait Draw {
    fn draw(&self) -> Frame;
    fn draw_mode(&self) -> DrawMode;
}

pub enum DrawMode {
    FPS(f32),
    Draw(bool),
}
