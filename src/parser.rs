use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn error<T>(message: &str) -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::Other, message))
}

pub fn parse_lines<'a, T>(input: &str) -> io::Result<FileInput<FileLines, T>> {
    FileInput::new(FileLines::new(input)?)
}

pub fn parse_bytes<'a, T>(input: &str, split: &u8) -> io::Result<FileInput<FileSplit, T>> {
    FileInput::new(FileSplit::new(input, split)?)
}

pub fn parse_from<'a, T, U>(source: &'a mut T) -> Option<U>
where
    U: TryFrom<&'a mut T, Error = io::Error>,
{
    match U::try_from(source) {
        Ok(item) => Some(item),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => {
            eprintln!("Error! {:}", e);
            None
        }
    }
}

pub struct FileInput<T, U> {
    pub source: T,
    _dud: Option<U>,
}

impl<T, U> FileInput<T, U> {
    fn new(source: T) -> io::Result<Self> {
        Ok(Self {
            source: source,
            _dud: None,
        })
    }
}

pub struct FileLines {
    lines: io::Lines<io::BufReader<File>>,
}

impl FileLines {
    pub fn new(filename: &str) -> io::Result<Self> {
        let f = File::open(filename)?;
        Ok(FileLines {
            lines: io::BufReader::new(f).lines(),
        })
    }

    pub fn next_result(&mut self) -> io::Result<String> {
        match self.lines.next() {
            Some(Ok(s)) => Ok(s),
            Some(Err(e)) => {
                eprintln!("Error! {:}", e);
                Err(e)
            }
            None => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }
}

impl Iterator for FileLines {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.lines.next() {
            Some(Ok(s)) => Some(s),
            Some(Err(e)) => {
                eprintln!("Error! {:}", e);
                None
            }
            None => None,
        }
    }
}

pub struct FileSplit {
    split: io::Split<io::BufReader<File>>,
}

impl FileSplit {
    pub fn new(filename: &str, split: &u8) -> io::Result<Self> {
        let f = File::open(filename)?;
        Ok(FileSplit {
            split: io::BufReader::new(f).split(*split),
        })
    }

    pub fn next_result(&mut self) -> io::Result<Vec<u8>> {
        match self.split.next() {
            Some(Ok(s)) => Ok(s),
            Some(Err(e)) => {
                eprintln!("Error! {:}", e);
                Err(e)
            }
            None => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }
}

impl Iterator for FileSplit {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        match self.split.next() {
            Some(Ok(s)) => Some(s),
            Some(Err(e)) => {
                eprintln!("Error! {:}", e);
                None
            }
            None => None,
        }
    }
}
