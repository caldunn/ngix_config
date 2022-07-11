use std::io;
use std::io::{Read, Write};

pub struct Environment {
    pub io: IO,
}

impl Environment {
    pub fn live() -> Environment {
        Environment {
            io: IO {
                reader: Box::new(io::stdin()),
                writer: Box::new(io::stdout()),
            },
        }
    }
    pub fn on_out_error(self) -> &'static str {
        "Failed to write to output"
    }
    pub fn on_in_error(self) -> &'static str {
        "Failed to read input"
    }

    pub fn test() -> Environment {
        Environment {
            io: IO {
                reader: Box::new(io::stdin()),
                writer: Box::new(io::stdout()),
            },
        }
    }
}

pub struct IO {
    pub reader: Box<dyn Read>,
    pub writer: Box<dyn Write>,
}
