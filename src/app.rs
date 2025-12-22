use std::{
    collections::HashMap,
    io,
    rc::Rc,
    sync::{
        Arc, Barrier, Mutex,
        mpsc::{self, RecvError},
    },
    thread,
    time::{Duration, Instant},
    vec,
};

use ratatui::{
    CompletedFrame, Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Backend,
};
use thiserror::Error;

use crate::view::{Action, Time, View};

pub struct App<B, A, L>
where
    B: Backend,
    L: AppLayout,
{
    views: Vec<(Box<dyn View<A>>, bool)>,
    layout: L,
    terminal: Terminal<B>,
    assets: A,
    frame_duration: Duration,
}

impl<B: Backend, A, L: AppLayout> App<B, A, L> {
    pub fn new(
        terminal: Terminal<B>,
        assets: A,
        layout: L,
        start_view: Box<dyn View<A>>,
        fps: f64,
    ) -> Self {
        App {
            views: vec![(start_view, true)],
            layout,
            terminal,
            assets,
            frame_duration: Duration::from_secs_f64(1.0 / fps),
        }
    }

    fn add_views(&mut self, view: Vec<Box<dyn View<A>>>) {}

    pub fn run(&mut self) -> Result<(), AppError> {
        let start_time = Instant::now();
        let mut frame_time = Time {
            time: Duration::ZERO,
            delta: self.frame_duration,
        };

        loop {
            let start_frame = Instant::now();

            let barrier = Barrier::new(self.views.len());
            let (draw_sender, draw) = mpsc::sync_channel(1);
            let (stop_sender, stop) = mpsc::sync_channel(1);
            let (view_sender, spawn_views) = mpsc::channel::<Vec<Box<dyn View<A>>>>();

            thread::scope(|s| {
                for view in self.views.iter_mut() {
                    s.spawn(|| {
                        let mut action_sink = vec![];
                        view.0.update(&frame_time, &mut action_sink);
                        // wait for all views to update
                        barrier.wait();
                        view.0.after(&frame_time, &mut action_sink);

                        for action in action_sink {
                            match action {
                                Action::Draw => {
                                    let _ = draw_sender.try_send(());
                                }
                                Action::Kill => {
                                    view.0.death();
                                    view.1 = false;
                                }
                                Action::Stop => {
                                    let _ = stop_sender.try_send(());
                                }
                                Action::Transition(to) => {
                                    view.0.death();
                                    *view = (to, true);
                                }
                                Action::Spawn(views) => {
                                    let _ = view_sender.send(views);
                                }
                            }
                        }
                    });
                }
            });

            self.views.retain(|view| view.1);

            // stop?
            if let Ok(_) = stop.try_recv() {
                break;
            }

            if self.views.len() == 0 {
                break;
            }

            if let Ok(_) = draw.try_recv() {
                self.draw(&frame_time)?;
            }

            let elapsed = start_frame.elapsed();
            thread::sleep(self.frame_duration.saturating_sub(elapsed));

            frame_time.time = start_time.elapsed();
            frame_time.delta = start_frame.elapsed();
        }

        // finilize views if Stop was awnsered
        for view in &mut self.views {
            view.0.death();
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&mut self, frame_time: &Time) -> io::Result<CompletedFrame> {
        self.views.sort_by_key(|view| view.0.layer());

        self.views.iter().group_by();

        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());

            for view in &self.views {
                view.0.draw(frame, chunks[0], &mut self.assets, frame_time);
            }
        })
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Draw call to the terminal failed")]
    DrawError(#[from] io::Error),
}

pub trait AppLayout {
    fn layout(element_count: usize, layer: usize, frame_area: Rect) -> Rc<[Rect]>;
}

pub fn default_layout(element_count: usize, frame_area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Fill(1)])
        .split(frame_area)
}
