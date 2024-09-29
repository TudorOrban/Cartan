use iced::{widget::{Button, Image, Row, Text}, Length};

use crate::ui::types::Message;


pub fn icon_button<'a>(
    icon_path: &'a str,
    text: &'a str,
    message: Message,
    width: Option<f32>,
    height: Option<f32>,
) -> Button<'a, Message> {
    let icon_height = height.unwrap_or(30.0);
    let icon_width = width.unwrap_or(icon_height);
    let icon = Image::new(icon_path)
        .width(Length::Fixed(icon_width))
        .height(Length::Fixed(icon_height));
    
    let label = Text::new(text);
    let content = Row::new().push(icon).push(label);

    let button_width = width.unwrap_or(icon_width + 20.0);
    let button_height = height.unwrap_or(icon_height);
    
    Button::new(content)
        .on_press(message)
        .width(Length::Fixed(button_width)) 
        .height(Length::Fixed(button_height))
}