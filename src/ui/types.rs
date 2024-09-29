
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

#[derive(Debug, Clone, Copy)]
pub struct UIConfig {
    pub upper_header_height: f32,
    pub header_color: (u8, u8, u8), 
    pub header_height: u32,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            upper_header_height: 40.0,
            header_color: (0, 100, 200), 
            header_height: 50,
        }
    }
}

impl UIConfig {
    pub fn new(upper_header_height: f32, header_color: (u8, u8, u8), header_height: u32) -> Self {
        Self {
            upper_header_height,
            header_color,
            header_height,
        }
    }
}
