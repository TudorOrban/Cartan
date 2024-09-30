use iced::Task;

use crate::{renderer::html_parser::{self, traverse}, ui::types::{BrowserState, Message, Tab}};

pub mod navigation_handlers;
pub mod fetch_url;

pub fn handle_message(state: &mut BrowserState, message: Message) -> Task<Message> {
    match message {
        Message::TabChanged(index) => {
            state.active_tab = index;
            return Task::none();
        },
        Message::AddressChanged(address) => {
            return navigation_handlers::handle_navigate_to(state, &address);
        },
        Message::AddressInputChanged(address) => {
            if let Some(tab) = state.tabs.get_mut(state.active_tab) {
                tab.address = address;
            }
            return Task::none();
        },
        Message::ContentFetched(Ok(content)) => {
            if let Some(tab) = state.tabs.get_mut(state.active_tab) {
                let parsed_dom = html_parser::parse_html_content(&content);

                tab.parsed_dom = Some(parsed_dom);
                
                // Use as_ref() to avoid cloning the big parsed_dom
                traverse(tab.parsed_dom.as_ref().unwrap());
            }
            return Task::none();
        },
        Message::ContentFetched(Err(e)) => {
            println!("Error fetching content: {}", e);
            if let Some(tab) = state.tabs.get_mut(state.active_tab) {
                tab.content = format!("Error: {}", e);
            }
            return Task::none();
        }
        Message::NewTab => {
            state.tabs.push(Tab::default());
            state.active_tab = state.tabs.len() - 1;
            return Task::none();
        },
        Message::CloseTab(index) => {
            state.tabs.remove(index);
            if state.active_tab >= state.tabs.len() {
                state.active_tab = state.tabs.len() - 1;
            }
            return Task::none();
        },
        Message::GoBack => {
            navigation_handlers::handle_go_back_message(state);
            return Task::none();
        },
        Message::GoForward => {
            navigation_handlers::handle_go_forward_message(state);
            return Task::none();
        },
    }
}