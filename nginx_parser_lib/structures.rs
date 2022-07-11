use crate::serialiser::MySerialiser;

use crate::serialiser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleProxyServer {
    #[serde(rename = "listen")]
    port: u16,
    server_name: String,

    location: Option<Location>,
}

impl SimpleProxyServer {
    pub fn new(port: u16, server_name: String, location: Location) -> SimpleProxyServer {
        SimpleProxyServer {
            port,
            server_name,
            location: Some(location),
        }
    }

    pub fn from_config_string() -> SimpleProxyServer {
        SimpleProxyServer::new_sample()
    }

    pub fn as_nginx_string(&self) -> String {
        format!(
            "server {{\n\
          \x20\x20listen {};\n\
          \x20\x20server_name {};\n\
        \n\
          \x20\x20location {} {{\n\
              \x20\x20\x20\x20proxy_pass {};\n\
        \x20\x20}}\n\
        }}",
            self.port,
            self.server_name,
            self.location.as_ref().unwrap().root,
            self.location.as_ref().unwrap().proxy_pass
        )
    }

    pub fn as_json(&self) -> String {
        let x = serialiser::to_string(&self);
        serde_json::to_string_pretty(&self).unwrap();
        x.unwrap()
    }

    pub fn new_sample() -> SimpleProxyServer {
        SimpleProxyServer {
            port: 80,
            server_name: "kek.localhost".to_string(),
            location: Some(Location::new_sample()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    root: String,
    proxy_pass: String,
    proxy_set_header: Vec<String>,
}

impl Location {
    pub fn new_sample() -> Location {
        Location {
            root: "/".to_string(),
            proxy_pass: "localhost:8080".to_string(),
            proxy_set_header: vec!["Host".to_string(), "$host".to_string()],
        }
    }
}
