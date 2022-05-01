use std::fs;
use std::io;
use std::thread;
use std::sync::mpsc;
use thiserror::Error;
use chrono::prelude::*;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, prelude::*};

use crossterm::{event::{self, Event as CEvent, KeyCode},
terminal::{disable_raw_mode, enable_raw_mode},};

use tui::{backend::CrosstermBackend,
layout::{Alignment, Constraint, Direction, Layout},
style::{Color, Modifier, Style},
text::{Span, Spans},
widgets::{Block, BorderType, Borders, Cell, List, ListItem, 
ListState, Paragraph, Row, Table, Tabs,},Terminal,};

const DATABASE: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
struct Data {
    id: u32,
    password: String,
    hash: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error)
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Clone, Copy, Debug)]
enum TabItem {
    Main,
    HashedPasses,
}

impl From<TabItem> for usize {
    fn from(input: TabItem) -> usize {
        match input {
            TabItem::Main => 0,
            TabItem::HashedPasses => 1,
        }
    }
}

// TODO from main

// fn read_wordlist() {
//     let read_list = fs::read_to_string("./wordlist.txt");
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });



}