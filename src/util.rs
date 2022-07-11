use crate::environment;
use std::io;
use std::io::Write;

/// Loop and keep asking for user input until valid string
pub fn get_user_input_string(io: &mut environment::IO, msg: String) -> String {
    // get_user_input_stdin(&msg).unwrap_or(get_user_input_string(msg))
    match gen_user_input_new(io, &msg) {
        Ok(s) => s,
        Err(e) => {
            println!("{:#?}", e); // I dont expect to hit here on normal operation.
            get_user_input_string(io, msg)
        }
    }
}
/// Loops until valid u8 -> using string buffer cos I am lazy.
pub fn get_user_input_u16(msg: String) -> u16 {
    match get_user_input_stdin(&msg) {
        Ok(raw_val) => match raw_val.trim().parse::<u16>() {
            Ok(res) => res,
            Err(_) => {
                println!("Must be a 16bit integer.");
                get_user_input_u16(msg)
            }
        },
        Err(_) => get_user_input_u16(msg),
    }
}

fn get_user_input_stdin(msg: &String) -> io::Result<String> {
    let mut s = String::new();
    print!("{}", msg);
    io::stdout().flush().expect("TODO: panic message");
    io::stdin().read_line(&mut s)?;
    Ok(s)
}

fn gen_user_input_new(io: &mut environment::IO, msg: &String) -> io::Result<String> {
    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut s = String::new();
    writeln!(&mut io.writer, "{}", msg).expect("couldnt write to output");
    input.read_line(&mut s)?;

    Ok(s)
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub fn directional_key() {}

#[cfg(test)]
mod tests {
    #[test]
    fn one_plus_onee() {
        assert_eq!(1 + 1, 2)
    }
}
