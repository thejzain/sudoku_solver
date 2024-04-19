use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

mod tui;

pub struct App {
    table: u8,
    exit: bool,
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::defualt().run(&mut terminal);
    tui::restore()?;
    app_result
}
