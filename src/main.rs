pub mod ui;
pub mod handlers;

use iced::{widget::{Column, Text}, Element, Task};

use ui::{navigation_bar_builder, types::{BrowserState, Message}, upper_header_builder};


pub fn main() -> iced::Result {
    iced::run("Cartan", update, view)
}

fn update(state: &mut BrowserState, message: Message) -> Task<Message> {
    handlers::handle_message(state, message)
}

fn view(state: &BrowserState) -> Element<Message> {
    let upper_header = upper_header_builder::build_upper_header(state);

    let navigation_bar = navigation_bar_builder::build_navigation_bar(state);

    let content = Text::new(&state.tabs[state.active_tab].content);

    Column::new()
        .push(upper_header) 
        .push(navigation_bar)
        .push(content)
        .into()
}
