#![allow(dead_code)]

use std::collections::BTreeMap;

use iced::Theme;
use iced::widget::{center, space};
use iced::{Element, Subscription, Task};

use crate::window::{Window, WindowInfo};

mod window;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .theme(App::theme)
        .run()
}

#[derive(Debug)]
struct App {
    theme: Theme,
    windows: BTreeMap<window::Id, Window>,
}

#[derive(Debug, Clone, Copy)]
enum AppMessage {
    WindowOpened(window::Id),
    WindowClosed(window::Id),
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let (_, open) = window::open(window::Settings::default());

        let app = Self {
            theme: Theme::CatppuccinMocha,
            windows: BTreeMap::new(),
        };
        let open = open.map(AppMessage::WindowOpened);

        (app, open)
    }

    pub fn title(&self, window_id: window::Id) -> String {
        self.windows
            .get(&window_id)
            .map(|window| window.title().to_string())
            .unwrap_or_default()
    }

    // pub fn theme(&self, window_id: window::Id) -> Theme {
    //     self.windows
    //         .get(&window_id)
    //         .map(|window| window.theme().clone())
    //         .unwrap_or_else(|| Theme::Dark)
    // }

    pub fn theme(&self, _window_id: window::Id) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::WindowOpened(id) => {
                self.windows.insert(id, Window::Main(WindowInfo::default()));

                Task::none()
            }
            AppMessage::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, AppMessage> {
        let content = self
            .windows
            .get(&window_id)
            .map(|window| window.view(window_id))
            .unwrap_or_else(|| space().into());

        center(content).into()
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        window::close_events().map(AppMessage::WindowClosed)
    }
}
