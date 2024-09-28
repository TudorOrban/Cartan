pub mod ui;

use iced::{widget::{button, text, Button, Column, Row, Text, TextInput}, Element, Renderer, Theme};

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
        Message::NewTab => {
            state.tabs.push(Tab {
                address: String::new(),
                content: String::from("new tab content"),
            });
            state.active_tab = state.tabs.len() - 1;
        },
    }
}

fn view(state: &BrowserState) -> Element<Message> {
    let tabs = state.tabs.iter().enumerate().map(|(i, tab)| {
        button(text(&tab.address))
            .on_press(Message::TabChanged(i))
            .into()
    });

    let new_tab_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("+"))
        .on_press(Message::NewTab)
        .into();

    let address_input: Element<Message> = TextInput::<Message, Theme, Renderer>::new(
        "Enter URL...",
        &state.tabs[state.active_tab].address,
    ).into();

    let content = Text::new(&state.tabs[state.active_tab].content);

    Column::new()
        .push(Row::new().push(Column::with_children(tabs.collect::<Vec<_>>())).push(new_tab_button))
        .push(address_input)
        .push(content)
        .into()
}

struct BrowserState {
    tabs: Vec<Tab>,
    active_tab: usize,
}

impl Default for BrowserState {
    fn default() -> Self {
        let default_tab = Tab {
            address: String::from("https://example.com"),
            content: String::from("Hello, world!"),
        };
        BrowserState {
            tabs: vec![default_tab],
            active_tab: 0,
        }
    }
}

struct Tab {
    address: String,
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    TabChanged(usize),
    AddressChanged(String),
    NewTab,
}