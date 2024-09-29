pub mod ui;

use iced::{widget::{Column, Text}, Element};

use ui::{types::{BrowserState, Message, Tab}, ui_builder};

pub fn main() -> iced::Result {
    iced::run("Cartan", update, view)
}

fn update(state: &mut BrowserState, message: Message) {
    match message {
        Message::TabChanged(index) => {
            state.active_tab = index;
        },
        Message::AddressChanged(address) => {
            if let Some(tab) = state.tabs.get_mut(state.active_tab) {
                tab.address = address;
            }
        },
        Message::AddressInputChanged(address) => {
            if let Some(tab) = state.tabs.get_mut(state.active_tab) {
                tab.address = address;
            }
        },
        Message::NewTab => {
            state.tabs.push(Tab {
                label: String::from("New Tab"),
                address: String::new(),
                content: String::from("new tab content"),
            });
            state.active_tab = state.tabs.len() - 1;
        },
    }
}

fn view(state: &BrowserState) -> Element<Message> {
    let upper_header = ui_builder::build_upper_header(state);

    let navigation_bar = ui_builder::build_navigation_bar(state);

    let content = Text::new(&state.tabs[state.active_tab].content);

    Column::new()
        .push(upper_header) 
        .push(navigation_bar)
        .push(content)
        .into()
}
