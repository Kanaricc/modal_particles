use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read, Write},
};
use utils::Buffer;

mod utils;

const WORD_DICT1: [&'static str; 16] = [
    "嗯", "啊", "呜", "唔", "咿", "呼", "哈啊", "嗯啊", "呜啊", "咿啊", "呼啊", "呜嗯", "呜呜",
    "啊嗯", "啊啊", "咿呜",
];
const WORD_DICT2: [&'static str; 3] = ["!", "..", "~"];

lazy_static! {
    static ref WORD2ID: HashMap<&'static str, usize> = {
        let key: Vec<_> = (0..WORD_DICT1.len()).collect();
        WORD_DICT1.into_iter().zip(key.into_iter()).collect()
    };
    static ref ID2WORD: HashMap<usize, &'static str> = WORD_DICT1.into_iter().enumerate().collect();
    static ref BYTE_SET: HashSet<&'static u8> =
        WORD_DICT1.iter().flat_map(|x| x.as_bytes()).collect();
}

pub struct Encoder<W: Write> {
    writer: W,
}

impl<W: Write> Encoder<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
    pub fn get_writer(self) -> W {
        self.writer
    }
}

impl<W: Write> Write for Encoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut idx=0usize;
        for byte in buf.iter() {
            let (a, b) = (byte & 0xF, (byte >> 4) & 0xF);
            let (a, b) = (
                ID2WORD.get(&(a as usize)).unwrap(),
                ID2WORD.get(&(b as usize)).unwrap(),
            );
            self.writer.write_all(a.as_bytes())?;
            self.writer
                .write_all(WORD_DICT2.get(idx).unwrap().as_bytes())?;
            idx=(idx+1)%WORD_DICT2.len();
            self.writer.write_all(b.as_bytes())?;
            self.writer
                .write_all(WORD_DICT2.get(idx).unwrap().as_bytes())?;
            idx=(idx+1)%WORD_DICT2.len();
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub struct Decoder<W: Write> {
    writer: W,
    buffer: Buffer,
}

impl<W: Write> Decoder<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            buffer: Buffer::new(),
        }
    }
    pub fn get_writer(self) -> W {
        self.writer
    }
}

impl<W: Write> Write for Decoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let (s, len) = match std::str::from_utf8(buf) {
            Ok(s) => (s, buf.len()),
            Err(err) => unsafe {
                let valid_len = err.valid_up_to();
                (std::str::from_utf8_unchecked(buf), valid_len)
            },
        };
        let mut s = s.to_string();
        for d in WORD_DICT2 {
            s = s.replace(d, "/");
        }
        let u8_list: Vec<_> = s
            .split("/")
            .filter(|x| !x.is_empty())
            .map(|x| WORD2ID.get(x).unwrap())
            .map(|x| *x as u8)
            .collect();
        self.buffer.write_all(&u8_list)?;

        while self.buffer.len() >= 2 {
            let mut t: [u8; 2] = [0, 0];
            self.buffer.read(&mut t)?;
            let t = t[0] | (t[1] << 4);
            self.writer.write(&[t])?;
        }
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.buffer.len() != 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid input"));
        }
        self.writer.flush()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn encode() {
        // [230, 136, 145, 232, 131, 189, 229, 144, 158, 228, 184, 139, 231, 142, 187, 231, 146, 131, 232, 128, 140, 228, 184, 141, 228, 188, 164, 232, 186, 171, 228, 189, 147]
        let s = "我能吞下玻璃而不伤身体";
        let mut encoder = Encoder::new(Vec::new());
        encoder.write_all(s.as_bytes()).unwrap();

        String::from_utf8(encoder.get_writer()).unwrap();
    }
    #[test]
    fn decode() {
        let s = "呼~啊啊...呜.呜啊...呜嗯.呜嗯...咿啊~啊啊!嗯~呜啊..呜呜...呜嗯...";
        let mut decoder = Decoder::new(Vec::new());
        decoder.write_all(s.as_bytes()).unwrap();

        String::from_utf8(decoder.get_writer()).unwrap();
    }

    #[test]
    fn encode_and_decode(){
        let s = "我能吞下玻璃而不伤身体！I am the storm that is approaching!";
        let mut encoder = Encoder::new(Vec::new());
        encoder.write_all(s.as_bytes()).unwrap();
        let encoded=String::from_utf8(encoder.get_writer()).unwrap();

        let mut decoder = Decoder::new(Vec::new());
        decoder.write_all(encoded.as_bytes()).unwrap();
        let ss=String::from_utf8(decoder.get_writer()).unwrap();
        assert_eq!(s,ss);
    }
}
