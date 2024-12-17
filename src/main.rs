
mod clock;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line,Text},
    symbols::border,
    style::{Stylize, Modifier, Style, Color},
    widgets::{Paragraph,Block,Widget,Borders},
    DefaultTerminal,
    Frame,
};


fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal:DefaultTerminal) -> io::Result<()>{
    let file = File::open("pic.pic").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    loop{
        terminal.draw(|frame|{
            let greeting = Paragraph::new(buf.clone())
                .white()
                .block(Block::default().style(Style::default().white())
                    .borders(Borders::ALL))
                .centered();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()?{
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q'){
                return Ok(());
            }
        }
    }
}
