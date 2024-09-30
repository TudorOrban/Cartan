

use iced::Task;

use crate::ui::types::{BrowserState, Message, Tab};

use super::fetch_url;


pub fn handle_go_back_message(state: &mut BrowserState) {
    if let Some(tab) = state.tabs.get_mut(state.active_tab) {
        if tab.tab_history.history.len() > 1 {
            let current_address = tab.address.clone();
            tab.tab_history.forward_stack.push(current_address);

            tab.tab_history.history.pop();
            tab.address = tab.tab_history.history.last().unwrap().clone();
        }
    }
}

pub fn handle_go_forward_message(state: &mut BrowserState) {
    if let Some(tab) = state.tabs.get_mut(state.active_tab) {
        if !tab.tab_history.forward_stack.is_empty() {
            let current_address = tab.address.clone();
            tab.tab_history.history.push(current_address);

            tab.address = tab.tab_history.forward_stack.pop().unwrap();
        }
    }
}

pub fn handle_history_update(tab: &mut Tab, address: &String) {
    if let Some(last) = tab.tab_history.history.last() {
        if last != address {
            tab.tab_history.history.push(address.clone());
        }
    } else {
        tab.tab_history.history.push(address.clone());
    }

    if tab.tab_history.forward_stack.len() > 0 {
        tab.tab_history.forward_stack.clear();
    }
}

pub fn handle_navigate_to(state: &mut BrowserState, address: &String) -> Task<Message> {
    if let Some(tab) = state.tabs.get_mut(state.active_tab) {
        let address_clone = address.clone();
        tab.address = address.clone();

        handle_history_update(tab, &address_clone);

        let url = process_input(&address_clone.clone().as_str());

        return Task::perform(fetch_url::fetch_url(url), Message::ContentFetched);
    } else {
        eprintln!("No active tab found");
        return Task::none();
    }
}

fn process_input(input: &str) -> String {
    let trimmed_input = input.trim();
    if trimmed_input.starts_with("http://") || trimmed_input.starts_with("https://") || trimmed_input.starts_with("www://") {
        trimmed_input.to_string()
    } else {
        format!("https://www.google.com/search?q={}", trimmed_input)
    }
}