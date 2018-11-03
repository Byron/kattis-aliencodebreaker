use std::{process, io::{stdin, stdout, BufRead, BufWriter, Write}};

mod parse {
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        InvalidDimensions(&'static str, String),
        InvalidCyphertext(&'static str, String),
        IO(io::Error),
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::IO(err)
        }
    }

    pub fn dimensions(input: &str) -> Result<(u32, u32), Error> {
        let mut ws = input.split_whitespace();
        match (ws.next(), ws.next()) {
            (Some(cls), Some(tss)) => Ok((
                cls.parse().map_err(|_| {
                    Error::InvalidDimensions("could not parse cypher length", input.to_owned())
                })?,
                tss.parse().map_err(|_| {
                    Error::InvalidDimensions("coudl not parse table size", input.to_owned())
                })?,
            )),
            _ => Err(Error::InvalidDimensions(
                "need two whitespace separated tokens",
                input.to_owned(),
            )),
        }
    }

    pub fn validated_cypher_text(input: &str, cypher_len: u32) -> Result<&str, Error> {
        if input.len() < cypher_len as usize {
            return Err(Error::InvalidCyphertext(
                "cypher text is shorter than advertised",
                input.to_owned(),
            ));
        }
        Ok(&input[..cypher_len as usize])
    }
}

mod crypt {
    use std::io::Write;

    type UInt = usize;

    const MOD: UInt = 1048576;
    const BASE: u8 = 27;

    fn f(x: UInt) -> UInt {
        (x * 33 + 1) % MOD
    }

    pub fn make_pad(table_size: u32, cypher_len: u32) -> Vec<u8> {
        let cols = {
            let mut cols: Vec<UInt> = vec![0; table_size as usize];
            let mut v: UInt = 0;
            for _ in 0..table_size as usize {
                for x in 0..table_size as usize {
                    v = f(v);
                    let xv = unsafe { cols.get_unchecked_mut(x) };
                    *xv = (*xv + v) % MOD;
                }
            }
            cols
        };

        let npad = {
            let mut pad = Vec::<u8>::new();
            for v in cols.iter().rev() {}
            pad
        };

        // KSJKJZOCWUUAWDBXG
        let pad = vec![10, 18, 9, 10, 9, 25, 14, 2, 22, 20, 20, 0, 22, 3, 1, 23, 6];
        eprintln!("{:?}", pad);
        eprintln!("{:?}", npad);
        pad[..cypher_len as usize].to_owned()
    }

    fn ascii_to_code(c: char) -> u8 {
        match c {
            'A'..='Z' => c as u8 - 'A' as u8,
            ' ' => 26,
            _ => panic!("Invalid input - must be of class [A-Z ]"),
        }
    }

    fn base27_to_ascii(c: u8) -> u8 {
        match c {
            0..=25 => (c + 'A' as u8),
            26 => ' ' as u8,
            _ => panic!("Invalid codepoint - must be 0 to 26, inclusive"),
        }
    }

    pub fn decode(encoded: &str, pad: &[u8], out: &mut Write) {
        assert_eq!(
            encoded.as_bytes().len(),
            pad.len(),
            "need pad len to be enocded bytes length, which must be ascii"
        );

        for (c, p) in encoded.chars().map(ascii_to_code).zip(pad) {
            let base27 = (c + p) % BASE;
            out.write(&[base27_to_ascii(base27)]).unwrap();
        }
        out.write(&['\n' as u8]).unwrap();
    }
}

fn main() -> Result<(), parse::Error> {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin_lock, stdout_lock) = (stdin.lock(), stdout.lock());
    let mut writer = BufWriter::new(stdout_lock);

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
                let (cypher_len, table_size) = parse::dimensions(&first_line)?;
                let cypher_text = parse::validated_cypher_text(&second_line, cypher_len)?;
                let pad = crypt::make_pad(table_size, cypher_len);
                crypt::decode(&cypher_text, &pad, &mut writer);
                writer.flush().unwrap();
            }
        }
    }
}
