use crossterm::event::DisableMouseCapture;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::error::Error;
use std::io::{self, stdout};
use std::rc::Rc;

mod app;
mod cron;
mod ui;

use crate::app::InputState;
use crate::cron::CronTask;
use crate::{
    app::{App, CurrentTab},
    ui::*,
};

//boilerplate
fn main() -> Result<(), Box<dyn Error>> {
    //Terminal init
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    //Main APP loop
    while !app.exit {
        //todo? for self, learn closures more, might be helpful
        terminal.draw(|f| render_ui(f, app))?;
        key_handler(app);
        app.change_menu();
    }
    Ok(true)
}
//todo? Add selection of the new item, change the screen state depending on the selected type. Make new menus.
fn key_handler(app: &mut App) {
    if let Ok(Event::Key(key)) = event::read() {
        if key.kind == event::KeyEventKind::Press {
            //General Key Binds
            match key.code {
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
                    KeyCode::Up => app.next_input(),

                    _ => {}
                },

                CurrentTab::New => match app.input_mode {
                    InputState::Idle => match key.code {
                        KeyCode::Char('n') => {
                            app.input_mode = InputState::Minute;
                        }
                        KeyCode::Esc => {}
                        _ => {}
                    },

                    InputState::Minute => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputState::Idle;
                        }

                        KeyCode::Char(c) => {
                            app.minute_buffer.push(c);
                        }

                        KeyCode::Backspace => {
                            app.minute_buffer.pop();
                        }

                        KeyCode::Enter => {
                            app.input_mode = InputState::Hour;
                        }

                        _ => {}
                    },

                    InputState::Hour => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputState::Idle;
                        }

                        KeyCode::Char(c) => {
                            app.hour_buffer.push(c);
                        }

                        KeyCode::Backspace => {
                            app.hour_buffer.pop();
                        }

                        KeyCode::Enter => {
                            app.input_mode = InputState::Periodic;
                        }

                        _ => {}
                    },

                    InputState::Periodic => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputState::Idle;
                        }
                        KeyCode::Char('p') => {
                            if app.periodic_buffer {
                                app.periodic_buffer = false;
                                app.periodic_text = String::from("Once")
                            } else {
                                app.periodic_buffer = true;
                                app.periodic_text = String::from("Periodic")
                            }
                        }
                        KeyCode::Enter => {
                            app.input_mode = InputState::Weekday;
                        }

                        _ => {}
                    },

                    InputState::Weekday => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputState::Idle;
                        }

                        KeyCode::Char(c) => {
                            app.weekday_buffer.push(c);
                        }

                        KeyCode::Backspace => {
                            app.weekday_buffer.pop();
                        }

                        KeyCode::Enter => {
                            app.input_mode = InputState::Script;
                        }

                        _ => {}
                    },

                    InputState::Script => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputState::Idle;
                        }

                        KeyCode::Char(c) => {
                            app.command_buffer.push(c);
                        }

                        KeyCode::Backspace => {
                            app.command_buffer.pop();
                        }

                        KeyCode::Enter => {
                            app.input_mode = InputState::Confirm;
                        }

                        _ => {}
                    },

                    InputState::Confirm => match key.code {
                        KeyCode::Esc => app.input_mode = InputState::Idle,

                        KeyCode::Enter => {
                            app.push_task();
                            app.input_mode = InputState::Idle;
                        }

                        KeyCode::Backspace => {
                            app.input_mode = InputState::Minute;
                        }

                        _ => {}
                    },
                },

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
