use std::{process, io::{self, stdin, stdout, BufRead}};

#[derive(Debug)]
enum Error {
    InvalidDimensions(&'static str, String),
    InvalidCyphertext(&'static str, String),
    IO(io::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

fn parse_dimensions(input: &str) -> Result<(u64, u64), Error> {
    let mut ws = input.split_whitespace();
    match (ws.next(), ws.next()) {
        (Some(cls), Some(tss)) => {
            Ok((
                cls.parse().map_err(|_| Error::InvalidDimensions("could not parse cypher length", input.to_owned()))?,
                tss.parse().map_err(|_| Error::InvalidDimensions("coudl not parse table size", input.to_owned()))?,
            ))
        },
        _ => Err(Error::InvalidDimensions("need two whitespace separated tokens", input.to_owned())),
    }
}

fn validated_cypher_text(input: &str, cypher_len: u64) -> Result<&str, Error> {
    if input.len() < cypher_len as usize {
        return Err(Error::InvalidCyphertext("cypher text is shorter than advertised", input.to_owned()))
    }
    Ok(&input[..cypher_len as usize])
}

fn main() -> Result<(), Error> {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin_lock, mut stdout_lock) = (stdin.lock(), stdout.lock());

    let mut first_line = String::new();
    let mut second_line = String::new();
    loop {
        first_line.clear();
        second_line.clear();
        stdin_lock.read_line(&mut first_line)?;
        stdin_lock.read_line(&mut second_line)?;

        match (first_line.len(), second_line.len()) {
            (0, 0) => process::exit(0),
            (_, 0) => {
                eprintln!("input exhausted prematurely");
                process::exit(2)
            }
            _ => {
                let (cyper_len, table_size) = parse_dimensions(&first_line)?;
                let cypher_text = validated_cypher_text(&second_line, cyper_len)?;
                unimplemented!("{} {} {}", cyper_len, table_size, cypher_text)
            }
        }
    }
}
