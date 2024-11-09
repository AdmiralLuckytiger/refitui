mod textviewer;

use std::env::args;
use std::fs;
use std::io::{stdin, stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    color,
    style,
};


fn main() {
    // Get arguments from command line
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Please provide a file name as argument");
        std::process::exit(0);
    }

    // Check if file exists. If not, print error
    // message and exit process
    if !std::path::Path::new(&args[1]).exists() {
        eprintln!("File does not exists");
        std::process::exit(0);
    } 

    // Open file and load into struct
    println!("{}", termion::cursor::Show);

    // Iniatialize viewer 
    let mut viewer = textviewer::TextViewer::init(&args[1]);
    viewer.show_document();
    viewer.run();
}
