use iced::{border::Radius, widget::{container, text_input, Container, Row, TextInput}, Background, Border, Color, Element, Length, Renderer, Theme};

use super::{elements::icon_button, types::{BrowserState, Message, UIConfig}};


pub fn build_navigation_bar(state: &BrowserState) -> Container<'_, Message> {
    let config = UIConfig::default();

    let back_button: Element<Message> = icon_button::icon_button("resources/images/arrow-left-solid.png", "", Message::NewTab, Some(40.0), Some(40.0), Some(20.0)).into();
    let forward_button: Element<Message> = icon_button::icon_button("resources/images/arrow-right-solid.png", "", Message::NewTab, Some(40.0), Some(40.0), Some(20.0)).into();
    let reload_button: Element<Message> = icon_button::icon_button("resources/images/rotate-right-solid.png", "", Message::NewTab, Some(40.0), Some(40.0), Some(20.0)).into();

    let address_input: Element<Message> = TextInput::<Message, Theme, Renderer>::new(
        "Enter URL...",
        "DIAJOIQWJEOIQJOI",
    )
        .style(|_theme, _status| {
            text_input::Style { 
                background: Background::Color(Color::WHITE),
                value: Color::BLACK, 
                border: Border {
                    color: Color::BLACK,
                    width: 0.5,
                    radius: Radius {
                        top_left: 0.0,
                        top_right: 0.0,
                        bottom_left: 0.0,
                        bottom_right: 0.0,
                    }
                },
                icon: Color::WHITE,
                placeholder: Color::from_rgb8(128, 128, 128),
                selection: Color::TRANSPARENT,
            }
        })
        .on_input(Message::AddressInputChanged)
        .on_submit(Message::AddressChanged(state.tabs[state.active_tab].address.clone()))
        .into();

    let downloads_button: Element<Message> = icon_button::icon_button("resources/images/download-solid.png", "", Message::NewTab, Some(40.0), Some(40.0), Some(20.0)).into();
    let application_menu_button: Element<Message> = icon_button::icon_button("resources/images/bars-solid.png", "", Message::NewTab, Some(40.0), Some(40.0), Some(20.0)).into();

    let navigation_bar = Row::new()
        .push(back_button)
        .push(forward_button)
        .push(reload_button)
        .push(address_input)
        .push(downloads_button)
        .push(application_menu_button)
        .height(32.0);

    // Add border with a visual trick
    Container::new(navigation_bar)
        .width(Length::Fill)
        .height(Length::Fixed(41.0))
        .padding(0.5)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(config.header_color.0, config.header_color.1, config.header_color.2))),
            ..Default::default()
        })
        .height(32.0)
        .into()
}

