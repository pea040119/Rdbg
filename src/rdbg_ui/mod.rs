use std::io::{
    self, 
    Stdout
};
use tui::{
    Frame,
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
    layout::{Layout, Constraint, Direction, Rect},
};
use crossterm::{
    event::{self, Event, KeyCode}, 
    terminal::{enable_raw_mode, disable_raw_mode}
};

use crate::rdbg_utils::error::DbgError;



pub struct RdbgUI {
    ui: Terminal <CrosstermBackend <io::Stdout>>,
    top_percentage: u16,
    left_percentage: u16,
    terminal_block: Paragraph<'_>,
    register_block: Paragraph<'_>,
    memory_block: Paragraph<>,
}


impl RdbgUI {
    pub fn new(top_percentage: u16, left_percentage: u16) -> Result<RdbgUI, DbgError> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let ui = Terminal::new(backend).unwrap();
        let chunks = Vec::new();
        let terminal_block = Paragraph::new("Terminal").block(Block::default().title("Terminal").borders(Borders::ALL));
        let register_block = Paragraph::new("Registers").block(Block::default().title("Registers").borders(Borders::ALL));
        let memory_block = Paragraph::new("Memory").block(Block::default().title("Memory").borders(Borders::ALL));

        return Ok(RdbgUI {
            ui: ui,
            top_percentage: top_percentage,
            left_percentage: left_percentage,
            terminal_block: terminal_block,
            register_block: register_block,
            memory_block: memory_block,
        })
    }


    pub fn set_chunk(&mut self, top_percentage: u16, left_percentage: u16) -> Result<_, DbgError> {
        if (top_percentage>100) || (left_percentage>100) {
            return Err(DbgError::new(&format!("Invalid percentage value: top_percentage: {}, left_percentage: {}", top_percentage, left_percentage)));
        }
        self.top_percentage = top_percentage;
        self.left_percentage = left_percentage;
        return Ok(());
    }


    pub fn draw_ui(&mut self) {
        self.ui.draw(|f| {
            let chunks = self.draw_chunk(f);
        })?;
    }


    fn draw_chunk(&self, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<Vec<Rect>, DbgError>{
        let chunks: Vec<Rect> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(self.top_percentage), Constraint::Percentage(100-self.top_percentage)].as_ref())
            .split(f.size())
            .iter()
            .flat_map(|chunk| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(self.left_percentage), Constraint::Percentage(100-self.left_percentage)].as_ref())
                    .split(*chunk)
            })
            .collect::<Vec<_>>();

        return Ok(chunks);
    }


    fn draw_terminal(&self, f: &mut Frame<CrosstermBackend<Stdout>>, terminal_block: Vec<Rect>) {
        terminal_block = Paragraph::new("Terminal").block(Block::default().title("Terminal").borders(Borders::ALL));
    }
}