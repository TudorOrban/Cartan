use iced::{widget::{Button, Row, Text, TextInput}, Element, Renderer, Theme};

use super::{elements::icon_button, types::{BrowserState, Message}};


pub fn build_upper_header(state: &BrowserState) -> Row<'_, Message> {
    let mut upper_header = Row::new();

    for (i, tab) in state.tabs.iter().enumerate() {
        let tab_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new(&tab.label))
            .on_press(Message::TabChanged(i))
            .into();
        upper_header = upper_header.push(tab_button);
    }

    let new_tab_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("+"))
        .on_press(Message::NewTab)
        .into();

    Row::new().push(upper_header).push(new_tab_button)
}

pub fn build_navigation_bar(state: &BrowserState) -> Row<'_, Message> {
    let back_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Back"))
        .into();
    let forward_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Forward"))
        .into();
    let reload_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Reload"))
        .into();

    let test_icon_button = icon_button::icon_button("resources/images/arrow-left-solid.png", "", Message::NewTab, Some(50.0), Some(50.0));

    let address_input: Element<Message> = TextInput::<Message, Theme, Renderer>::new(
        "Enter URL...",
        &state.tabs[state.active_tab].address,
    )
    .on_input(Message::AddressInputChanged)
    .on_submit(Message::AddressChanged(state.tabs[state.active_tab].address.clone()))
    .into();

    let downloads_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Downloads"))
        .into();
    let application_menu_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Application Menu"))
        .into();

    Row::new()
        .push(back_button)
        .push(forward_button)
        .push(reload_button)
        .push(test_icon_button)
        .push(address_input)
        .push(downloads_button)
        .push(application_menu_button)
        .height(200)
}
