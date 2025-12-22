use ratatui::prelude::*;
use tuy::app::{self, AppLayout};
use tuy::view::{Action, Time, View};
use tuy::{app::App, assets::Sprite};

fn main() {
    let terminal = ratatui::init();
    let mut app = App::new(
        terminal,
        ExampleAssets {},
        ExampleLayout {},
        Box::new(ExampleView {}),
        60.0,
    );
    app.run();

    // loop {
    //     terminal.draw(draw).expect("failed to draw frame");
    //     if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
    //         break;
    //     }
    // }
    // ratatui::restore();
}

// fn draw(frame: &mut Frame) {
//     use Constraint::{Fill, Length, Min};
//
//     let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
//     let [title_area, main_area, status_area] = vertical.areas(frame.area());
//     let horizontal = Layout::horizontal([Fill(1); 2]);
//     let [left_area, right_area] = horizontal.areas(main_area);
//
//     frame.render_widget(Block::bordered().title("Title Bar"), title_area);
//     frame.render_widget(Block::bordered().title("Status Bar"), status_area);
//     frame.render_widget(Block::bordered().title("Left"), left_area);
//     frame.render_widget(Block::bordered().title("Right"), right_area);
// }

struct ExampleLayout;

impl AppLayout for ExampleLayout {
    fn layout(element_count: usize, layer: usize, frame_area: Rect) -> std::rc::Rc<[Rect]> {
        app::default_layout(element_count, frame_area)
    }
}

struct ExampleView;

impl View<ExampleAssets> for ExampleView {
    fn start(&mut self) {
        todo!()
    }

    fn update(&mut self, time: &Time, action_sink: &mut Vec<Action<ExampleAssets>>) {
        todo!()
    }

    fn draw(&self, frame: &mut ratatui::Frame, area: Rect, assets: &ExampleAssets, time: &Time) {
        todo!()
    }

    fn layer(&self) -> i32 {
        todo!()
    }
}

struct ExampleAssets {}

impl ExampleAssets {
    fn get_sprite<'a>(&self, key: Asset) -> &'a Sprite {
        todo!()
    }
}

enum Asset {
    SomeAsset,
}
