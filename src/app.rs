use std::io;
use ratatui::{prelude::*, widgets::*};

pub enum ScreenState {
    Main,
    Tab,
    List,
    Exit,
}

pub enum CurrentTab {
    Menu,
    Options,
    Exit,
}


pub struct App {
    pub selected_screen: ScreenState,
    pub selected_tab: CurrentTab,
    pub key_input: String,
    pub list_index: Option<u8>,
    //pub list_selected: u8, too verbose? unneeded? just get index and process in event handler
    pub tab_index: Option<u8>,
    // pub tab_selected:u8, too verbose? unneeded? just get index and process in event handler
}

//App method, pass to main
impl App {
    pub fn new() -> App {
        //initialize values part of App struct
        App {
            selected_screen: ScreenState::Main,
            selected_tab: CurrentTab::Menu,
            key_input: String::new(),
            list_index: Some(0),
            tab_index: Some(0),
        }
    }
    //app functions
}