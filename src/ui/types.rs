
pub struct BrowserState {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
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

pub struct Tab {
    pub label: String,
    pub address: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabChanged(usize),
    AddressChanged(String),
    AddressInputChanged(String),
    NewTab,
    CloseTab(usize),
}