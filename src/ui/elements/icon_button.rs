use iced::{alignment, widget::{Button, Image, Row, Space, Text}, Length};

use crate::ui::types::Message;

use super::styles;


pub fn icon_button<'a>(
    icon_path: &'a str,
    text: &'a str,
    message: Message,
    width: Option<f32>,
    height: Option<f32>,
    icon_height: Option<f32>,
    disabled: bool,
) -> Button<'a, Message> {
    let icon_height = icon_height.unwrap_or(height.unwrap_or(40.0) - 20.0);
    let icon = Image::new(icon_path)
        .width(Length::Fixed(icon_height))
        .height(Length::Fixed(icon_height));
    
    let label = Text::new(text);
    let content = Row::new()
        .push(Space::with_width(Length::Fill))
        .push(icon)
        .push(label)
        .push(Space::with_width(Length::Fill))
        .align_y(alignment::Vertical::Center);

    let button_width = width.unwrap_or(40.0);
    let button_height = height.unwrap_or(40.0);
    
    let padding = (button_height - icon_height) / 2.5;

    let mut button = Button::new(content)
        .width(Length::Fixed(button_width)) 
        .height(Length::Fixed(button_height))
        .padding(padding)
        .style(move |_theme, status| styles::style_header_button(status, disabled));

    if !disabled {
        button = button.on_press(message);
    }

    button
}