use iced::{widget::{Button, Image, Row, Text}, Length};

use crate::ui::types::Message;

use super::styles;


pub fn icon_button<'a>(
    icon_path: &'a str,
    text: &'a str,
    message: Message,
    width: Option<f32>,
    height: Option<f32>,
    icon_height: Option<f32>,
) -> Button<'a, Message> {
    let icon_height = icon_height.unwrap_or(height.unwrap_or(40.0) - 15.0);
    let icon = Image::new(icon_path)
        .width(Length::Fixed(icon_height))
        .height(Length::Fixed(icon_height));
    
    let label = Text::new(text);
    let content = Row::new().push(icon).push(label);

    let button_width = width.unwrap_or(40.0);
    let button_height = height.unwrap_or(40.0);
    
    Button::new(content)
        .on_press(message)
        .width(Length::Fixed(button_width)) 
        .height(Length::Fixed(button_height))
        .style(|_theme, status| styles::style_header_button(status))
}