use std::fs::File;
use std::io::{self, BufRead, Read};

pub struct Input<'a> {
    source: Box<dyn BufRead + 'a>,
}

impl<'a> Input<'a> {
    pub fn from_stdin(stdin: &'a io::Stdin) -> Input<'a> {
        Input {
            source: Box::new(stdin.lock()),
        }
    }

    pub fn from_file(path: &str) -> io::Result<Input<'a>> {
        File::open(path).map(|file| Input {
            source: Box::new(io::BufReader::new(file)),
        })
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
}
