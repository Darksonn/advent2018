use advent2018::DataReader;
use std::hash::{Hash, Hasher};
use std::fmt;
use fnv::FnvHashSet;

fn main() {
    let input = DataReader::open(2);

    let mut ids = Vec::new();
    for line in input {
        ids.push(BoxID::new(line));
    }

    let checksum = ChecksumValue::checksum(ids.iter().map(|id| id.checksum()));
    println!("Part one: {}", checksum);

    let partial = find_matching_partial(&ids).expect("No BoxIDs differ by one");
    println!("Part two: {}", partial);
}

fn find_matching_partial<'a>(ids: &'a [BoxID]) -> Option<PartialBoxID<'a>> {
    let mut seen = FnvHashSet::default();
    for boxid in ids {
        for partial in boxid.partial() {
            if !seen.insert(partial) {
                // We have already seen this partial.
                return Some(partial);
            }
        }
    }
    None
}

pub struct BoxID {
    /// We use a byte array to avoid handling utf-8.
    id: Vec<u8>,
}
impl BoxID {
    pub fn new(id: String) -> Self {
        BoxID {
            id: id.into(),
        }
    }
    /// Count how many times every character occurs.
    fn count_characters(&self) -> [usize; 256] {
        let mut count = [0; 256];
        for &byte in &self.id {
            count[byte as usize] += 1;
        }
        count
    }
    pub fn checksum(&self) -> ChecksumValue {
        let mut res = ChecksumValue::new();
        for &count in self.count_characters().iter() {
            if count == 2 {
                res.has_double = true;
            }
            if count == 3 {
                res.has_triple = true;
            }
        }
        res
    }
    pub fn partial<'a>(&'a self) -> impl Iterator<Item = PartialBoxID<'a>> {
        let slice = &self.id[..];
        (0..self.id.len()).map(move |index| {
            PartialBoxID {
                id: slice,
                hidden: index,
            }
        })
    }
}

pub struct ChecksumValue {
    pub has_double: bool,
    pub has_triple: bool,
}
impl ChecksumValue {
    pub fn new() -> Self {
        ChecksumValue {
            has_double: false,
            has_triple: false,
        }
    }
    pub fn checksum<I>(iter: I) -> u64
    where
        I: Iterator<Item = ChecksumValue>,
    {
        let mut doubles = 0;
        let mut triples = 0;
        for check in iter {
            if check.has_double {
                doubles += 1;
            }
            if check.has_triple {
                triples += 1;
            }
        }
        doubles * triples
    }
}

/// A box id where one character is hidden.
///
/// If two PartialBoxIDs are equal, their BoxIDs either differ by one character,
/// or are equal.
#[derive(Eq,Clone,Copy)]
pub struct PartialBoxID<'a> {
    id: &'a [u8],
    hidden: usize,
}
impl<'a> PartialBoxID<'a> {
    pub fn bytes(&self) -> impl Iterator<Item = u8> + 'a {
        let (first, last) = self.id.split_at(self.hidden);
        let first = first.iter().cloned();
        let middle = std::iter::once(0u8);
        let last = last.iter().skip(1).cloned();
        first.chain(middle).chain(last)
    }
    pub fn len(&self) -> usize {
        self.id.len()
    }
}
impl<'a, 'b> PartialEq<PartialBoxID<'b>> for PartialBoxID<'a> {
    fn eq(&self, other: &PartialBoxID<'b>) -> bool {
        if self.len() == other.len() {
            self.bytes().eq(other.bytes())
        } else {
            false
        }
    }
}
/// Our hash implementation must ensure that partial id's have the same hash if they are
/// equal, so the hidden index must be ignored.
impl<'a> Hash for PartialBoxID<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.bytes() {
            byte.hash(state);
        }
    }
}
impl<'a> fmt::Display for PartialBoxID<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str::from_utf8;
        let (first, last) = self.id.split_at(self.hidden);
        let last = &last[1..];
        from_utf8(first).unwrap().fmt(f)?;
        from_utf8(last).unwrap().fmt(f)
    }
}
