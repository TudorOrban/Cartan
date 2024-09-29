use iced::{widget::button, Color};


pub fn style_header_button(status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(30, 144, 255))),
            text_color: Color::WHITE,
            ..button::Style::default()
        },
        _ => button::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(0, 100, 200))),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }
}