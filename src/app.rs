use ratatui::{prelude::*, widgets::*};
use std::clone::Clone;
use std::{io, error};
use crate::CronTask;
use itertools::Itertools;
use style::palette::tailwind;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

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

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
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
    Script,
    Weekday,
    Confirm,
}
pub struct App {
    pub selected_tab: CurrentTab,
    pub currently_editing: Option<InputState>,
    pub tab_state: ListState,
    pub tabs: Vec<String>,
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
    pub colors: TableColors,
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
            option: 0,
            exit: false,
            input_state: None,
            tasks: vec![],
            task_list: vec![],
            minute_buffer: String::new(),
            hour_buffer: String::new(),
            weekday_buffer: String::new(),
            command_buffer: String::new(),
            periodic_buffer: false,
            formatted_cron: String::new(),
            colors: TableColors::new(&PALETTES[0]),
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
/*
    pub fn inc_buffer(&mut self, max: usize)  {
        let mut i = self.num_buffer;
                if i == max - 1 {
                    i = 0;
                }
                else {
                    i += 1;
                }
        self.num_buffer = i;
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
    */
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
            Some(0) => self.input_mode = InputState::Minute,
            Some(1) => self.input_mode = InputState::Script,
            None => self.input_mode = InputState::Idle,
            _ => {}
        };
    }
    
    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
    pub fn clear_fields(&mut self) {
        self.minute_buffer.clear();
        self.hour_buffer.clear();
        self.weekday_buffer.clear();
        self.command_buffer.clear();
        self.periodic_buffer = false;
    }
    pub fn push_task(&mut self) {
        let crontask = CronTask::new(
            self.minute_buffer.to_owned(),
            self.hour_buffer.to_owned(),
            self.weekday_buffer.to_owned(),
            self.command_buffer.to_owned(),
            self.periodic_buffer.to_owned(),
        );
        self.clear_fields();
        self.input_mode = InputState::Idle;
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
    pub fn format_task(&mut self) -> String {
        if self.periodic_buffer == true {
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
    pub fn make_task(&mut self, cron: &mut CronTask) {
        let task = CronTask::new(
            self.minute_buffer.to_owned(),
            self.hour_buffer.to_owned(),
            self.command_buffer.to_owned(),
            self.weekday_buffer.to_owned(),
            self.periodic_buffer.to_owned()
        );
        
        self.tasks.push(task);
        self.clear_fields();
        self.input_mode = InputState::Idle
    }
}
