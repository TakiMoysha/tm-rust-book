use std::fs::File;
use std::io::{Error, Write};
use std::path;
use std::string::ToString;

use crate::editor::line::Line;

use super::file_info::FileInfo;
use super::CharLocation;

#[derive(Debug, Default)]
pub struct Buffer {
    pub data: Vec<Line>,
    pub file_info: FileInfo,
    pub dirty: bool,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Buffer, Error> {
        let path = path::Path::new(file_name);
        let data = std::fs::read_to_string(path)
            .expect("Can't read the file, check the path")
            .lines()
            .map(Line::from)
            .collect::<Vec<Line>>();
        Ok(Buffer {
            data,
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
    }

    fn save_to_file(&self, file_info: &FileInfo) -> Result<(), Error> {
        if let Some(file_path) = &file_info.get_path() {
            let mut file = File::create(file_path)?;
            for line in &self.data {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }

    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(file_name);
        self.save_to_file(&file_info)?;
        self.file_info = file_info;
        self.dirty = false;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.save_to_file(&self.file_info)?;
        self.dirty = false;
        Ok(())
    }

    pub const fn is_file_loaded(&self) -> bool {
        self.file_info.has_path()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn insert_char(&mut self, character: char, at: CharLocation) {
        if at.line_index > self.data.len() {
            return;
        }
        if at.line_index == self.data.len() {
            self.data.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.data.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
            self.dirty = true;
        }
    }

    pub fn delete(&mut self, at: CharLocation) {
        if let Some(line) = self.data.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count()
                && self.height() > at.line_index.saturating_add(1)
            {
                let next_line = self.data.remove(at.line_index.saturating_add(1));
                self.data[at.line_index].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_index < line.grapheme_count() {
                self.data[at.line_index].delete(at.grapheme_index);
                self.dirty = true;
            }
        }
    }

    pub fn insert_newline(&mut self, at: super::CharLocation) {
        if at.line_index == self.height() {
            self.data.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.data.get_mut(at.line_index) {
            let new = line.split(at.grapheme_index);
            self.data.insert(at.line_index.saturating_add(1), new);
            self.dirty = true;
        }
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
