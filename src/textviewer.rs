use std::fs;

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

        // Initialize the buffer that is used to store the file contents
        let mut doc_file = Doc {
            lines: vec![]
        };

        // Read the file contents as a string
        let file_handle = fs::read_to_string(file).unwrap();

        // Read each line from the file and store it in ht Doc buffer
        for doc_line in file_handle.lines() {
            doc_file.lines.push(doc_line.to_string());
        }

        // Initailize the doc_length variable with the number of lines of the file
        let doc_lenght = file_handle.lines().count();

        // Use the termion crate to get the terminal size
        let size = termion::terminal_size().unwrap();

        // Create a new struct of the TextViewer type and return it from the init() method
        Self {
            doc: doc_file,
            cur_pos: Coordinates {
                x: 1,
                y: doc_lenght,
            },
            doc_lenght,
            terminal_size: Coordinates {
                x: size.0 as usize,
                y: size.1 as usize
            },
            file_name: file.into(),
        }
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