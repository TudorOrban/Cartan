use crate::ui::types::{BrowserState, Tab};


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

pub fn handle_address_change(tab: &mut Tab, address: String) {
    if let Some(last) = tab.tab_history.history.last() {
        if last != &address {
            tab.tab_history.history.push(address);
        }
    } else {
        tab.tab_history.history.push(address);
    }

    if tab.tab_history.forward_stack.len() > 0 {
        tab.tab_history.forward_stack.clear();
    }
}