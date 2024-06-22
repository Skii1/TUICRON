use ratatui::{prelude::*, widgets::*};
use std::clone::Clone;
use std::{io, error};
use crate::CronTask;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}
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
    Minute,
    Hour,
    Periodic,
    Script,
    Weekday,
    Confirm,
}
pub struct App {
    pub selected_tab: CurrentTab,
    pub currently_editing: Option<InputState>,
    pub tab_state: ListState,
    pub tabs: Vec<String>,
    pub fields: Vec<String>,
    pub option: usize,
    pub exit: bool,
    pub character_index: usize,
    pub input_mode: InputState,
    pub messages: Vec<String>,
    pub input_state: Option<usize>,
    pub tasks: Vec<CronTask>,
    pub task_list: Vec<CronTask>,
    pub minute_buffer: String,
    pub hour_buffer: String,
    pub weekday_buffer:String,
    pub command_buffer: String,
    pub periodic_buffer: bool,
    pub formatted_cron: String,
    pub periodic_text: String
}

//App method, pass to main
impl App {
    pub fn new() -> App {
        //initialize values part of App struct
        App {
            selected_tab: CurrentTab::Menu,
            character_index: 0,
            input_mode: InputState::Idle,
            messages: Vec::new(), 
            currently_editing: None,
            tab_state: ListState::default().with_selected(Some(0)),
            tabs: vec![
                String::from(" Menu "),
                String::from(" New "),
                String::from(" Edit "),
                String::from(" Options "),
                String::from(" Exit "),
            ],
            fields: vec![
                String::from(" Minute "),
                String::from(" Hour "),
                String::from(" Period "),
                String::from(" Weekday"),
                String::from(" Command "),
                String::from(" Confirm "),
            ],
            option: 0,
            exit: false,
            input_state: Some(0),
            tasks: vec![],
            task_list: vec![],
            minute_buffer: String::new(),
            hour_buffer: String::new(),
            weekday_buffer: String::new(),
            command_buffer: String::new(),
            periodic_buffer: false,
            formatted_cron: String::new(),
            periodic_text: String::new(),
        }
    }
 
    //Input state vs Input mode. Input state is the numerical index (usize), which is translated into "input mode", with the InputState type enum.
    pub fn previous_field(&mut self) {
        let i = match self.input_state {
            Some(i) => {
                if i >= self.fields.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.input_state = Some(i);
        self.change_input();
    }
    
    pub fn next_field(&mut self) {
        let i = match self.input_state {
            Some(i) => {
                if i == 0 {
                    self.fields.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.input_state = Some(i);
        self.change_input();
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

    //unused helper function, used for scrolling, keep for implementation
    pub fn change_input(&mut self) {
        match self.input_state {
            Some(0) => self.input_mode = InputState::Minute,
            Some(1) => self.input_mode = InputState::Hour,
            Some(2) => self.input_mode = InputState::Periodic,
            Some(3) => self.input_mode = InputState::Weekday,
            Some(4) => self.input_mode = InputState::Script,
            Some(5) => self.input_mode = InputState::Confirm,
            None => self.input_mode = InputState::Idle,
            _ => {}
        };
    }
    pub fn clear_fields(&mut self) {
        self.minute_buffer.clear();
        self.hour_buffer.clear();
        self.weekday_buffer.clear();
        self.command_buffer.clear();
        self.periodic_buffer = false;
    }
    pub fn push_task(&mut self) {
        self.periodic_text = if self.periodic_buffer {
            String::from("Periodic")
        }
        else{
            String::from("On")
        };
        
        let crontask = CronTask::new(
            self.minute_buffer.to_owned(),
            self.hour_buffer.to_owned(),
            self.weekday_buffer.to_owned(),
            self.command_buffer.to_owned(),
            self.periodic_text.to_owned(),
        );
        
        self.tasks.push(crontask);
        self.clear_fields();
        self.input_mode = InputState::Idle;
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
    pub fn task_format(&mut self) -> String {
        if self.periodic_buffer {
           let periodic = format!("*/{} */{} * * {} {}", self.minute_buffer, self.hour_buffer, self.weekday_buffer, self.command_buffer);
            //self.formatted_cron = periodic.clone();
            periodic
        }
        else {
            let once = format!("{} {} * * {} {}", self.minute_buffer, self.hour_buffer, self.weekday_buffer, self.command_buffer);
            //self.formatted_cron = once.clone();
            once
        }
    }
}
