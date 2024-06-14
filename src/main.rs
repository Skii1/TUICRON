use std::io::{self, stdout};
use crossterm::{event::{self, Event, KeyCode}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, execute};
use crossterm::event::DisableMouseCapture;
use ratatui::{prelude::*, widgets::*};

mod app;
mod ui;

use crate::{
    app::{App, ScreenState, CurrentTab},
    ui::ui,
};

//boilerplate
fn main() -> io::Result<()> {
    //Terminal init
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    //todo? : Should the backend process be in a new "tui" file or stay in main?
    //create app instance
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
    loop {
        terminal.draw(|f| ui(f, app))?;
        key_handler(app);
    }
}


fn key_handler(app: &mut App) {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == event::KeyEventKind::Press {
                match app.selected_tab {
                    CurrentTab::Menu => match key.code {
                        KeyCode::Char('q') => {
                            app.selected_tab = CurrentTab::Options;
                        }
                        KeyCode::Esc => {
                            app.selected_tab = CurrentTab::Exit;
                        }
                        KeyCode::Char('w') => {
                            app.next();
                        }
                        KeyCode::Char('s') => {
                            app.prev();
                        }
                        _ => {}
                    },

                    CurrentTab::Options => match key.code {
                        // KeyCode::Char('v') =>
                        _ => {}
                    },
                    CurrentTab::Exit => match key.code {
                        KeyCode::Char('y') => {
                            //app.exit;
                        }
                        KeyCode::Char('n') => {
                            //return to loop, exit tab
                        }

                        _ => {}
                    },
                }
            }
        }
}
//boilerplate END

//todo? pass ui framework to "App"
//ui and template code HERE. commented while building app structure.
/*
fn ui(frame: &mut Frame) {
   let main_layout = Layout::new(
       Direction::Vertical,
       [
           Constraint::Length(1),
           Constraint::Min(0),
           Constraint::Length(1),
       ],
   )
       //defnine and render MAIN border(s)
       .split(frame.size());
   frame.render_widget(
       Block::new().borders(Borders::TOP).title("APERTURE ESCAPE"),
       main_layout[0],
   );
   frame.render_widget(
       Block::new().borders(Borders::TOP).title("Controls |  Exit Tab \"Alt + C\" | View Files \"Alt + D\" | "),
       main_layout[2],
   );

   //define main menu components/objects
   let mut state = ListState::default().with_selected(Some(0));
   let items = ["Continue", "New Game", "Exit"];
   let main_menu = List::new(items)
       .block(Block::bordered().title("Main Menu"))
       .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
       .highlight_symbol("->")
       .repeat_highlight_symbol(true);

   *state.offset_mut() = 0;
   state.select(Some(0));

   let menupar = Paragraph::new(Text::from("Welcome to Aperture Escape.\nNavigate the Menu using \"W, A, S, D\" or \"Up, Down, Left, Right\""))
       .block(Block::bordered().title("Welcome"))
       .wrap(Wrap { trim: true })
       .scroll((1, 1));

   //split MAIN box into two blocks with constraints
   let inner_layout = Layout::new(
       Direction::Horizontal,
       [Constraint::Percentage(20), Constraint::Percentage(80)],
   )
       //call the split function to MAIN box, render the boxes with components (widgets) added
       .split(main_layout[1]);
   frame.render_stateful_widget(main_menu, inner_layout[0], &mut state);
   frame.render_widget(menupar, inner_layout[1]);
}
*/