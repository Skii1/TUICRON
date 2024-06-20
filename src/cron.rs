use ratatui::layout::Constraint;
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::text::{Span, Text};
use ratatui::widgets::canvas::Line;
use ratatui::widgets::{Block, Paragraph, Row, Table};
use crate::app::{App, CurrentTab, InputState};
use crate::ui::render_ui;

pub struct CronTask {
    pub minute: String,
    pub hour: String,
    pub weekday: String,
    pub command: String,
    pub periodic: bool,
    //pub tasks: Vec<CronTask>, //this shouldn't be 'apart' of the cron task. the cron task struct should contain the info to formulate a task.
}

impl CronTask {
    pub fn new(minute: String, hour: String, weekday: String, command: String, periodic: bool) -> CronTask {
        CronTask {
            minute,
            hour,
            weekday,
            command,
            periodic,
        }
    }

    //todo? find a way to formulate the cron task struct into a list / database vector OF cron tasks.
    pub fn form_task(&mut self) {
        String::from(format!("{}", self.minute));
        
    }

    pub fn print_task(&mut self) -> Paragraph {
        let task_item = Paragraph::new(format!("CRON : {} at {} on days {}", self.command, self.minute, self.weekday));
        task_item
    }
    pub fn get_logs() {}

    pub fn get_task() {}
}

