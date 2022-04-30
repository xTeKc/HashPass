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
    HashedPass,
}

fn read_wordlist() -> Result<String, Error> {
    let read_list = fs::read_to_string("./wordlist.txt");
    read_list
}

fn main() {
   let read_it = read_wordlist()
    .expect("no wordlist found");
    println!("{:?}", read_it)
}