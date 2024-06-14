use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::*,
    prelude::*,
    Frame,
};

use crate::app::{App, CurrentTab, CurrentlyEditing};
pub fn ui(f: &mut Frame, app: &App) {
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
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Title here",
        Style::default().fg(Color::Blue),
    ))
        .block(title_block);

    f.render_widget(title, chunks[0]);

    let mut state = ListState::default().with_selected(Some(0));
    let items = ["Continue", "New Game", "Exit"];
    let main_menu = List::new(items)
        .block(Block::bordered().title("Main Menu"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    *state.offset_mut() = 0;
    state.select(Some(2));

    f.render_stateful_widget(main_menu, chunks[1], &mut state);

    let current_navigation_text = vec![
        // The first half of the text
        match app.selected_tab {
            CurrentTab::Menu => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentTab::Options => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentTab::Exit => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
            .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let tab_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let key_instructions = {
        match app.selected_tab {
            CurrentTab::Menu => Span::styled(
            "Options <l> | Exit <Esc>",
            Style::default().fg(Color::Cyan),
            ),
            CurrentTab::Options => Span::styled(
             "Controls <c> | Preferences <p>",
             Style::default().fg(Color::Yellow),
            ),
            CurrentTab::Exit => Span::styled(
                "Quit <q>",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_tips_footer =
    Paragraph::new(Line::from(key_instructions)).block(Block::default().borders(Borders::ALL));

    let footer_chunks =  Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(45)])
        .split(chunks[2]);

    f.render_widget(tab_footer, footer_chunks[0]);
    f.render_widget(key_tips_footer, footer_chunks[1]);

    if let CurrentTab::Exit = app.selected_tab {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area);
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