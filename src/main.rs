pub mod ui;

use iced::{widget::{Button, Column, Row, Text, TextInput}, Element, Renderer, Theme};

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

    let back_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Back"))
        .into();
    let forward_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Forward"))
        .into();
    let reload_button: Element<Message> = Button::<Message, Theme, Renderer>::new(Text::new("Reload"))
        .into();

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

        
    let navigation_bar = Row::new()
        .push(back_button)
        .push(forward_button)
        .push(reload_button)
        .push(address_input)
        .push(downloads_button)
        .push(application_menu_button);

    let content = Text::new(&state.tabs[state.active_tab].content);

    Column::new()
        .push(Row::new().push(upper_header).push(new_tab_button)) 
        .push(navigation_bar)
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
            label: String::from("New Tab"),
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
    label: String,
    address: String,
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    TabChanged(usize),
    AddressChanged(String),
    AddressInputChanged(String),
    NewTab,
}