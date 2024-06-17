use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::*, 
    Frame,
};
use std::rc::Rc;

use crate::app::{App, CurrentTab, InputState};
pub fn render_ui(f: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), //Tab Toolbar
            Constraint::Min(5), //TEMPORARY ?todo REMOVE once inner blocks are properly implemented WHY : something is targeting the layout. causing it to break
            Constraint::Length(1), //Main Body, main block as Tab title and contents
            Constraint::Length(1), //Footer, key bind tips
        ])
        .split(f.size());
    
    //TOOLBAR
    render_toolbar(f, app, layout[0]);

    //TITLE, Main tab frame along with the tab contents within, 
    let tab_frame = tab_frame(f, app, layout[1]);
    
    //TAB BODY
    render_tab(f, app, tab_frame);

    //FOOTER
    render_footer(f, app, layout[3]);
    //Here, the middle segment of the screen is organized differently depending on the selected tab, text, and widget content is specified later
}

//Helper Functions //
//Menu Styling won't work??? Why???

///new_tab_frame In development function to make outer box for the entire tab frame, use render title for now, called tab render
fn tab_frame(f: &mut Frame, app: &mut App, frame: Rect) -> Rect{
        let menu_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::White)
            .bg(Color::Rgb(0,0,55));
    
        let menu_text = match app.selected_tab {
            CurrentTab::Menu => Line::styled("Menu", menu_style),
            CurrentTab::New => Line::styled("NEW | Cron Task", menu_style),
            CurrentTab::Edit => Line::styled("EDIT | Cron Task", menu_style),
            CurrentTab::Options => Line::styled("Options", menu_style),
            CurrentTab::Exit => Line::styled("Exit", menu_style),
        };
        let title_block = Block::bordered()
            .title(menu_text)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Double)
            .padding(Padding::new(1, 1, 1, 1));
        let tab_frame = title_block.inner(frame);
        f.render_widget(title_block, frame);
    tab_frame
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
            CurrentTab::Menu => "Navigate Tabs (↹ Tab) | Navigate Menu ( ↑ ↓ ← →) | Select (Enter) |",
            CurrentTab::New => "Navigate Tabs (↹ Tab) | Enter Data (↳ Enter)| Navigate Fields ( ↑ ↓ ← →) | Cancel (Esc) | Confirm New Job (C) | Create New Job (N) |",
            CurrentTab::Edit => "Navigate Tabs (↹ Tab) | Navigate Fields ( ↑ ↓ ← →) | Cancel (Esc) | Confirm Edit (C) |",
            CurrentTab::Options => "Navigate Tabs (↹ Tab) | Navigate Fields ( ↑ ↓ ← →) | Select Job (Enter) | Cancel (Esc) | Confirm Edit (C) |",
            CurrentTab::Exit => "Navigate Tabs (↹ Tab) | Exit (Y) |",
        };
    let footer = Paragraph::new(key_tips)
        .style(Style::new().white().bg(Color::Rgb(0, 0, 55)))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(footer, area)
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

//  Tab Layouts //
fn menu_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(tab);

    let context = Paragraph::new("Find tooltips and context for selected elements in this block")
        .block(Block::bordered().title("Context"))
        .wrap(Wrap { trim: true });
    let other = Paragraph::new("Welcome to \"TUICRON\", a text based interface for CRON. You can view, edit, and add new cron tasks through their respective menus. Detailed descriptions Will appear in their respective context menus.")
        .block(Block::bordered().title("Main Menu"))
        .wrap(Wrap { trim: true });

    f.render_widget(context, window[0]);
    f.render_widget(other, window[1]);
}
fn new_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(15), Constraint::Percentage(85)])
        .split(tab);

    let fields = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            ])
        .split(window[1]);

    let context = Paragraph::new("Find tooltips and context for selected elements in this block")
        .block(Block::bordered().title("Context"))
        .wrap(Wrap { trim: true });
    f.render_widget(context, window[0]);

    //content of cronmaker
    let sometitle = Paragraph::new("TITLE")
        .block(Block::bordered().title("TITLE"))
        .wrap(Wrap { trim: true });
    let time_input = Paragraph::new(app.time_input.as_str())
        .style(match app.input_mode {
            InputState::Time => Style::default().fg(Color::Yellow),
            _ => {Style::default()},
        })
        .block(Block::bordered().title("Time"));
    let weekdays = Paragraph::new("Weekdays")
        .block(Block::bordered().title("WEEK"))
        .wrap(Wrap { trim: true });
    let dayofmonth = Paragraph::new("calendar day of month")
        .block(Block::bordered().title("DOM"))
        .wrap(Wrap { trim: true });
    let preview = Paragraph::new("other idk")
        .block(Block::bordered().title("Context"))
        .wrap(Wrap { trim: true });

    f.set_cursor(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        fields[1].x + app.character_index as u16 + 1,
        // Move one line down, from the border to the input line
        fields[1].y + 1,
    );

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{i}: {m}")));
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(Block::bordered().title("Messages"));
    
    //render all tabs
    f.render_widget(sometitle, fields[0]);
    f.render_widget(time_input, fields[1]);
    f.render_widget(weekdays, fields[2]);
    f.render_widget(dayofmonth, fields[3]);
    f.render_widget(messages, fields[4]);

  
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

    let area = centered_rect(20, 25, f.size());
    f.render_widget(exit_paragraph, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r` probably useless
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((80 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((80 - percent_y) / 2),
        ])
        .split(r);
    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((60 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((60 - percent_x) / 2),
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
