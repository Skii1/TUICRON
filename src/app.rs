use std::io;
use ratatui::{prelude::*, widgets::*};

//GUIDE CODE
pub enum CurrentTab {
    Menu,
    New,
    Edit,
    Options,
    Exit,
}
//replace with tabs
pub enum CurrentMenu {
    Main,
    Edit,
    View,
    Options,
    Exit,
}
//GUIDE CODE
pub enum CurrentlyEditing {
    Time,
    Script,
}
pub struct App {
    pub selected_tab: CurrentTab,
    pub key_input: String,
    pub currently_editing: Option<CurrentlyEditing>,
    pub state: ListState,
    pub items: Vec<String>,
    pub selected_option: Option<usize>,
    pub exit: bool
}

//App method, pass to main
impl App {
    pub fn new() -> App {
        //initialize values part of App struct
        App {
            selected_tab: CurrentTab::Menu,
            key_input: String::new(),
            currently_editing: None,
            state: ListState::default().with_selected(Some(0)),
            items: vec![
                String::from("New"),
                String::from("Edit"),
                String::from("Options"),
                String::from("Exit")],
            selected_option: Some(0),
            exit: false,
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
    
    pub fn change_menu(&mut self) {
        match self.state.selected() {
            Some(0) => {
                self.selected_tab = CurrentTab::New
            }
            Some(1) => {
                self.selected_tab = CurrentTab::Edit
            }
            Some(2) => {
                self.selected_tab = CurrentTab::Options
            }
            Some(3) => {
                self.selected_tab = CurrentTab::Exit
            }
            None => {
                self.selected_tab = CurrentTab::Menu
            }
            _ => {
                self.selected_tab = CurrentTab::Menu
            }
        };
    }
    pub fn exit(&mut self){
        self.exit = true;
    }
}