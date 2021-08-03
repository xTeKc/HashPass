//import env from standard library
//import error from standard library
//import fs and io from standard library
//import sha-1 from crates_io
use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

//use Box errors for error handling
fn main() -> Result<(), Box<dyn Error>> {
    //calls method args from this module and returns iterator
    //which is collected into Vec<String>, a Vector of String objects
    let args: Vec<String> = env::args().collect();

    //return msg if length is not atleast 3 characters
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1fetch: <wordlist.txt> <sha1hash>");
        return Ok(());
    }

    let hash_to_fetch = args[2].trim();
    //check if hash to fetch is correct hash length
    if hash_to_fetch.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    //wordlist_file owns the file and has <main> func as scope
    //whenever <main> func exits, the owned file is closed
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);
    //read from the wordlist file
    for line in reader.lines() {
        let common_password = line?.trim().to_string();
        //check if hash is correct hash and print 
        if hash_to_fetch == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("{}", &common_password);
            return Ok(());
        }
    }

    println!("password not found in wordlist :(");

    Ok(())
}
