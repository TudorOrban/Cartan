use crate::ui::types::BrowserState;


pub fn handle_go_back_message(state: &mut BrowserState) {
    if let Some(tab) = state.tabs.get_mut(state.active_tab) {
        if tab.history.len() > 1 {
            tab.history.pop();
            tab.address = tab.history.last().unwrap().clone();
        }
    }
}

pub fn handle_go_forward_message(state: &mut BrowserState) {
    if let Some(tab) = state.tabs.get_mut(state.active_tab) {
        if tab.history.len() > 1 {
            tab.history.pop();
            tab.address = tab.history.last().unwrap().clone();
        }
    }
}