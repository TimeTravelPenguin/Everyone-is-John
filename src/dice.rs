use iced::{
    self, Alignment, Element, Length, Task,
    widget::{self, button, center, column, container, row, space, text},
};
use iced_aw::number_input;
use rand::{Rng, RngExt};

#[derive(Debug)]
pub struct Dice {
    pub quantity: i32,
    pub sides: i32,
    pub modifier: i32,
    result: Option<i64>,
    in_edit_mode: bool,
}

#[derive(Debug, Clone)]
pub enum DiceMessage {
    Roll,
    Clear,
    UpdateQuantity(i32),
    UpdateSides(i32),
    UpdateModifier(i32),
    QuantityInputChanged(String),
    SidesInputChanged(String),
    ModifierInputChanged(String),
    SetEditMode(bool),
}

impl Dice {
    pub fn new(quantity: i32, sides: i32, modifier: i32) -> Self {
        Self {
            quantity,
            sides,
            modifier,
            result: None,
            in_edit_mode: false,
        }
    }

    pub fn roll<R: Rng>(&self, rng: &mut R) -> i64 {
        let mut total = self.modifier as i64;
        for _ in 0..self.quantity {
            total += rng.random_range(1..=self.sides) as i64;
        }

        total
    }

    pub fn update<R: Rng>(&mut self, message: DiceMessage, rng: &mut R) -> Task<DiceMessage> {
        match message {
            DiceMessage::Roll => {
                self.result = Some(self.roll(rng));
                Task::none()
            }
            DiceMessage::Clear => {
                self.result = None;
                Task::none()
            }
            DiceMessage::UpdateQuantity(q) => {
                self.quantity = q;
                Task::none()
            }
            DiceMessage::UpdateSides(s) => {
                self.sides = s;
                Task::none()
            }
            DiceMessage::UpdateModifier(m) => {
                self.modifier = m;
                Task::none()
            }
            DiceMessage::QuantityInputChanged(input) => {
                if let Ok(q) = input.parse() {
                    self.quantity = q;
                }
                Task::none()
            }
            DiceMessage::SidesInputChanged(input) => {
                if let Ok(s) = input.parse() {
                    self.sides = s;
                }
                Task::none()
            }
            DiceMessage::ModifierInputChanged(input) => {
                if let Ok(m) = input.parse() {
                    self.modifier = m;
                }
                Task::none()
            }
            DiceMessage::SetEditMode(edit_mode) => {
                self.in_edit_mode = edit_mode;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, DiceMessage> {
        let view = if self.in_edit_mode {
            self.edit_view()
        } else {
            self.result_view()
        };

        center(view)
            .padding(10)
            .width(160)
            .style(widget::container::bordered_box)
            .into()
    }

    fn result_view(&self) -> Element<'_, DiceMessage> {
        let dice_format = format!(
            "{}d{}{}",
            self.quantity,
            self.sides,
            if self.modifier != 0 {
                format!("{:+}", self.modifier)
            } else {
                String::new()
            }
        );

        let roll_result = if let Some(result) = self.result {
            container(text(format!("Roll: {}", result)))
        } else {
            container(space())
        };

        let buttons = row![
            button("Roll")
                .on_press(DiceMessage::Roll)
                .width(Length::FillPortion(2)),
            button("Edit")
                .style(button::secondary)
                .on_press(DiceMessage::SetEditMode(true))
                .width(Length::FillPortion(1)),
            // button("Clear").on_press(DiceMessage::Clear),
        ]
        .spacing(10);

        column![
            row![text("Dice:"), text(dice_format)].spacing(10),
            roll_result,
            container(buttons).center_x(Length::Fill)
        ]
        .spacing(5)
        .into()
    }

    fn edit_view(&self) -> Element<'_, DiceMessage> {
        let quantity = number_input(&self.quantity, 1.., DiceMessage::UpdateQuantity).width(50);

        let sides = number_input(&self.sides, 1.., DiceMessage::UpdateSides).width(50);

        let modifier = number_input(
            &self.modifier,
            i32::MIN..=i32::MAX,
            DiceMessage::UpdateModifier,
        )
        .width(50);

        let quantity_row = row![
            text("Quantity:").width(50).align_x(Alignment::End),
            quantity
        ]
        .align_y(Alignment::Center)
        .spacing(10);

        let sides_row = row![text("Sides:").width(50).align_x(Alignment::End), sides]
            .align_y(Alignment::Center)
            .spacing(10);

        let modifier_row = row![
            text("Modifier:").width(50).align_x(Alignment::End),
            modifier
        ]
        .align_y(Alignment::Center)
        .spacing(10);

        let submit = button("Submit")
            .on_press(DiceMessage::SetEditMode(false))
            .width(100);

        column![
            container(column![quantity_row, sides_row, modifier_row].spacing(5))
                .center_x(Length::Fill),
            container(row![submit]).center_x(Length::Fill)
        ]
        .spacing(10)
        .into()
    }
}
