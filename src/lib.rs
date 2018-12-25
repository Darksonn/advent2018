use std::io::{BufRead, BufReader, Lines};
use std::fs::File;

pub struct DataReader {
    lines: Lines<BufReader<File>>,
}

impl DataReader {
    pub fn open(day: usize) -> Self {
        let f = match File::open(format!("input/{}.txt", day)) {
            Ok(f) => f,
            Err(_) => panic!("Input for day {} is missing.", day),
        };
        let f = BufReader::new(f);
        DataReader {
            lines: f.lines(),
        }
    }
}

impl Iterator for DataReader {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.lines.next().map(|res| res.expect("IO error while reading string."))
    }
}


