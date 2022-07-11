use nom::{
    bytes::complete::is_not,
    character::complete::{anychar, char, multispace0},
    combinator::value,
    error::ParseError,
    sequence::{delimited, pair},
    IResult,
};

pub fn grab_server(input: &str) -> IResult<&str, char> {
    anychar(input)
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
