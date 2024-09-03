use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(String::from(value));
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        if self.lines.len() == 0 {
            return true;
        }
        false
    }
}

// impl Default for Buffer {
//     fn default() -> Self {
//         Self {
//             //lines: vec!["hello world".to_string()]
//             lines: vec![]
//         }
//     }
// }
