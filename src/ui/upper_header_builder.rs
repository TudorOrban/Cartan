use iced::{alignment, border::Radius, widget::{button, container, Button, Container, Row, Text}, Background, Border, Color, Element, Length, Renderer, Theme};

use super::{elements::{icon_button, styles}, types::{BrowserState, Message, Tab}};


pub fn build_upper_header(state: &BrowserState) -> Container<'_, Message> {
    let mut upper_header = Row::new();

    for (i, tab) in state.tabs.iter().enumerate() {
        let tab_row = build_tab_container(state, i, tab);
        upper_header = upper_header.push(tab_row);
    }

    let new_tab_button: Element<Message> = Button::new(Text::new("+"))
        .style(|_theme, status| styles::style_header_button(status, false))
        .on_press(Message::NewTab)
        .into();

    let complete_header = Row::new()
        .push(upper_header)
        .push(new_tab_button)
        .height(Length::Fixed(state.ui_config.upper_header_height))
        .align_y(alignment::Vertical::Bottom);
        
    Container::new(complete_header)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(state.ui_config.header_color.0, state.ui_config.header_color.1, state.ui_config.header_color.2))),
            ..Default::default()
        })
        .height(Length::Fixed(state.ui_config.upper_header_height))
        .into()
}

fn build_tab_container<'a>(state: &'a BrowserState, i: usize, tab: &'a Tab) -> Container<'a, Message> {
    let tab_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new(&tab.label))
        .style(move |_theme, status| {
            if i == state.active_tab {
                button::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(30, 144, 255))),
                    text_color: Color::WHITE,
                    ..button::Style::default()
                } 
            } else {
                styles::style_header_button(status, false)
            }
        })
        .on_press(Message::TabChanged(i))
        .into();

    let close_tab_button: Element<Message> = icon_button::icon_button("resources/images/xmark-solid.png", "", Message::CloseTab(i), Some(32.0), Some(32.0), Some(16.0), false)
        .style(|_theme, status| styles::style_header_button(status, false))
        .into();

    let tab_row = Row::new()
        .push(tab_button)
        .push(close_tab_button);

    Container::new(tab_row)
        .width(Length::Shrink)
        .padding(1)
        .style(|_| container::Style {
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_left: 0.0,
                    bottom_right: 0.0,
                },
            },
            ..Default::default()
        })
        .into()
}