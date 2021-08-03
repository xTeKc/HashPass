//import env from standard library
use std::env;

fn main() {
    //calls method args from this module and returns iterator
    //which is collected into Vec<String>, a Vector of String objects
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1fetch: <wordlist.txt> <sha1hash>");
        return;
    }
}
