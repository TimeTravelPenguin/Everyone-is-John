use iced::Element;
use iced::widget::{center_x, column, container, scrollable};
use iced::window;

pub use iced::window::{Id, Settings, close_events, open};

use crate::AppMessage;

#[derive(Debug, Default)]
pub struct WindowInfo {}

impl WindowInfo {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub enum Window {
    Main(WindowInfo),
    Rules(WindowInfo),
}

impl Window {
    pub fn info(&self) -> &WindowInfo {
        match self {
            Window::Main(info) | Window::Rules(info) => info,
        }
    }

    pub fn info_mut(&mut self) -> &mut WindowInfo {
        match self {
            Window::Main(info) | Window::Rules(info) => info,
        }
    }

    pub const fn title(&self) -> &str {
        match self {
            Window::Main(_info) => "EiJ Tracker",
            Window::Rules(_info) => "Rules",
        }
    }

    pub fn view(&self, _id: window::Id) -> Element<'_, AppMessage> {
        let content = match self {
            Window::Main(_info) => {
                // TODO
                column!["Main Window"]
            }
            Window::Rules(_info) => {
                column!["Rules"]
            }
        };

        container(scrollable(center_x(content))).padding(10).into()
    }
}
