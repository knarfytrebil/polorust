#[macro_use]
extern crate log;

// extern crate cpython;
extern crate redux;
extern crate regex;
extern crate simplelog;
extern crate termion;
extern crate tui;
extern crate uuid;

#[macro_use]
mod utils;
mod actions;
mod components;
mod middlewares;
mod reducers;
mod structs;

use simplelog::*;
use std::boxed::Box;
use std::fs::File;
use std::sync::{mpsc, Arc};
use std::{io, thread};

use termion::input::{MouseTerminal, TermRead};
use termion::screen::AlternateScreen;
use termion::raw::IntoRawMode;

use redux::Store;
// use tui::backend::MouseBackend;
use tui::backend::{TermionBackend};
use tui::Terminal;

use actions::AppAction;
use components::app;
use middlewares::{
    CommandBarMiddleWare, CommandMiddleWare, ConsoleMiddleWare, DebugMiddleWare, KeyboardMiddleWare,
};
use structs::app::events::Event;
use structs::app::{AppState, CommandHandler};

fn main() -> Result<(), io::Error> {
    // Init Logs
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    )]).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    // let (exit_tx, exit_rx) = mpsc::channel();

    let (input_tx, subscribe_tx) = (cmd_tx.clone(), tx.clone());

    // Input
    thread::spawn(move || {
        for c in io::stdin().keys() {
            input_tx
                .send(AppAction::Keyboard(c.unwrap()).into_event())
                .unwrap();
        }
    });

    // Middlewares
    let keyboard_mw = Box::new(KeyboardMiddleWare {});
    let command_bar_mw = Box::new(CommandBarMiddleWare {});
    let command_mw = Box::new(CommandMiddleWare {
        tx: cmd_tx.clone(),
        handler: CommandHandler::default(),
    });
    let console_mw = Box::new(ConsoleMiddleWare {});
    let debug_mw = Box::new(DebugMiddleWare {});
    // let exit_mw = Box::new(CommandMiddleWare {
    //     tx: exit_tx,
    //     handler: CommandHandler::default(),
    // });

    // App & State
    let store: Arc<Store<AppState>> = Arc::new(Store::new(vec![
        command_mw,
        console_mw,
        command_bar_mw,
        keyboard_mw,
        debug_mw,
    ]));

    // Create Subscription from store to render
    store.subscribe(Box::new(move |store, _| {
        let state = store.get_state();
        subscribe_tx.send(Event::Render(state)).unwrap();
    }));

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();
    terminal.hide_cursor()?;

    let size = terminal.size().unwrap();
 
    // if size != app.size && Rect::default() != app.size {
    //     size = app.size;
    //     terminal.resize(size).unwrap();
    // }

    // init state app size
    let resize_action = AppAction::ResizeApp(terminal.size().unwrap()).into_event();
    cmd_tx.send(resize_action).unwrap();

    let exit_tx = tx.clone();
    thread::spawn(move || loop {
        match cmd_rx.recv().unwrap() {
            Event::Dispatch(action) => {
                let _ = store.dispatch(action);
            }
            Event::Exit => {
                let _ = exit_tx.send(Event::Exit);
                break;
            }
            _ => {}
        }
    });

    loop {
        match rx.recv().unwrap() {
            Event::Render(app_state) => terminal.draw(|mut f| { app::render(&mut f, &app_state) }),
            Event::Exit => { break; },
            _ =>  { Ok(()) }
        };
    }

    // show cursor on end
    terminal.show_cursor()?;
    Ok(())
}
