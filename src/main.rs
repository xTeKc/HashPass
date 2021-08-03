//import env from standard library
use std::env;

fn main() {
    //calls method args from this module and returns iterator
    //which is collected into Vec<String>, a Vector of String objects
    let args: Vec<String> = env::args().collect();
}
