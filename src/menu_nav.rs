use crate::config_io;
use crate::environment::Environment;
use nginx_config_parser::structures::SimpleProxyServer;

pub fn main_menu() {
    println!("Main Menu");
    let mut env = Environment::live();
    //let proxy_server = config_io::from_user_input(&mut env);
    let proxy_server = SimpleProxyServer::new_sample();
    println!("{}", proxy_server.as_nginx_string())
}
