use std::io;
use ratatui::{prelude::*, widgets::*};

const MENU_OPTIONS: [&str; 3] = [
    "Enter", "Options", "Exit"
];
pub enum ScreenState {
    Main,
    Tab,
    List,
    Exit,
}
//GUIDE CODE
pub enum CurrentTab {
    Menu,
    Options,
    Exit,
}
//GUIDE CODE
pub enum CurrentlyEditing {
    Key,
    Value,
}
pub struct App {
    pub selected_screen: ScreenState,
    pub selected_tab: CurrentTab,
    pub key_input: String,
    pub list_index: Option<u8>,
    //pub list_selected: u8, too verbose? unneeded? just get index and process in event handler
    pub tab_index: Option<u8>,
    // pub tab_selected:u8, too verbose? unneeded? just get index and process in event handler
    pub currently_editing: Option<CurrentlyEditing>,
    pub state: ListState,
    pub items: Vec<String>,
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
            currently_editing: None,
            state: ListState::default(),
            items: vec![
                String::from("Item1"),
                String::from("Item2"),
                String::from("Item3")],
        }
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    //app functions
}

//StatefulList Structure
//traits could help with list implementation. Get a trait that increments/decrements using the List struct
/*
pub struct MenuList {
    pub state: ListState,
    pub items: Vec<String>,
}

impl MenuList {
    pub fn with_items(items: Vec<String>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

 */