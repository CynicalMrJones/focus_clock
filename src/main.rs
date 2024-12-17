
mod clock;
use clock::clock;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Constraint, prelude::{Alignment, CrosstermBackend, Direction, Layout, Style, Terminal}, style::{Modifier, Color, Styled, Stylize}, widgets::{block, Block, Borders, Paragraph, Wrap, List, ListState},
    DefaultTerminal,
};
use std::{thread, time::Duration};

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
            let picture = Paragraph::new(buf.clone())
                .white()
                .block(Block::default().style(Style::default().white())
                    .borders(Borders::ALL))
                .centered();
            let outer_border = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
                ])
                .split(frame.area());
            frame.render_widget(picture, outer_border[0]);
            let clock_string = clock(4200);
            let clock = Paragraph::new(clock_string)
                .white()
                .block(Block::default().style(Style::default().white())
                    .borders(Borders::ALL))
                .centered();
            frame.render_widget(clock, outer_border[1]);
        })?;

        if let event::Event::Key(key) = event::read()?{
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q'){
                return Ok(());
            }
        }
    }
}
