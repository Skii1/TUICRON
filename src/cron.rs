use ratatui::text::Span;
use ratatui::widgets::canvas::Line;
use ratatui::widgets::Paragraph;
use crate::app::{App, CurrentTab, InputState};
use crate::ui::render_ui;

pub struct CronTask {
    pub minute: String,
    pub hour: String,
    pub time: String,
    pub weekday: String,
    pub day_of_month: String,
    pub day_of_week: String,
    pub command: String,
    pub periodic_minute: bool,
    pub periodic_hour: bool,
    pub periodic_day: bool,
    pub periodic_day_of_month: bool,
    pub periodic_day_of_week: bool,
    pub tasks: Vec<CronTask>, //this shouldn't be 'apart' of the cron task. the cron task struct should contain the info to formulate a task.
}

impl CronTask {
    pub fn new () -> CronTask {
        CronTask {
            time: String::new(),
            minute: String::new(),
            hour: String::new(),
            weekday: String::new(),
            day_of_month: String::new(),
            day_of_week: String::new(),
            command: String::new(),
            periodic_minute: false,
            periodic_hour: false,
            periodic_day: false,
            periodic_day_of_month: false,
            periodic_day_of_week: false,
            tasks: Vec::new(),
        }
    }

    //todo? find a way to formulate the cron task struct into a list / database vector OF cron tasks.
    pub fn push_input(&mut self, app: &mut App, input_state: InputState) {
        let input = app.input_buffer.clone();

        match input_state {
            InputState::Idle => {}
            InputState::Time => self.time = input,
            InputState::Script => self.command = input,
            InputState::Weekday => self.weekday = input,
            InputState::Confirm => {}
        }
    }
    pub fn form_task(&mut self) {
       let sample = String::from("* * * * *");
        let placeholder = String::from("*m *hr *dom *mon *dow");
        let demo = Paragraph::new("CRON : {self.command} at {self.time} on days {}");
    }

    pub fn print_task(&mut self) -> Paragraph {
        let task = Paragraph::new("CRON : {self.command} at {self.time} on days {self.week}");
        task
    }
    pub fn push_task() {}

    pub fn get_logs() {}

    pub fn get_task() {}

}

