pub mod ui;
pub mod handlers;
pub mod renderer;

use iced::{widget::{Canvas, Column}, Element, Length, Task};

use renderer::types::HTMLCanvas;
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

    let html_canvas = HTMLCanvas::new(state.tabs[state.active_tab].parsed_dom.clone());
    let canvas = Canvas::new(html_canvas)
        .width(Length::Fill)
        .height(Length::Fill);

    Column::new()
        .push(upper_header) 
        .push(navigation_bar)
        .push(canvas)
        .into()
}
