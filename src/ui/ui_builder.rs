use iced::{widget::{button, container, Button, Container, Row, Text, TextInput}, Background, Color, Element, Length, Renderer, Theme};

use super::{elements::{icon_button, styles}, types::{BrowserState, Message, Tab}};

pub fn build_upper_header(state: &BrowserState) -> Container<'_, Message> {
    let mut upper_header = Row::new();

    for (i, tab) in state.tabs.iter().enumerate() {
        let tab_row = build_tab_container(state, i, tab);
        upper_header = upper_header.push(tab_row);
    }

    let new_tab_button: Element<Message> = Button::new(Text::new("+"))
        .style(|_theme, status| styles::style_header_button(status))
        .on_press(Message::NewTab)
        .into();

    let complete_header = Row::new()
        .push(upper_header)
        .push(new_tab_button);

    Container::new(complete_header)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(0, 100, 200))),
            ..Default::default()
        })
        .into()
}

fn build_tab_container<'a>(state: &'a BrowserState, i: usize, tab: &'a Tab) -> Row<'a, Message> {
    let tab_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new(&tab.label))
        .style(move |_theme, status| {
            if i == state.active_tab {
                button::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(0, 180, 230))),
                    text_color: Color::WHITE,
                    ..button::Style::default()
                } 
            } else {
                styles::style_header_button(status)
            }
        })
        .on_press(Message::TabChanged(i))
        .into();

    let close_tab_button: Element<Message> = icon_button::icon_button("resources/images/xmark-solid.png", "", Message::CloseTab(i), Some(20.0), Some(20.0))
        .style(|_theme, status| styles::style_header_button(status))
        .into();

    Row::new()
        .push(tab_button)
        .push(close_tab_button)
}

pub fn build_navigation_bar(state: &BrowserState) -> Container<'_, Message> {
    let back_button: Element<Message> = icon_button::icon_button("resources/images/arrow-left-solid.png", "", Message::NewTab, Some(40.0), Some(40.0)).into();
    let forward_button: Element<Message> = icon_button::icon_button("resources/images/arrow-right-solid.png", "", Message::NewTab, Some(40.0), Some(40.0)).into();
    let reload_button: Element<Message> = icon_button::icon_button("resources/images/rotate-right-solid.png", "", Message::NewTab, Some(40.0), Some(40.0)).into();

    let address_input: Element<Message> = TextInput::<Message, Theme, Renderer>::new(
        "Enter URL...",
        &state.tabs[state.active_tab].address,
    )
        .line_height(40.0)
        .on_input(Message::AddressInputChanged)
        .on_submit(Message::AddressChanged(state.tabs[state.active_tab].address.clone()))
        .into();

    let downloads_button: Element<Message> = icon_button::icon_button("resources/images/download-solid.png", "", Message::NewTab, Some(40.0), Some(40.0)).into();
    let application_menu_button: Element<Message> = icon_button::icon_button("resources/images/bars-solid.png", "", Message::NewTab, Some(40.0), Some(40.0)).into();

    let navigation_bar = Row::new()
        .push(back_button)
        .push(forward_button)
        .push(reload_button)
        .push(address_input)
        .push(downloads_button)
        .push(application_menu_button)
        .height(40.0);

    // Add border with a visual trick
    Container::new(navigation_bar)
        .width(Length::Fill)
        .height(Length::Fixed(41.0))
        .padding(0.5)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::BLACK)),
            ..Default::default()
        })
        .into()
}

