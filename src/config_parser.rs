use nom::character::complete;
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::complete;
use nom::{
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::{char, multispace0},
    combinator::{map_res, value},
    error::ParseError,
    sequence::{delimited, pair, tuple},
    IResult,
};

pub fn grab_server(input: &str) -> IResult<&str, char> {
    complete::anychar(input)
}
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn peol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(char('#'), is_not("\n\r")),
    )(i)
}
