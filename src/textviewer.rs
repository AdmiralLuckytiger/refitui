/// Data strcuture that stores the document parsed in lines
struct Doc {
    lines: Vec<String>
}

/// Data structure that stores the curso position and to record the current size of the terminal
#[derive(Debug)] 
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

/// Main data structure representing the text viewer
pub struct TextViewer {
    doc: Doc,
    doc_lenght: usize,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
    file_name: String,
} 

impl TextViewer {
    /// Instantiate TextViewer and initializate
    pub fn init(file: &str) -> Self {
        todo!()
    }

    /// Displays the contents of the file on the terminal screen
    pub fn show_document(&mut self) {
        todo!()
    }

    /// Waits for user inputs to the process.
    /// If the user presses Ctrl + Q, the program exits. 
    pub fn run(&mut self) {
        todo!()
    }
}