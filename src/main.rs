use std::{fs, io::Error};

use serde::{Serialize, Deserialize};

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
    #[error("error parsing the DB file: {0)")]
    ParseDBError(#[from] serde_json::Error)
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