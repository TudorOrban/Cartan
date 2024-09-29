use crate::ui::types::{BrowserState, Message, Tab};

pub mod navigation_handlers;

pub fn handle_message(state: &mut BrowserState, message: Message) {
    match message {
        Message::TabChanged(index) => {
            state.active_tab = index;
        },
        Message::AddressChanged(address) => {
            if let Some(tab) = state.tabs.get_mut(state.active_tab) {
                println!("Address changed to: {}", address);
                let address_clone = address.clone();
                tab.address = address;

                if let Some(last) = tab.history.last() {
                    if last != &address_clone {
                        tab.history.push(address_clone);
                    }
                } else {
                    tab.history.push(address_clone);
                }
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
                address: String::from(""),
                content: String::from("new tab content"),
                history: vec![String::from("")],
            });
            state.active_tab = state.tabs.len() - 1;
        },
        Message::CloseTab(index) => {
            state.tabs.remove(index);
            if state.active_tab >= state.tabs.len() {
                state.active_tab = state.tabs.len() - 1;
            }
        },
        Message::GoBack => {
            navigation_handlers::handle_go_back_message(state);
        },
        Message::GoForward => {
            navigation_handlers::handle_go_forward_message(state);
        },
    }
}