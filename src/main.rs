#![allow(dead_code)]

use iced::keyboard::key;
use iced::widget::{center, center_x, column, container, operation, scrollable};
use iced::{Element, Event, Font, Subscription, Task, Theme, keyboard};
use rand::rngs::ThreadRng;
use tracing::info;

use crate::dice::{Dice, DiceMessage};

mod dice;
mod eij;

pub fn main() -> iced::Result {
    #[cfg(not(target_family = "wasm"))]
    tracing_subscriber::fmt::init();

    #[cfg(target_family = "wasm")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title("EiJ Tracker")
        .theme(App::theme)
        .font(include_bytes!("../fonts/nunito sans/NunitoSans-VariableFont.ttf").as_slice())
        .default_font(Font::with_name("Nunito Sans"))
        .centered()
        .run()
}

#[derive(Debug)]
struct App {
    theme: Theme,
    dice: Dice,
    rng: ThreadRng,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Exit,
    ToggleTheme,
    DiceMessage(DiceMessage),
    Event(iced::Event),
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::CatppuccinMocha,
            dice: Dice::new(1, 6, 0),
            rng: rand::rng(),
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    // Listen for global events (like keyboard input) and convert them into messages
    fn subscription(_state: &App) -> Subscription<AppMessage> {
        iced::event::listen().map(AppMessage::Event)
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::Exit => iced::exit(),
            AppMessage::ToggleTheme => {
                self.theme = match self.theme {
                    Theme::CatppuccinMocha => Theme::CatppuccinLatte,
                    Theme::CatppuccinLatte => Theme::CatppuccinMocha,
                    _ => Theme::CatppuccinMocha,
                };
                Task::none()
            }
            AppMessage::DiceMessage(dice_message) => self
                .dice
                .update(dice_message, &mut self.rng)
                .map(AppMessage::DiceMessage),
            AppMessage::Event(event) => {
                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(key::Named::Tab),
                        modifiers,
                        ..
                    }) => {
                        if modifiers.shift() {
                            info!("Shift + Tab pressed");
                            operation::focus_previous()
                        } else {
                            info!("Tab pressed");
                            operation::focus_next()
                        }
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        // Q Key
                        key,
                        modifiers,
                        ..
                    }) => {
                        let not_wasm = !cfg!(target_family = "wasm");

                        if key == keyboard::Key::Character("w".into())
                            && modifiers.command()
                            && not_wasm
                        {
                            info!("Command + W pressed");
                            self.exit()
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
        }
    }

    pub fn exit(&mut self) -> Task<AppMessage> {
        iced::exit()
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let content = column!["Main Window", self.dice.view().map(AppMessage::DiceMessage)];
        let content = container(scrollable(center_x(content))).padding(10);

        center(content).into()
    }
}
