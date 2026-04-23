#![allow(dead_code)]

use std::fmt::Debug;

use eij_tracker::eij::Attribute;
use iced::alignment::Vertical;
use iced::keyboard::key;
use iced::widget::{
    button, center, column, container, markdown, mouse_area, opaque, operation, pick_list, row,
    scrollable, space, stack, svg, text,
};
use iced::{Color, Element, Event, Font, Length, Subscription, Task, Theme, keyboard};
use rand::rngs::ThreadRng;
use tracing::{info, warn};

use eij_tracker::dice::{Dice, DiceMessage};

const LOGO_LIGHT: &[u8] = include_bytes!("../assets/eij_latte.svg");
const LOGO_DARK: &[u8] = include_bytes!("../assets/eij_mocha.svg");
const RULES: &str = include_str!("../assets/rules.md");

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
    selected_theme: Option<Theme>,
    show_rules: bool,
    rules_markdown: markdown::Content,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Exit,
    ToggleTheme,
    SetTheme(Theme),
    DiceMessage(DiceMessage),
    ShowRules,
    HideRules,
    LinkClicked(String),
    Event(iced::Event),
}

impl App {
    pub fn new() -> Self {
        let rules_markdown = markdown::Content::parse(RULES);

        Self {
            theme: Theme::CatppuccinMocha,
            dice: Dice::new(1, 6, 0),
            rng: rand::rng(),
            selected_theme: Some(Theme::CatppuccinMocha),
            show_rules: false,
            rules_markdown,
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
            AppMessage::SetTheme(theme) => {
                self.selected_theme = Some(theme.clone());
                self.theme = theme;
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
            AppMessage::ShowRules => {
                self.show_rules = true;
                Task::none()
            }
            AppMessage::HideRules => {
                self.show_rules = false;
                Task::none()
            }
            AppMessage::LinkClicked(url) => {
                let res = webbrowser::open(&url);
                if let Err(e) = res {
                    warn!("Failed to open link: {}", e);
                }

                Task::none()
            }
        }
    }

    pub fn exit(&mut self) -> Task<AppMessage> {
        iced::exit()
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let header = self.view_header();

        let dice = self.dice.view().map(AppMessage::DiceMessage);
        let rules_button = button("Show Rules").on_press(AppMessage::ShowRules);
        let content = column!["Main Window", dice, rules_button];

        let body = container(scrollable(center(content)))
            .padding(10)
            .align_top(Length::FillPortion(4));

        let spacer = space().height(Length::FillPortion(1));
        let app_body = column![header, spacer, body];

        if self.show_rules {
            modal(app_body, self.view_rules(), AppMessage::HideRules)
        } else {
            app_body.into()
        }
    }

    fn view_header(&self) -> Element<'_, AppMessage> {
        let logo_svg = if self.theme.extended_palette().is_dark {
            svg(svg::Handle::from_memory(LOGO_DARK))
        } else {
            svg(svg::Handle::from_memory(LOGO_LIGHT))
        };

        let logo = container(
            logo_svg
                .width(100)
                .height(100)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(theme.palette().text),
                }),
        )
        .align_top(Length::Shrink)
        .align_left(Length::Shrink);

        let theme_combo = pick_list(
            Theme::ALL,
            self.selected_theme.as_ref(),
            AppMessage::SetTheme,
        )
        .placeholder("Select Theme");

        let theme_selector = row![text("Theme:"), theme_combo]
            .align_y(Vertical::Center)
            .spacing(10);

        row![logo, container(theme_selector).align_right(Length::Fill)]
            .padding(10)
            .into()
    }

    fn view_rules(&self) -> Element<'_, AppMessage> {
        let ok_button = container(button("Ok").on_press(AppMessage::HideRules))
            .padding(10)
            .align_right(Length::Fill);

        let rules = container(scrollable(
            center(
                markdown::view(self.rules_markdown.items(), &self.theme)
                    .map(AppMessage::LinkClicked),
            )
            .padding(20),
        ));

        container(column![rules.max_height(500), ok_button].spacing(20))
            .style(container::bordered_box)
            .max_width(500)
            .into()
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
