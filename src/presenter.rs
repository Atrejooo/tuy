use crate::{
    screen::Screen,
    view::{Draw, View},
};

pub struct Presenter<V>
where
    V: View,
{
    view: V,
}

impl<V> Presenter<V>
where
    V: View,
{
    fn new(data: V::Data, screen: Screen) -> Self {
        let view = V::new(data);
        Presenter { view }
    }
}

struct GameView {}

struct GameViewData {}

impl View for GameView {
    type Data = GameViewData;

    fn new(data: Self::Data) -> Self {
        todo!()
    }

    fn update(&mut self, data: Self::Data) {
        todo!()
    }
}

impl Draw for GameView {
    fn draw(&self) -> ratatui::Frame {
        todo!()
    }
}
