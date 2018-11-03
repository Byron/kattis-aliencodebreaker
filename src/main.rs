use std::{process, io::{self, stdin, stdout, BufRead, Write}};

fn main() -> Result<(), io::Error> {
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
            _ => unimplemented!();
        }
    }
}
