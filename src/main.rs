use crossterm::event::DisableMouseCapture;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

mod app;
mod ui;

use crate::{
    app::{App, CurrentMenu, CurrentTab},
    ui::*,
};

//boilerplate
fn main() -> io::Result<()> {
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
        terminal.draw(|f| ui(f, app))?;
        key_handler(app);
    }
    Ok(true)
}
//todo? Add selection of the new item, change the screen state depending on the selected type. Make new menus.
fn key_handler(app: &mut App) {
    if let Ok(Event::Key(key)) = event::read() {
        if key.kind == event::KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => {
                    app.selected_tab = CurrentTab::Exit;
                }
                KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                    app.next();
                }
                KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                    app.previous();
                }
                KeyCode::Enter => {
                    app.change_menu();
                }
                _ => {}
            };
            match app.selected_tab {
                //Main Menu key binds
                CurrentTab::Menu => match key.code {
                    KeyCode::Char('q') => {
                        app.selected_tab = CurrentTab::Options;
                    }
                    _ => {}
                },
                //Options Key Binds
                CurrentTab::Options => match key.code {
                    // KeyCode::Char('v') =>
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
                CurrentTab::Edit => match key.code {
                    KeyCode::Char('n') => {}
                    _ => {}
                },

                CurrentTab::New => match key.code {
                    KeyCode::Char('n') => {}
                    _ => {}
                },
            };
        }
    }
}
