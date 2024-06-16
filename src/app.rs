use std::io;
use std::clone::Clone;
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
pub enum FocusedTab {
    Menu,
    New,
    Edit,
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
    pub tab_state: ListState,
    pub tabs: Vec<String>,
    pub option: usize,
    pub exit: bool,
    pub focused_tab: CurrentTab,
}

//App method, pass to main
impl App {
    pub fn new() -> App {
        //initialize values part of App struct
        App {
            selected_tab: CurrentTab::Menu,
            key_input: String::new(),
            currently_editing: None,
            tab_state: ListState::default().with_selected(Some(0)),
            tabs: vec![
                String::from("Menu"),
                String::from("New"),
                String::from("Edit"),
                String::from("Options"),
                String::from("Exit")],
            option: 0,
            exit: false,
            focused_tab: CurrentTab::Menu,
            //main_layout: Layout
            //body_layout: Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(100)]).split(chunks[1])
        }
    }
    pub fn previous(&mut self) {
        let i = match self.tab_state.selected() {
            Some(i) => {
                if i >= self.tabs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.tab_state.select(Some(i));
    }

    pub fn next(&mut self) {
        let i = match self.tab_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tabs.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.tab_state.select(Some(i));
    }
//Equivalent to the next function, just for scrolling tabs with one key
    pub fn scroll_tab(&mut self) {
                if self.option == self.tabs.len() - 1 {
                    self.option = 0;
                }
                 else {
                    self.option += 1;
                }
    self.tab_state.select(Some(self.option));
            }
    
    pub fn change_menu(&mut self) {
        match self.tab_state.selected() {
            Some(0) => {
                self.selected_tab = CurrentTab::Menu;
            }
            Some(1) => {
                self.selected_tab = CurrentTab::New;
            }
            Some(2) => {
                self.selected_tab = CurrentTab::Edit;
            }
            Some(3) => {
                self.selected_tab = CurrentTab::Options;
            }
            Some(4) => {
                self.selected_tab = CurrentTab::Exit;
            }
            None => {
                self.selected_tab = CurrentTab::Menu;
            }
            _ => {
                self.selected_tab = CurrentTab::Menu;
            }
        };
    }
    pub fn exit(&mut self){
        self.exit = true;
    }
    pub fn focus_tab(&mut self) {
        match self.tab_state.selected() {
            Some(0) => {
                self.focused_tab = CurrentTab::Menu;
            }
            Some(1) => {
                self.focused_tab = CurrentTab::New;
            }
            Some(2) => {
                self.focused_tab = CurrentTab::Edit;
            }
            Some(3) => {
                self.focused_tab = CurrentTab::Options;
            }
            Some(4) => {
                self.focused_tab = CurrentTab::Exit;
            }
            None => {
                self.focused_tab = CurrentTab::Menu;
            }
            _ => {
                self.focused_tab = CurrentTab::Menu;
            }
        };
    }
}

