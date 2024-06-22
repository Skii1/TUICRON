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
use crate::cron::CronTask;

pub fn render_ui(f: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), //Tab Toolbar
            Constraint::Min(5), //TEMPORARY ?todo REMOVE once inner blocks are properly implemented WHY : something is targeting the layout. causing it to break
            Constraint::Length(3), //Footer, key bind tips
        ])
        .split(f.size());

    //TOOLBAR
    render_toolbar(f, app, layout[0]);

    //TITLE, Main tab frame along with the tab contents within,
    let tab_frame = tab_frame(f, app, layout[1]);

    //TAB BODY
    render_tab(f, app, tab_frame);

    //FOOTER
    render_footer(f, app, layout[2]);
    //Here, the middle segment of the screen is organized differently depending on the selected tab, text, and widget content is specified later
}

//Helper Functions //
//Menu Styling won't work??? Why???

///new_tab_frame In development function to make outer box for the entire tab frame, use render title for now, called tab render
fn tab_frame(f: &mut Frame, app: &mut App, frame: Rect) -> Rect {
    let menu_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::White)
        .bg(Color::Rgb(0, 0, 55));

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
        .highlight_style(Style::default().yellow().bg(Color::Rgb(50, 50, 50)))
        .select(app.option)
        .padding(" ", " ")
        .divider("|");

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
    let footer_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::new().on_light_yellow().bg(Color::Rgb(0, 0, 155)));
    let footer = Paragraph::new(key_tips)
        .style(Color::White)
        .alignment(Alignment::Center)
        .block(footer_block)
        .wrap(Wrap { trim: true });

    f.render_widget(footer, area);
}

fn render_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    match app.selected_tab {
        CurrentTab::Menu => menu_tab(f, app, tab),
        CurrentTab::Edit => edit_tab(f, app, tab),
        CurrentTab::New => new_tab(f, app, tab),
        CurrentTab::Options => options_tab(f, app, tab),
        CurrentTab::Exit => exit_tab(f, app, tab),
    }
}

//  Tab Layouts //
fn menu_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let window = Layout::default()
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
        .constraints([
            Constraint::Min(15),
            Constraint::Min(25),
            Constraint::Min(30),
        ])
        .split(tab);

    let fields = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(window[1]);

    let time = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ])
        .split(fields[2]);

    //Context Block init
    let mut context = Paragraph::new("")
        .block(Block::bordered().title("Context"))
        .wrap(Wrap { trim: true });
    
    //Minute Field
    let minute_text = if app.minute_buffer.is_empty() {
        String::from("Enter Minute : ")
    } else {
        app.minute_buffer.clone()
    };
    let mut minute_block = Paragraph::new(minute_text).block(Block::bordered().title("Minute"));

    //Hour Field
    let hour_text = if app.minute_buffer.is_empty() {
        String::from("Enter Hour : ")
    } else {
        app.hour_buffer.clone()
    };
    let mut hour_block = Paragraph::new(hour_text).block(Block::bordered().title("Hour"));

    //Periodic Field
    let periodic_text = if app.periodic_buffer{
        String::from("Periodic")
    } else {
        String::from("Once")
    };
    let mut periodic_block = Paragraph::new(periodic_text).block(Block::bordered().title("Hour"));
    
    //Command Field
    let command_text = if app.minute_buffer.is_empty() {
        String::from("Enter Minute : ")
    } else {
        app.command_buffer.clone()
    };
    let mut command_block = Paragraph::new(command_text)
        .block(Block::bordered().title("Command"))
        .wrap(Wrap { trim: true });

    let weekday_text = if app.minute_buffer.is_empty() {
        String::from("Enter a value, in a range, a-b, where a and b are day values, 0 indexed from Sunday. (e.g : 1-5 is running Monday to Friday")
    } else {
        app.weekday_buffer.clone()
    };

    let mut weekday_block = Paragraph::new(weekday_text)
        .block(Block::bordered().title("Weekday(s)"))
        .wrap(Wrap { trim: true });

    let mut preview = Paragraph::new(app.task_format())
        .block(Block::bordered().title("Preview Task"))
        .wrap(Wrap { trim: true });

    let confirm_text = String::from("Make Task (Enter)");
    let mut confirm_block = Paragraph::new(confirm_text)
        .block(Block::bordered().title("Confirm"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    
    match app.input_mode {
        InputState::Idle => {
            context = Paragraph::new("Create a new task with (N). Find Tooltips on each field in this box, with information on how to select an option, and what the option does.")
                .block(Block::bordered().title("Context"))
                .wrap(Wrap { trim: true });
        }

        InputState::Minute => {
            context = Paragraph::new("Enter a value in minutes (0-59).")
                .block(Block::bordered().title("Context"))
                .wrap(Wrap { trim: true });

            let input = Text::from(vec![Line::from(vec![
                "".into(),
                app.minute_buffer.clone().red(),
            ])]);
            let min_txt = Paragraph::new(input)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title("Edit Minute"));

            minute_block = min_txt;
        }

        InputState::Hour => {
            context = Paragraph::new("Enter a value in hours (0-23).")
                .block(Block::bordered().title("Context"))
                .wrap(Wrap { trim: true });

            let input = Text::from(vec![Line::from(vec![
                "".into(),
                app.hour_buffer.clone().red(),
            ])]);
            let hour_txt = Paragraph::new(input)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title("Edit Hour"));

            hour_block = hour_txt;
        }

        InputState::Periodic => {
            context = Paragraph::new("Cycle through the period type with (P). \"Once\" indicates a task will run on the time specified, while \"Periodic\" indicates a task running every specified time. (e.g, 5:30 Once will run at 5:30 every specified day, while Periodic will run every 5 hours and 30 minutes during the specified days ")
                .block(Block::bordered().title("Context"))
                .wrap(Wrap { trim: true });

            let input = Text::from(vec![Line::from(vec![
                "".into(),
                app.periodic_text.clone().green(),
            ])]);
            let periodic_txt = Paragraph::new(input)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title("Period"));

            periodic_block = periodic_txt;
        }
        
        InputState::Script => {
            context = Paragraph::new("Enter a command, or script path to be run here.")
                .block(Block::bordered().title("Context"))
                .wrap(Wrap { trim: true });

            let input = Text::from(vec![Line::from(vec![
                "Command | ".into(),
                app.command_buffer.clone().red(),
            ])]);

            let command_input = Paragraph::new(input)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title("Edit Command"));

            command_block = command_input;
        }
        InputState::Weekday => {
            context = Paragraph::new("Enter a day, or a range of days of the week for the task to be run. (Values are indexed 0-7, where Sunday is either 0 or 7. Monday is 1. Indicate a range of days by separating with a dash. (e.g, 1-5 is Monday to Friday")
                .block(Block::bordered().title("Context"))
                .wrap(Wrap { trim: true });

            let input = Text::from(vec![Line::from(vec![
                "d-d | ".into(),
                app.weekday_buffer.clone().red(),
            ])]);

            let weekday_input = Paragraph::new(input)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title("Edit Weekday(s)"));

            weekday_block = weekday_input;
        }
        InputState::Confirm => {
            context = Paragraph::new("Ready to add this task? Press (Enter) to submit it, or (Backspace) to edit your options")
                .block(Block::bordered().title("Context"))
                .wrap(Wrap { trim: true });
            let confirm_button = Paragraph::new("Add Task (Enter)")
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center)
                .block(Block::bordered().title("Confirm"));
            
            confirm_block = confirm_button;
        }
        _ => {}
    };

    //render all tabs
    f.render_widget(context, window[0]);
    f.render_widget(preview, fields[0]);
    preview_task(app, f, fields[1]);
    f.render_widget(minute_block, time[0]);
    f.render_widget(hour_block, time[1]);
    f.render_widget(periodic_block, time[2]);
    f.render_widget(weekday_block, fields[3]);
    f.render_widget(command_block, fields[4]);
    f.render_widget(confirm_block, fields[5]);
    task_list(app, f, window[2]);
}

fn edit_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let mut window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Percentage(80)])
        .split(tab);

    let context = Paragraph::new("View CRON tasks in different modes here. You can either present a list in a table form, which is more readable, or in raw form, as you would put them into the crontab file itself. Editing Tasks coming soon!")
        .block(Block::bordered().title("Context"));
    let other = Paragraph::new("View CRON tasks").block(Block::bordered().title("Title"));
    f.render_widget(context, window[0]);

    let path_block = Paragraph::new("Cron Table Here").block(Block::bordered().title("Cron Tasks"));
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

    let items = ["Controls", "Preferences", "Credits"];
    let options = List::new(items)
        .block(Block::bordered().title("Options Menu"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    let mut option_state = ListState::default();
    option_state.select(Some(0));

    let context = Paragraph::new("This will be implemented at a future date!")
        .block(Block::bordered().title("Context"));
    f.render_stateful_widget(options, window[0], &mut option_state);
    f.render_widget(context, window[1]);

    let schedule_block = Paragraph::new("Coming Soon").block(Block::bordered().title("Window 1"));
    f.render_widget(schedule_block, window[0]);

    let path_block = Paragraph::new("Coming Soon").block(Block::bordered().title("Window 2"));
    f.render_widget(path_block, window[1]);
}

fn exit_tab(f: &mut Frame, app: &mut App, tab: Rect) {
    let window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Fill(1),
            Constraint::Percentage(20),
        ])
        .split(tab);

    let exit_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Fill(1),
            Constraint::Percentage(20),
        ])
        .split(window[1]);

    let other =
        Paragraph::new("Would you like to exit? Ensure all changes are saved before closing.")
            .block(Block::bordered().title("Exit? (Y/N)"))
            .style(Style::new().bg(Color::Rgb(0, 0, 55)));
    f.render_widget(other, exit_window[1]);
}

pub fn preview_task(app: &mut App, f: &mut Frame, area: Rect) {
    let task = Paragraph::new(format!(
        "CRON : {} at {}:{} on day(s) {}",
        app.command_buffer, app.hour_buffer, app.minute_buffer, app.weekday_buffer
    ))
    .block(Block::bordered());
    f.render_widget(task, area);
}

pub fn task_list(app: &mut App, f: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .tasks
        .clone()
        .into_iter()
        .map(|item| {
            ListItem::new(format!(
                "{} | {}:{} | {} | {}",
                item.periodic.to_owned(),
                item.hour.to_owned(),
                item.minute.to_owned(),
                item.weekday.to_owned(),
                item.command.to_owned()
            ))
        })
        .collect();

    let list_chunks = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    let list = List::new(items)
        .block(Block::bordered().title("CRONlist"))
        .highlight_symbol("->")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(list, area);
}
