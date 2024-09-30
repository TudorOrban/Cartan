use iced::{widget::button, Color};


pub fn style_header_button(
    status: button::Status,
    disabled: bool
) -> button::Style {
    if disabled {
        return button::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(200, 200, 200))),
            text_color: Color::from_rgb8(0, 80, 160),
            ..button::Style::default()
        }
    }

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