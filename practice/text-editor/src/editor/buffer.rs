use std::default;
use std::io::Error;
use std::path;

#[derive(Debug, Default)]
pub struct Buffer {
    title: String,
    pub data: Vec<String>,
}

impl Buffer {
    pub(crate) fn load(file_name: &str) -> Result<Buffer, Error> {
        let path = path::Path::new(file_name);
        let data = std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        Ok(Buffer {
            title: file_name.to_string(),
            data,
        })
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
        assert_eq!(buffer.data, Vec::<String>::default());
    }
}
