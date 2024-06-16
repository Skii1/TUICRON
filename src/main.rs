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
                KeyCode::Esc => {
                    app.selected_tab = CurrentTab::Exit;
                }
                KeyCode::Tab => {
                    app.scroll_tab();
                }
                KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                    app.next();
                }
                KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                    app.previous();
                }
                KeyCode::Enter => {
                   // app.focus_tab();
                }
                _ => {}
            };
            match app.focused_tab {
                //Menu Key Binds
                CurrentTab::Menu => match key.code {
                    _ => {}
                },
                CurrentTab::Options => match key.code {
                    _ => {}
                },
                
                CurrentTab::Edit => match key.code {
                    KeyCode::Tab => {
                        //app.next_block();
                    }
                    KeyCode::Enter => {}
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left  => {}
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right  => {}
                    KeyCode::Esc  => {}
                    _ => {}
                },

                CurrentTab::New => match key.code {
                    KeyCode::Char('n') => {}
                    _ => {}
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
