use crate::environment::Environment;
use crate::util::{get_user_input_string, get_user_input_u16};
use nginx_config_parser::structures::{Location, SimpleProxyServer};
use std::io;
use std::{fs, path};

pub fn parse_config() -> Result<(), io::Error> {
    println!("Parsing config.");
    let mut entries = fs::read_dir("./resources/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let as_path = entries
        .iter()
        .map(path::PathBuf::as_path)
        .collect::<Vec<&path::Path>>();

    for path in as_path {
        // parse_single_file(path)?;
    }

    parse_single_file(path::Path::new("./resources/sample.conf"))?;
    // List current files available now.
    // parse each into dedicated ds.
    Ok(())
}

fn parse_single_file(file: &path::Path) -> io::Result<()> {
    println!("\nReading file\n {:?} \nDone Reading.", &file);
    let contents = &fs::read_to_string(file)?;
    Ok(())
}

pub fn from_user_input(env: &mut Environment) -> SimpleProxyServer {
    let port = get_user_input_u16("port:\t".to_string());
    let server_name = get_user_input_string(&mut env.io, "Server Name:\t".to_string());
    SimpleProxyServer::new(port, server_name, Location::new_sample())
}
