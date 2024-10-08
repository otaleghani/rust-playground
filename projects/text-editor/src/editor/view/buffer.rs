use std::fs::read_to_string;
use std::io::Error;
use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        if self.lines.len() == 0 {
            return true;
        }
        false
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }
    
    // pub fn longest(&self) -> usize {
    //     let mut max = 0;
    //     for line in &self.lines {
    //         if line.lenght() > max {
    //             max = line.lenght();
    //         }
    //     }
    //     max
    // }
}
