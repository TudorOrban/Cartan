use iced::{alignment, border::Radius, widget::{container, text_input, Container, Row, TextInput}, Background, Border, Color, Element, Length, Renderer, Theme};

use super::{elements::icon_button, types::{BrowserState, Message}};


pub fn build_navigation_bar(state: &BrowserState) -> Container<'_, Message> {
    let tab = &state.tabs[state.active_tab];
    let back_disabled = tab.tab_history.history.len() <= 1;
    let forward_disabled = tab.tab_history.forward_stack.is_empty();

    let back_button: Element<Message> = icon_button::icon_button("resources/images/arrow-left-solid.png", "", Message::GoBack, Some(48.0), Some(48.0), Some(30.0), back_disabled).into();
    let forward_button: Element<Message> = icon_button::icon_button("resources/images/arrow-right-solid.png", "", Message::GoForward, Some(48.0), Some(48.0), Some(30.0), forward_disabled).into();
    let reload_button: Element<Message> = icon_button::icon_button("resources/images/rotate-right-solid.png", "", Message::NewTab, Some(48.0), Some(48.0), Some(30.0), false).into();

    let address_input: Element<Message> = TextInput::<Message, Theme, Renderer>::new(
        "Enter URL...",
        &state.tabs[state.active_tab].address,
    )
        .style(|_theme, _status| {
            text_input::Style { 
                background: Background::Color(Color::WHITE),
                value: Color::BLACK, 
                border: Border {
                    color: Color::BLACK,
                    width: 0.5,
                    radius: Radius {
                        top_left: 4.0,
                        top_right: 4.0,
                        bottom_left: 4.0,
                        bottom_right: 4.0,
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

    let downloads_button: Element<Message> = icon_button::icon_button("resources/images/download-solid.png", "", Message::NewTab, Some(48.0), Some(48.0), Some(30.0), false).into();
    let application_menu_button: Element<Message> = icon_button::icon_button("resources/images/bars-solid.png", "", Message::NewTab, Some(48.0), Some(48.0), Some(30.0), false).into();

    let navigation_bar = Row::new()
        .push(back_button)
        .push(forward_button)
        .push(reload_button)
        .push(address_input)
        .push(downloads_button)
        .push(application_menu_button)
        .align_y(alignment::Vertical::Center)
        .height(40.0);

    // Add border with a visual trick
    Container::new(navigation_bar)
        .width(Length::Fill)
        .height(Length::Fixed(41.0))
        .padding(0.5)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(state.ui_config.header_color.0, state.ui_config.header_color.1, state.ui_config.header_color.2))),
            ..Default::default()
        })
        .height(40.0)
        .into()
}

