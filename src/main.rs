extern crate log;
extern crate stderrlog;
extern crate termion; 
extern crate tui;

mod store;
mod components;

use std::io;
use std::io::{Write};
use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget, Paragraph, Tabs};
use tui::layout::{Direction, Group, Rect, Size};

use store::loops::App;
use components::status_bar;
use components::command_bar;

enum Event {
    Input(event::Key),
}

fn main() {
    stderrlog::new().verbosity(4).init().unwrap();
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    // Input
    thread::spawn(move || {
        let mut input_cmd = String::new();
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });
    // App
    let mut app = App::new();
    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    render_app(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Char(':') => {
                    break;
                }
               _ => {}
            },
        }
        render_app(&mut terminal, &app);
    }
    terminal.show_cursor().unwrap();
}

fn render_app(t: &mut Terminal<MouseBackend>, app: &App) -> Result<(), io::Error> {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(1), Size::Fixed(1), Size::Fixed(1)])
        .render(t, &app.size, |t, chunks| {
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
            match app.tabs.selection {
                0 => { render_text(t, app, &chunks[1]) }
                1 => { }
                _ => { }
            }
            status_bar::status_bar::render(t, app, &chunks[2]);
            command_bar::command_bar::render(t, app, &chunks[3]);
        });
    try!(t.draw());
    Ok(())
}

fn render_text(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
     Paragraph::default()
        .block(Block::default().title("Text"))
        .wrap(true)
        .text("text")
        .render(t, area);
}
