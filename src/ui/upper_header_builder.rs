use iced::{alignment, border::Radius, widget::{button, container, scrollable::{Direction, Scrollbar}, Button, Container, Row, Scrollable, Text}, Background, Border, Color, Element, Length, Renderer, Theme};

use super::{elements::{icon_button, styles}, types::{BrowserState, Message, Tab}};


pub fn build_upper_header(state: &BrowserState) -> Container<'_, Message> {
    let mut tabs_row = Row::new()
        .height(Length::Fixed(state.ui_config.upper_header_height))
        .align_y(alignment::Vertical::Center)
        .spacing(4.0);

    for (i, tab) in state.tabs.iter().enumerate() {
        let tab_row = build_tab_container(state, i, tab);
        tabs_row = tabs_row.push(tab_row);
    }
    
    let new_tab_button: Element<Message> = Button::new(Text::new("+"))
        .style(|_theme, status| styles::style_header_button(status, false))
        .on_press(Message::NewTab)
        .into();
    tabs_row = tabs_row.push(new_tab_button);

    let scrollable_tabs = Scrollable::new(tabs_row)
        .direction(Direction::Horizontal(Scrollbar::new()
            .width(0.1)
            .margin(0)
            .scroller_width(0.1)
            .anchor(iced::widget::scrollable::Anchor::End)
            .spacing(0)
        ))
        .width(Length::FillPortion(18));
        
    Container::new(scrollable_tabs)
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

    let close_tab_button: Element<Message> = icon_button::icon_button("resources/images/xmark-solid.png", "", Message::CloseTab(i), Some(30.0), Some(30.0), Some(15.0), false)
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
                color: Color::from_rgb8(0, 60, 120),
                width: 0.5,
                radius: Radius {
                    top_left: 4.0,
                    top_right: 4.0,
                    bottom_left: 4.0,
                    bottom_right: 4.0,
                },
            },
            ..Default::default()
        })
        .into()
}