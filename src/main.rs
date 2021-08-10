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
use gio::prelude::*;
use gtk::prelude::*;
use gladis::Gladis;

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

    //wordlist_file uses <RAII> (Resource Acquisition Is Initialization)
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

    //if program reaches the last line of <main> func the <main>
    //the <main> func will evaluate <Ok(())> which means "success"

    //equivalent would have been <return Ok(());> but NOT <Ok(());>
    
    //Rust expressions evaluate to a value and...
    //their opposites, statements, and instructions that do something...
    //and end with semicolon <;>
    Ok(())
}


const GLADE_SRC: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!-- Generated with glade 3.38.2 -->
<interface>
  <requires lib="gtk+" version="3.24"/>
  <object class="GtkWindow" id="window">
    <property name="can-focus">False</property>
    <property name="resizable">False</property>
    <property name="window-position">center-always</property>
    <property name="default-width">440</property>
    <property name="default-height">260</property>
    <child>
      <object class="GtkFixed" id="fixed">
        <property name="visible">True</property>
        <property name="can-focus">False</property>
        <child>
          <object class="GtkButton" id="button1">
            <property name="label" translatable="yes">Hash</property>
            <property name="width-request">100</property>
            <property name="height-request">29</property>
            <property name="visible">True</property>
            <property name="can-focus">True</property>
            <property name="receives-default">True</property>
            <signal name="clicked" handler="on_button1_clicked" object="input1" swapped="no"/>
          </object>
          <packing>
            <property name="x">308</property>
            <property name="y">133</property>
          </packing>
        </child>
        <child>
          <object class="GtkEntry" id="input1">
            <property name="width-request">270</property>
            <property name="height-request">34</property>
            <property name="visible">True</property>
            <property name="can-focus">False</property>
            <property name="placeholder-text" translatable="yes">Enter SHA1 Hash</property>
          </object>
          <packing>
            <property name="x">27</property>
            <property name="y">94</property>
          </packing>
        </child>
        <child>
          <object class="GtkTextView" id="textview1">
            <property name="width-request">270</property>
            <property name="height-request">35</property>
            <property name="visible">True</property>
            <property name="can-focus">True</property>
            <property name="editable">False</property>
            <property name="cursor-visible">False</property>
          </object>
          <packing>
            <property name="x">27</property>
            <property name="y">177</property>
          </packing>
        </child>
        <child>
          <object class="GtkLabel" id="label1">
            <property name="width-request">400</property>
            <property name="height-request">50</property>
            <property name="visible">True</property>
            <property name="can-focus">True</property>
            <property name="label" translatable="yes">SHA1Fetch</property>
            <attributes>
              <attribute name="font-desc" value="DejaVu Sans Bold 20"/>
            </attributes>
          </object>
          <packing>
            <property name="x">27</property>
            <property name="y">18</property>
          </packing>
        </child>
      </object>
    </child>
  </object>
</interface>"#;


#[derive(Gladis, Clone)]
pub struct Ui {

    pub window: gtk::Window,
    pub fixed: gtk::Fixed,
    pub button1: gtk::Button,
    pub input1: gtk::Entry,
    pub textview1: gtk::TextView,
    pub label1: gtk::Label,
}

fn build_ui(app &gtk::Application) {
    let glade_src = include_str!("sha1fetch_gui.glade");
    let ui = Ui::from_string(glade_src).unwrap();
}