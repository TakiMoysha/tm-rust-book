use std::io::Error;
use std::path;
use std::string::ToString;

use super::line::Line;

#[derive(Debug, Default)]
pub struct Buffer {
    title: String,
    pub data: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Buffer, Error> {
        let path = path::Path::new(file_name);
        let data = std::fs::read_to_string(path)
            .expect("Can't read the file, check the path")
            .lines()
            .map(Line::from)
            .collect::<Vec<Line>>();
        let title = path.file_name().unwrap().to_string_lossy().to_string();
        Ok(Buffer { title, data })
    }

    pub fn from_str(s: &str) -> Buffer {
        let data = s.lines().map(Line::from).collect::<Vec<Line>>();
        Buffer {
            title: String::from("default"),
            data,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn test_default_buffer() {
        let buffer = Buffer::default();
        assert_eq!(buffer.data, Vec::<Line>::default());
    }

    #[test]
    fn test_height_buffer() {
        let test_lines: &str = "a;lskdhga\nsa;ldkjf;aslkd";
        let buffer = Buffer::from_str(test_lines);
        assert_eq!(buffer.height(), 2);
    }
}
