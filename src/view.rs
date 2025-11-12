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
}
