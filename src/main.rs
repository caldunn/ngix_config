#![allow(dead_code, unused)]

extern crate core;

mod config_io;
mod config_parser;
mod environment;
mod menu_nav;
mod util;
use nginx_config_parser::lib_function;
use std::{thread, time};
fn main() {
    // menu_nav::main_menu();
    // let x = config_parser::grab_server(" abcb ");
    // let _ = config_io::parse_config();
    // playin_with_ansi_escapes();
    create_terminal();
    let x = nginx_config_parser::structures::SimpleProxyServer::new_sample();
    let nginx_string = x.as_nginx_string();
    let json_string = x.as_json();
    println!("{}", nginx_string);
    println!("{}", json_string);
    // ls();
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::json;
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

fn create_terminal() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("NginxConfigTool")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    });

    // thread::sleep(time::Duration::from_secs(2));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;
    println!("Reached to the end of the func.");

    Ok(())
}

fn my_menu() {
    let options = vec!["Option 1", "Option 2", "Option 3"];
    for (i, opt) in options.iter().enumerate() {
        println!("{}. {}", i, opt);
    }
}
fn playin_with_ansi_escapes() {
    print!("\x1b[2J \x1b[H");
    println!("");
    print!("\x1b7");
    println!("\x1b[1;31mGoing to sleep");
    thread::sleep(time::Duration::from_secs(1));
    print!("\x1b8");
    println!("\x1b[1;32mWaking up from sleep\x1b[1;0m");
}

fn ls() {
    use std::{error::Error, process::Command};

    let out = {
        Command::new("/home/caleb/dev/clangc/prog")
            .arg("--color=always")
            .output()
            .map(|s| String::from_utf8(s.stdout))
            .unwrap()
            .unwrap()
    };
    println!("Hello World!");
    println!("{}", out);
}
