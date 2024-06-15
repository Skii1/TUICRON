use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::*,
    Frame,
};
use std::rc::Rc;

use crate::app::{App, CurrentTab, CurrentlyEditing};
pub fn render_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Yellow).bold());

    let title = render_title(app).block(title_block);

    f.render_widget(title, chunks[0]);

    let mut body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1)])
        .split(chunks[1]);

    body_layout = screen_layout(f, app, &chunks);
    //Here, the middle segment of the screen is organized differently depending on the selected tab, text, and widget content is specified later

    //todo? remove/implement into edit/new function
    let field_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(body_layout[1]);

    //MAIN MENU
    render_menu(app, f, body_layout[0]);

    //todo? could use helper function / match
    let schedule_block = Paragraph::new("Test test").block(Block::bordered().title("Set Schedule"));

    f.render_widget(schedule_block, field_chunks[0]);

    //todo? could use helper function / match
    let path_block = Paragraph::new("Script Path : /home/pi/scripts/do.sh")
        .block(Block::bordered().title("Script Path"));

    f.render_widget(path_block, field_chunks[1]);

    //FOOTER
    let current_navigation_text = vec![
        // The first half of the text
        match app.selected_tab {
            CurrentTab::Menu => Span::styled("CRON Edit", Style::default().fg(Color::Green)),
            CurrentTab::New => Span::styled("New CRON Job", Style::default().fg(Color::Green)),
            CurrentTab::Edit => {
                Span::styled("Edit Existing CRON Job", Style::default().fg(Color::Green))
            }
            CurrentTab::Options => Span::styled("Options", Style::default().fg(Color::Yellow)),
            CurrentTab::Exit => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White).bold()),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Time => {
                        Span::styled("Editing : Task Schedule", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Script => {
                        Span::styled("Editing : PATH", Style::default().fg(Color::Blue))
                    }
                }
            } else {
                Span::styled("Editing : ", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let tab_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let key_instructions = {
        match app.selected_tab {
            CurrentTab::Menu => Span::styled(
                "Navigate Menu ( ↑ ↓ ← →) | Select (Enter)",
                Style::default().fg(Color::Cyan),
            ),
            CurrentTab::New => Span::styled("Make new etc <N>", Style::default().fg(Color::Yellow)),
            CurrentTab::Edit => Span::styled("Edit etc <E>", Style::default().fg(Color::Red)),
            CurrentTab::Options => Span::styled(
                "Controls <C> | Preferences <P>",
                Style::default().fg(Color::Red),
            ),
            CurrentTab::Exit => Span::styled("Quit", Style::default().fg(Color::Red)),
        }
    };

    //FOOTER
    let key_tips_footer =
        Paragraph::new(Line::from(key_instructions)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(45)])
        .split(chunks[2]);

    f.render_widget(tab_footer, footer_chunks[0]);
    f.render_widget(key_tips_footer, footer_chunks[1]);
    //EXIT MENU
}
//Helper Functions
//render_menu : renders the scrollable main menu. Should remain in the same place
//render_footer : renders the footer, with dynamic text and colour as parameters
//titl
fn render_menu(app: &mut App, f: &mut Frame, area: Rect) {
    let main_menu = List::new(app.items.clone())
        .block(Block::bordered().title("Main Menu"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);
    f.render_stateful_widget(main_menu, area, &mut app.state);
}

fn render_footer() {}

fn render_title(app: &mut App) -> Paragraph {
    match app.selected_tab {
        CurrentTab::Menu => {
            let text = "MENU";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().bg(Color::Rgb(204, 0, 0)))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            title
        }

        CurrentTab::New => {
            let text = "NEW";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().bg(Color::Rgb(204, 0, 0)))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            title
        }

        CurrentTab::Edit => {
            let text = "EDIT";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            title
        }

        CurrentTab::Options => {
            let text = "OPTIONS";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            title
        }

        CurrentTab::Exit => {
            let text = "EXITING";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            title
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn screen_layout(f: &mut Frame, app: &mut App, split_target: &Rc<[Rect]>) -> Rc<[Rect]> {
    match app.selected_tab {
        CurrentTab::Menu => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(split_target[1]);
            mod_layout
        }
        CurrentTab::Edit => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(4),
                    Constraint::Percentage(60),
                    Constraint::Min(1),
                ])
                .split(split_target[1]);
            mod_layout
        }
        CurrentTab::New => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(4),
                    Constraint::Percentage(60),
                    Constraint::Min(1),
                ])
                .split(split_target[1]);
            mod_layout
        }
        CurrentTab::Options => {
            let mut mod_layout = Layout::default()
                .constraints([
                    Constraint::Min(4),
                    Constraint::Percentage(50),
                    Constraint::Min(4),
                ])
                .split(split_target[1]);
            mod_layout
        }

        CurrentTab::Exit => {
            let mut mod_layout = Layout::default()
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(split_target[1]);
                exit_menu(f, app);
            mod_layout
        }
    }
}
//dont overlay, get clear screen
fn exit_menu(f: &mut Frame, app: &mut App) {
    f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
    let popup_block = Block::default()
        .title("Y/N")
        .borders(Borders::TOP)
        .title("Exit?")
        .style(Style::default().bg(Color::Rgb(7, 3, 252)));

    let exit_text = Text::styled(
        "Would you like exit the app? (Y/N)",
        Style::default().fg(Color::White),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
}
