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
            Constraint::Length(1), //Tab Toolbar
            Constraint::Length(3), //TEMPORARY ?todo REMOVE
            Constraint::Min(5), //Main Body, main block as Tab title and contents
            Constraint::Length(1), //Footer, key bind tips
        ])
        .split(f.size());

    //TOOLBAR
    render_toolbar(f, app, chunks[0]);

    //TITLE
    render_tab_frame(f, app, chunks[1]);

    //Dynamic body layout defenition (mutable) (now just splits the middle main chunk entirely
    let field_layout = render_tab(f, app, chunks[2]);

    //FOOTER
    render_footer(f, app, chunks[3]);
    //Here, the middle segment of the screen is organized differently depending on the selected tab, text, and widget content is specified later

}



//Helper Functions //
//Menu Styling won't work??? Why???

///new_tab_frame In development function to make outer box for the entire tab frame, use render title for now, called tab render
fn new_tab_frame(f: &mut Frame, app: &mut App, area: Rect) {

        let menu_style = Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow).bg(Color::Rgb(0,0,55));
        let mut menu_text = ratatui::prelude::Line::styled("DEFAULT", menu_style);

        match app.selected_tab {
            CurrentTab::Menu => menu_text = ratatui::prelude::Line::styled("Menu", menu_style),
            CurrentTab::New => menu_text = ratatui::prelude::Line::styled("NEW | Cron Task", menu_style),
            CurrentTab::Edit => menu_text = ratatui::prelude::Line::styled("EDIT | Cron Task", menu_style),
            CurrentTab::Options => menu_text = ratatui::prelude::Line::styled("Options", menu_style),
            CurrentTab::Exit => menu_text = ratatui::prelude::Line::styled("Exit", menu_style),
            _ => {}
        };
        let menu_block = Block::bordered().title(menu_text);
        let tab_block = Block::bordered().title("Inner");
        let inner = menu_block.inner(area);
        f.render_widget(menu_block, area);
        f.render_widget(tab_block, inner);
}
fn render_menu(app: &mut App, f: &mut Frame, area: Rect) {
    let main_menu = List::new(app.tabs.clone())
        .block(Block::bordered().title("Main Menu"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .add_modifier(Modifier::SLOW_BLINK)
        .highlight_symbol("»")
        .repeat_highlight_symbol(true);
    f.render_stateful_widget(main_menu, area, &mut app.tab_state);
}
fn render_toolbar(f: &mut Frame, app: &mut App, area: Rect) {
    let tabs = app.tabs.clone();
    let tab_bar = Tabs::new(tabs)
        .highlight_style(Style::default().yellow().bg(Color::Black))
        .select(app.option)
        .padding(" ", " ")
        .divider(" | ");

    f.render_widget(tab_bar, area);
}
fn render_footer(f: &mut Frame, app: &mut App, area: Rect) {
    let key_tips =
        match app.selected_tab {
            CurrentTab::Menu => "Navigate Tabs (↹ Tab) | Navigate Menu ( ↑ ↓ ← →) | Select (Enter)",
            CurrentTab::New => "Navigate Blocks (Tab) | Navigate Fields ( ↑ ↓ ← →) | Cancel (Esc) | Confirm New Job (C)",
            CurrentTab::Edit => "Navigate Blocks (Tab) Navigate Fields ( ↑ ↓ ← →) | Cancel (Esc) | Confirm Edit (C)",
            CurrentTab::Options => "Navigate Fields ( ↑ ↓ ← →) | Select Job (Enter) | Cancel (Esc) | Confirm Edit (C)",
            CurrentTab::Exit => "Exit y/n",
        };
    let footer = Paragraph::new(key_tips)
        .style(Style::new().white().bg(Color::Rgb(0, 0, 55)))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(footer, area)
}
fn render_tab_frame(f: &mut Frame, app: &mut App, area: Rect) {

    let menu_style = Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow).bg(Color::Rgb(0,0,55));
    let mut menu_text = Text::raw("DEFAULT");


    match app.selected_tab {
        CurrentTab::Menu => menu_text = Text::raw("Menu"),
        CurrentTab::New => menu_text = Text::raw("NEW | Cron Task"),
        CurrentTab::Edit => menu_text = Text::raw("EDIT | Cron Task"),
        CurrentTab::Options => menu_text = Text::raw("Options"),
        CurrentTab::Exit => menu_text = Text::raw("Exit"),
        _ => { }
    };

    let menu_block = Paragraph::new(menu_text)
        .style(menu_style)
        .alignment(Alignment::Center)
        .block(Block::bordered());

    f.render_widget(menu_block, area);
}
fn render_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    match app.selected_tab {
        CurrentTab::Menu => menu_tab(f, app, tab),
        CurrentTab::Edit => edit_tab(f, app, tab),
        CurrentTab::New => new_tab(f, app, tab),
        CurrentTab::Options => options_tab(f, app, tab),
        CurrentTab::Exit => exit_tab(f, app, tab), //why does this work now, wtf???
    }
}

//Tab Layouts //
fn menu_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Percentage(80)])
        .split(tab);

    let context = Paragraph::new("Main menu context").block(Block::bordered().title("context"));
    let other = Paragraph::new("we are locked in!").block(Block::bordered().title("Other"));
    f.render_widget(context, window[0]);
    f.render_widget(other, window[1]);
}
fn new_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Percentage(80)])
        .split(tab);

    let context =
        Paragraph::new("Main menu context").block(Block::bordered().title("context"));
    let other = Paragraph::new("we are locked in!").block(Block::bordered().title("Other"));
    f.render_widget(context, window[0]);
    f.render_widget(other, window[1]);

    let schedule_block = Paragraph::new("NEW CRON").block(Block::bordered().title("Set Schedule"));
    f.render_widget(schedule_block, window[0]);

    let path_block = Paragraph::new("NEW Script Path : /home/pi/scripts/do.sh")
        .block(Block::bordered().title("Script Path"));
    f.render_widget(path_block, window[1]);
}
fn edit_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(25), Constraint::Percentage(80)])
        .split(tab);

    let context = Paragraph::new("Edit the tab").block(Block::bordered().title("Edit tab"));
    let other = Paragraph::new("we are locked in!").block(Block::bordered().title("Time"));
    f.render_widget(context, window[0]);

    let schedule_block = Paragraph::new("EDIT CRON").block(Block::bordered().title("Modify Schedule"));
    f.render_widget(schedule_block, window[0]);

    let path_block = Paragraph::new("EDIT Script Path : /home/pi/scripts/do.sh")
        .block(Block::bordered().title("Script Path"));
    f.render_widget(path_block, window[1]);
}
fn options_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(4),
            Constraint::Percentage(50),
            Constraint::Min(4),
        ])
        .split(tab);

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
    f.render_stateful_widget(options, window[0], &mut option_state);
    f.render_widget(context, window[1]);

    let schedule_block = Paragraph::new("EDIT CRON").block(Block::bordered().title("Modify Schedule"));
    f.render_widget(schedule_block, window[0]);

    let path_block = Paragraph::new("EDIT Script Path : /home/pi/scripts/do.sh")
        .block(Block::bordered().title("Script Path"));
    f.render_widget(path_block, window[1]);
}
fn exit_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(tab);

    let context =
        Paragraph::new("Main menu context").block(Block::bordered().title("context"));
    let other = Paragraph::new("we are locked in!").block(Block::bordered().title("Other"));
    f.render_widget(context, window[0]);
    f.render_widget(other, window[1]);

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

//Broken or unused functions //
fn exiter(f: &mut Frame, app: &mut App, window: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(window);

    let context =
        Paragraph::new("Main menu context").block(Block::bordered().title("context"));
    let other = Paragraph::new("we are locked in!").block(Block::bordered().title("Other"));

    f.render_widget(context, window[0]);
    f.render_widget(other, window[1]);

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

//parameters TBD. Since a state will be held, should it reference just app? or cron?
fn cron_maker(f: &mut Frame, app: &mut App, area: Rect) {
    let body = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Min(10),
            Constraint::Min(5),
        ])
        .split(area);
}
