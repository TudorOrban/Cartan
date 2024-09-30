use kuchiki::NodeRef;


pub struct BrowserState {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub ui_config: UIConfig,
}

impl Default for BrowserState {
    fn default() -> Self {
        let default_tab = Tab::default();
        BrowserState {
            tabs: vec![default_tab],
            active_tab: 0,
            ui_config: UIConfig::default(),
        }
    }
}

pub struct Tab {
    pub label: String,
    pub address: String,
    pub search_query: String,
    pub content: String,
    pub parsed_dom: Option<NodeRef>,
    pub tab_history: TabHistory,
}

pub struct TabHistory {
    pub history: Vec<String>,
    pub forward_stack: Vec<String>,
}

impl Default for Tab {
    fn default() -> Self {
        Tab {
            label: String::from("New Tab"),
            address: String::from(""),
            search_query: String::from(""),
            content: String::from(""),
            parsed_dom: None,
            tab_history: TabHistory {
                history: vec![String::from("")],
                forward_stack: vec![],
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TabChanged(usize),
    AddressChanged(String),
    AddressInputChanged(String),
    ContentFetched(Result<String, String>),
    NewTab,
    CloseTab(usize),
    GoBack,
    GoForward,
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
