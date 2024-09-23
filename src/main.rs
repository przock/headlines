use std::{collections::BTreeMap, io};
use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, List, ListDirection, ListItem, ListState}, Terminal,
    widgets::{ Paragraph, Wrap },
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyEventKind, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::Value;
mod grab_news;
mod config_reader;

fn main() -> Result<(), io::Error> {

    let mut config: Option<config_reader::Config> = None;
    let mut sources: Option<String> = None;
    let mut ratio: Option<u16> = None;

    match config_reader::read_config() {
	Ok(c) => {
	    config = Some(c);
	}
	Err(_) => {}
    }

    if let Some(c) = config {
	sources = c.api.sources;
	ratio = c.tui.ratio;
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let articles_map: BTreeMap<String, Value> = grab_news::news_get_as_json(sources.unwrap_or(String::from("bbc-news"))).expect("Unable to fetch news");
    let mut list_items = vec![];

    for (_k, v) in &articles_map {
	let title = v.get("title").and_then(Value::as_str).unwrap();
	let description = v.get("description").and_then(Value::as_str).unwrap_or("No description");
	let published_at = v.get("published_at").and_then(Value::as_str).unwrap_or("No date");
	let source_name = v.get("source_name").and_then(Value::as_str).unwrap_or("No source");
	let content = v.get("content").and_then(Value::as_str).unwrap_or("No content");
	let author = v.get("author").and_then(Value::as_str).unwrap_or("no author.");
	let url = v.get("url").and_then(Value::as_str).unwrap_or("No URL");

	list_items.push((ListItem::new(title), format!("Author: {}\n\nDesc: {}\n\nDate: {}\n\nSource: {}\n\nContent: {}\n\nURL: {}", author, description, published_at, source_name, content, url)));
    }

    let items_left: Vec<ListItem> = list_items.iter().map(|(item, _)| item.clone()).collect();

    let options = List::new(items_left)
	.block(Block::bordered().title("List"))
	.style(Style::default().fg(Color::White))
	.highlight_symbol("|")
	.repeat_highlight_symbol(true)
	.direction(ListDirection::TopToBottom);

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(ratio.unwrap_or(45) as u16),
                        Constraint::Percentage(100 - ratio.unwrap_or(45) as u16),
                    ].as_ref()
		).split(f.area());

	    let selected = state.selected().unwrap_or(0);
	    let detail_text = &list_items[selected].1;
	    
	    let details = Paragraph::new(detail_text.clone())
		.block(Block::default().borders(ratatui::widgets::Borders::ALL).title("More"))
		.style(Style::default().fg(Color::White))
		.wrap(Wrap { trim: true } );
    
	    f.render_stateful_widget(&options, chunks[0], &mut state);
	    f.render_widget(details, chunks[1]);
        })?;

	if let Event::Key(key_event) = event::read()? {
	    if key_event.code == KeyCode::Char('q')  {
		break;
	    } else {
		handle_key(key_event, &mut state, options.len());
	    }
	};
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_key(key: KeyEvent, list: &mut ListState, len: usize) {
    if key.kind != KeyEventKind::Press {
	return;
    }
    match key.code {
	KeyCode::Char('k') | KeyCode::Up => {
	    let i = match list.selected() {
		Some(i) => {
		    if i == 0 {
			len - 1
		    } else {
			i - 1
		    }
		}
		None => 0,
	    };
	    list.select(Some(i));
        },
	KeyCode::Char('j') | KeyCode::Down => {
	    let i = match list.selected() {
		Some(i) => {
		    if i == len - 1 {
			0
		    } else {
			i + 1
		    }
		}
		None => 0,
	    };
	    list.select(Some(i));
        }
	_ => {}
    }
}
