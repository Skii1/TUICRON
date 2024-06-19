use std::error::Error;
use crossterm::event::DisableMouseCapture;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};
use std::rc::Rc;

mod app;
mod ui;
mod cron;

use crate::{
    app::{App, CurrentTab},
    ui::*,
};
use crate::app::InputState;
use crate::cron::CronTask;

//boilerplate
fn main() -> Result<(), Box<dyn Error>> {
    //Terminal init
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let mut cron = CronTask::new();
    let res = run_app(&mut terminal, &mut app, &mut cron);

    //Terminal Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        (LeaveAlternateScreen),
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, cron: &mut CronTask) -> io::Result<bool> {
    //Main APP loop
    while !app.exit {
        //todo? for self, learn closures more, might be helpful
        terminal.draw(|f| render_ui(f, app, cron))?;
        key_handler(app, cron);
        app.change_menu();
    }
    Ok(true)
}
//todo? Add selection of the new item, change the screen state depending on the selected type. Make new menus.
fn key_handler(app: &mut App, cron: &mut CronTask) {
    if let Ok(Event::Key(key)) = event::read() {
        if key.kind == event::KeyEventKind::Press {
            //General Key Binds
            match key.code {
                KeyCode::Esc => {
                    app.selected_tab = CurrentTab::Exit;
                }
                KeyCode::Tab => {
                    app.scroll_tab();
                }
                _ => {}
            };
            match app.selected_tab {
                //Menu Key Binds
                CurrentTab::Menu => match key.code {
                    _ => {}
                },
                CurrentTab::Options => match key.code {
                    _ => {}
                },
                
                CurrentTab::Edit => match key.code {
                    KeyCode::Enter => {}
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up  => app.next_input(),
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down  => app.previous_input(),
                    _ => {}
                },

                CurrentTab::New => match app.input_mode {
                    InputState::Idle =>
                        match key.code {
                            KeyCode::Char('n') => {
                                app.input_mode = InputState::Time;
                            }
                            KeyCode::Esc => {}
                            _ => {}
                        },

                    InputState::Time =>
                        match key.code {
                            KeyCode::Enter => {
                                cron.minute = app.input_buffer.clone();
                                cron.hour = app.input_buffer.clone();
                                cron.time = app.input_buffer.clone();
                                app.submit_message();
                                app.input_mode = InputState::Script;
                            }

                            KeyCode::Esc => app.input_mode = InputState::Idle,

                            KeyCode::Backspace => app.delete_char(),
                            
                            KeyCode::Char(to_insert) => app.enter_char(to_insert),
                            
                            /* this is mostly pointless, just get text instead
                            KeyCode::Up => {
                                let max:usize = 30;
                                app.inc_buffer(max);
                            },
                            */
                            //KeyCode::Down=> app.dec_buffer(),

                            _ => {}
                        },

                    InputState::Script =>
                        match key.code {
                            KeyCode::Enter => {
                                cron.command = app.input_buffer.clone();
                                app.submit_message();
                                app.input_mode = InputState::Weekday;
                            },

                            KeyCode::Esc => app.input_mode = InputState::Idle,

                            KeyCode::Backspace => app.delete_char(),

                            KeyCode::Char(to_insert) => app.enter_char(to_insert),

                            KeyCode::Up => app.next_input(),

                            KeyCode::Down=> app.previous_input(),

                            _ => {}
                        },

                    InputState::Weekday =>
                        match key.code {
                            KeyCode::Enter => {
                                cron.weekday = app.input_buffer.clone();
                                app.submit_message();
                                app.input_mode = InputState::Confirm;
                            },

                            KeyCode::Esc => app.input_mode = InputState::Idle,

                            KeyCode::Backspace => app.delete_char(),

                            KeyCode::Char(to_insert) => app.enter_char(to_insert),

                            KeyCode::Up => app.next_input(),

                            KeyCode::Down=> app.previous_input(),

                            _ => {}
                        },

                    InputState::Confirm =>
                        match key.code {
                            KeyCode::Esc => app.input_mode = InputState::Idle,

                            KeyCode::Enter => {
                                app.submit_message();
                                app.input_mode = InputState::Time;
                            }

                            KeyCode::Up => app.next_input(),

                            KeyCode::Down => app.previous_input(),

                            _ => {}
                        },
                }

                //Exit Key Binds
                CurrentTab::Exit => match key.code {

                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        app.exit();
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        app.selected_tab = CurrentTab::Menu;
                    }
                    _ => {}

                },
            };
        }
    }
}
