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
            Constraint::Length(1),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(f.size());

    //TITLE
    let title_block = Block::default().style(Style::default().bg(Color::Black).bold());
    let title = render_title(app).block(title_block);
    f.render_widget(title, chunks[0]);

    //Dynamic body layout defenition (mutable)
    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(25),
            Constraint::Fill(1),
        ])
        .split(chunks[1]);

    //MAIN MENU
    let field_layout = screen_layout(f, app, &body_layout);

    render_menu(app, f, body_layout[0]);
    
    //Here, the middle segment of the screen is organized differently depending on the selected tab, text, and widget content is specified later
/*
    //FOOTER : probably to be REMOVED
    let current_navigation_text = vec![
        // The first half of the text
        match app.selected_tab {
            CurrentTab::Menu => Span::styled("CRON Edit", Style::default().fg(Color::Green)),
            CurrentTab::New => Span::styled("New CRON Job", Style::default().fg(Color::Green)),
            CurrentTab::Edit =>  Span::styled("Edit Existing CRON Job", Style::default().fg(Color::Green)),
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
    */
    let key_tips = 
        match app.selected_tab {
            CurrentTab::Menu => "Navigate Menu ( ↑ ↓ ← →) | Select (Enter)",
            CurrentTab::New => "Navigate Blocks (Tab) | Navigate Fields ( ↑ ↓ ← →) | Cancel (Esc) | Confirm New Job (C)",
            CurrentTab::Edit => "Navigate Blocks (Tab) Navigate Fields ( ↑ ↓ ← →) | Cancel (Esc) | Confirm Edit (C)",
            CurrentTab::Options => "Navigate Fields ( ↑ ↓ ← →) | Select Job (Enter) | Cancel (Esc) | Confirm Edit (C)",
            CurrentTab::Exit => "Exit y/n",
        };
        
    //FOOTER
    /*
    let tab_footer = Paragraph::new(Line::from(current_navigation_text)).block(Block::default().borders(Borders::NONE));
     */
    
    let key_tips_footer = render_footer(key_tips);
    
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(5), Constraint::Fill(1)])
        .split(chunks[2]);

    //f.render_widget(tab_footer, footer_chunks[0]);
    f.render_widget(key_tips_footer, footer_chunks[1]);

    //EXIT MENU
}

//Helper Functions
//render_menu : renders the scrollable main menu. Should remain in the same place
//render_footer : renders the footer, with dynamic text and colour as parameters
//title
fn render_menu(app: &mut App, f: &mut Frame, area: Rect) {
    let main_menu = List::new(app.items.clone())
        .block(Block::bordered().title("Main Menu"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED)).add_modifier(Modifier::SLOW_BLINK)
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);
    f.render_stateful_widget(main_menu, area, &mut app.state);
}

fn render_footer(text: &str) -> Paragraph {
    let footer = Paragraph::new(text)
        .style(Style::new().white().bg(Color::Rgb(0, 0, 55)))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    footer
}
fn render_title(app: &mut App) -> Paragraph {
    match app.selected_tab {
        CurrentTab::Menu => {
            let text = "Menu";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().bg(Color::Rgb(204, 0, 0)))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            title
        }

        CurrentTab::New => {
            let text = "New Cron Task";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().bg(Color::Rgb(204, 0, 0)))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            title
        }

        CurrentTab::Edit => {
            let text = "Edit Cron Tasks";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            title
        }

        CurrentTab::Options => {
            let text = "Options";
            let title = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            title
        }

        CurrentTab::Exit => {
            let text = "Exiting...";
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

fn screen_layout(f: &mut Frame, app: &mut App, split_target: &Rc<[Rect]>) {
    match app.selected_tab {
        CurrentTab::Menu => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(25),
                    Constraint::Percentage(80),
                ])
                .split(split_target[1]);

            let context = Paragraph::new("Main menu context")
                .block(Block::bordered().title("Main Menu"));
            let other = Paragraph::new("we are locked in!")
                .block(Block::bordered().title("context"));
            f.render_widget(context, mod_layout[0]);
            f.render_widget(other, mod_layout[1]);
        }
        CurrentTab::Edit => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(25),
                    Constraint::Percentage(80),
                ])
                .split(split_target[1]);

            let context = Paragraph::new("Edit the tab")
                .block(Block::bordered().title("Edit tab"));
            let other = Paragraph::new("we are locked in!")
                .block(Block::bordered().title("Time"));
            f.render_widget(context, mod_layout[0]);
            f.render_widget(other, mod_layout[1]);
        }
        CurrentTab::New => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(25),
                    Constraint::Percentage(80),
                ])
                .split(split_target[1]);

            let context = Paragraph::new("Main menu context")
                .block(Block::bordered().title("context"));
            let other = Paragraph::new("we are locked in!")
                .block(Block::bordered().title("Other"));
            f.render_widget(context, mod_layout[0]);
            f.render_widget(other, mod_layout[1]);
        }
        CurrentTab::Options => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(4),
                    Constraint::Percentage(50),
                    Constraint::Min(4),
                ])
                .split(split_target[1]);
            
            let items = ["Item 1", "Item 2", "Item 3"];
            let options = List::new(items)
                .block(Block::bordered().title("Options List"))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);
            let mut option_state = ListState::default();
            option_state.select(Some(0));
            let context = Paragraph::new("Here's a quick description of what this item does")
                .block(Block::bordered().title("Context"));
            f.render_stateful_widget(options, mod_layout[0], &mut option_state);
            f.render_widget(context, mod_layout[1]);
        }

        CurrentTab::Exit => {
            let mut mod_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(split_target[1]);

            let context = Paragraph::new("Main menu context")
                .block(Block::bordered().title("context"));
            let other = Paragraph::new("we are locked in!")
                .block(Block::bordered().title("Other"));
            f.render_widget(context, mod_layout[0]);
            f.render_widget(other, mod_layout[1]);
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
        .style(Style::default().bg(Color::Rgb(7, 3, 252)).bold().italic());

    let exit_text = Text::styled(
        "Would you like exit the app? (Y/N)",
        Style::default().fg(Color::White).bold(),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
}

fn new_exit_menu(app: &mut App) {

}

fn new_menu(f: &mut Frame, app: &mut App, layout: Rc<[Rect]>) {
    let schedule_block = Paragraph::new("NEW CRON")
        .block(Block::bordered().title("Set Schedule"));
    f.render_widget(schedule_block, layout[0]);

    let path_block = Paragraph::new("NEW Script Path : /home/pi/scripts/do.sh")
        .block(Block::bordered().title("Script Path"));
    f.render_widget(path_block, layout[1]);
}

fn edit_menu(f: &mut Frame, app: &mut App, layout: Rc<[Rect]>) {
    let schedule_block = Paragraph::new("EDIT CRON")
        .block(Block::bordered().title("Modify Schedule"));
    f.render_widget(schedule_block, layout[0]);

    let path_block = Paragraph::new("EDIT Script Path : /home/pi/scripts/do.sh")
        .block(Block::bordered().title("Script Path"));
    f.render_widget(path_block, layout[1]);
}

fn focus_tab (app: &mut App, f: &mut Frame, layout: Rc<[Rect]>) {
    match app.focused_tab {
        CurrentTab::Menu => focus_menu(f, app, layout),
        CurrentTab::New => {}
        CurrentTab::Edit => {}
        CurrentTab::Options => {}
        CurrentTab::Exit => {}
    }
}

fn focus_menu(f: &mut Frame, app: &mut App, layout: Rc<[Rect]>) {
    let context = Paragraph::new("Main menu context")
        .block(Block::bordered().title("context"));
    let other = Paragraph::new("we are locked in!")
        .block(Block::bordered().title("Other"));
    f.render_widget(context, layout[0]);
    f.render_widget(other, layout[1]);
}
fn focus_new(f: &mut Frame, app: &mut App, layout: Rc<[Rect]>) {}
fn focus_edit(f: &mut Frame, app: &mut App, layout: Rc<[Rect]>) {}
fn focus_options(f: &mut Frame, app: &mut App, layout: Rc<[Rect]>) {}
fn focus_exit(f: &mut Frame, app: &mut App, layout: Rc<[Rect]>) {}
