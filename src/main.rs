use crate::config_io::parse_config;

mod config_io;
mod config_parser;
mod menu_nav;

fn main() {
    println!("Hello, world!");
    menu_nav::main_menu();
    let x = config_parser::grab_server(" abcb ");
    println!("{:#?}", x.unwrap());
    config_io::parse_config();
}
