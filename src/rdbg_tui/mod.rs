use std::io;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
    layout::{Layout, Constraint, Direction},
};
use crossterm::{
    event::{self, Event, KeyCode}, 
    terminal::{enable_raw_mode, disable_raw_mode}
};



pub struct RdbgUI {
    
}


impl RdbgUI {
    pub fn new() -> Self {
        Self {}
    }


    pub fn run()
}