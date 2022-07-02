use std::io;
use std::{fs, path};
// mod config_parser;

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

fn sample_entry() -> SimpleProxyServer {
    SimpleProxyServer {
        port: 0000,
        server_name: "kek.localhost".to_string(),
        location: Location {
            root: "/".to_string(),
            proxy_pass: "/".to_string(),
            proxy_set_header: vec!["Host".to_string(), "$host".to_string()],
        },
    }
}

#[derive(Debug)]
struct SimpleProxyServer {
    port: u8,
    server_name: String,
    location: Location,
}

#[derive(Debug)]
struct Location {
    root: String,
    proxy_pass: String,
    proxy_set_header: Vec<String>,
}
