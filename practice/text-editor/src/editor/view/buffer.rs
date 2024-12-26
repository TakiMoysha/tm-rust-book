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
    pub(crate) fn load(file_name: &str) -> Result<Buffer, Error> {
        let path = path::Path::new(file_name);
        let data = std::fs::read_to_string(path)
            .expect("Can't read the file, check the path")
            .lines()
            .map(Line::from)
            .collect::<Vec<Line>>();
        let title = path.file_name().unwrap().to_string_lossy().to_string();
        Ok(Buffer { title, data })
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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
}
