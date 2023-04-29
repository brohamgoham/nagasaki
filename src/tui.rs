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

enum Event<T, U> {
    Input(T),
    Tick(U),
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


pub fn start_terminal() {}