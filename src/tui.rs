// Cross term provides clearing &input handling for Windows &Unix
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, ListState},
    Terminal,
};

use crate::database;
use crate::asset;

enum Event<T> {
    Input(T),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuOpps {
    Home,
    Planets,
}

impl From<MenuOpps> for usize {
    fn from(input: MenuOpps) -> usize {
        match input {
            MenuOpps::Home => 0,
            MenuOpps::Planets => 1
        }
    }
}


pub fn start_terminal() -> Result <(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("Able to run in Raw Mode");

    // create new channel
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(200);

    // spwan new thread
    thread::spawn(move || {
        let mut last_tic = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tic.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("Error trying to poll the event") {
                if let CEvent::Key(key) = event::read().expect("Able to read events") {
                    tx.send(Event::Input(key)).expect("Able to send events");
                }
            }

            if last_tic.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tic = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Planets", "Add", "Delete", "Exit"];
    let mut active_menu_item = MenuOpps::Home;
    let mut planet_list_state = ListState::default();
    planet_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
                .split(size);

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuOpps::Home => rect.render_widget(asset::render_home(), chunks[1]),
                MenuOpps::Planets => {
                    let planet_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = asset::render_planets(&planet_list_state);
                    if let Some(x) = left {
                        rect.render_stateful_widget(x, planet_chunks[0], &mut planet_list_state);
                    }
                    if let Some(x) = right {
                        rect.render_widget(x, planet_chunks[1]);
                    }
                }
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = MenuOpps::Home,
                KeyCode::Char('p') => active_menu_item = MenuOpps::Planets,
                KeyCode::Char('a') => {
                    database::add_rand().expect("Able to add random planet");
                }
                KeyCode::Char('d') => {
                    database::remove_planet_index(&mut planet_list_state).expect("Able to remove planet");
                }
                KeyCode::Down => {
                    if let Some(selected) = planet_list_state.selected() {
                        let amount_planet = database::read_data().expect("Able to read data").len();
                        if selected >= amount_planet - 1 {
                            planet_list_state.select(Some(0));
                        } else {
                            planet_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = planet_list_state.selected() {
                        let amount_planet = database::read_data().expect("Able to read data").len();
                        if selected > 0 {
                            planet_list_state.select(Some(selected - 1));
                        } else {
                            planet_list_state.select(Some(amount_planet - 1));
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}