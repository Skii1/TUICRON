use ratatui::{prelude::*, widgets::*};
use std::clone::Clone;
use std::{io, error};

//GUIDE CODE
pub enum CurrentTab {
    Menu,
    New,
    Edit,
    Options,
    Exit,
}

pub enum Focused {
    Menu,
    Tab,
}
//GUIDE CODE
pub enum InputState {
    Idle,
    Time,
    Script,
    Confirm,
}
pub struct App {
    pub selected_tab: CurrentTab,
    pub currently_editing: Option<InputState>,
    pub tab_state: ListState,
    pub tabs: Vec<String>,
    pub option: usize,
    pub exit: bool,
    pub focused_tab: CurrentTab,
    pub time_input: String,
    pub character_index: usize,
    pub input_mode: InputState,
    pub messages: Vec<String>,
    pub input_state: Option<usize>
    /*
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub day_of_month: u8,
    pub day_of_week: u8,
     */
}

//App method, pass to main
impl App {
    pub fn new() -> App {
        //initialize values part of App struct
        App {
            selected_tab: CurrentTab::Menu,
            time_input: String::new(),
            character_index: 0,
            input_mode: InputState::Idle,
            messages: Vec::new(), 
            currently_editing: None,
            tab_state: ListState::default().with_selected(Some(0)),
            tabs: vec![
                String::from("Menu"),
                String::from("New"),
                String::from("Edit"),
                String::from("Options"),
                String::from("Exit"),
            ],
            option: 0,
            exit: false,
            focused_tab: CurrentTab::Menu,
            input_state: None,
            //main_layout: Layout
            //body_layout: Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(100)]).split(chunks[1])
        }
    }
    //unused helper function
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

    //unused helper function
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

    pub fn previous_input(&mut self) {
        let v = match self.input_state {
            Some(v) => {
                if v >= 1 {
                    0
                } else {
                    v + 1
                }
            }
            None => 0,
        };
        self.input_state = Some(v);
    }
    
    pub fn next_input(&mut self) {
        let v = match self.input_state {
            Some(v) => {
                if v == 0 {
                    1
                } else {
                    v - 1
                }
            }
            None => 0,
        };
        self.input_state = Some(v);
    }
    //Equivalent to the next function, just for scrolling tabs with one key
    pub fn scroll_tab(&mut self) {
        if self.option == self.tabs.len() - 1 {
            self.option = 0;
        } else {
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

    pub fn change_input(&mut self) {
        match self.input_state {
            Some(0) => self.input_mode = InputState::Time,
            Some(1) => self.input_mode = InputState::Script,
            None => self.input_mode = InputState::Idle,
            _ => {}
        };
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
    //unused helper function
   

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

   pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.time_input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&mut self) -> usize {
        self.time_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.time_input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.time_input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.time_input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.time_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.time_input.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        self.messages.push(self.time_input.clone());
        self.time_input.clear();
        self.reset_cursor();
    }
}
