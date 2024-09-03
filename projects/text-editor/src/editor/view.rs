use std::io::{Error};
use super::terminal::{Terminal, Size, Position};
mod buffer;
use buffer::{Buffer};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }


    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line)?;

            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;

            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }



    // fn draw_welcome_message() -> Result<(), Error> {
    //     let mut welcome_message = format!("{NAME} {VERSION}");
    //     let width = Terminal::size()?.width;
    //     let len = welcome_message.len();
    //     #[allow(clippy::integer_division)]
    //     let padding = width.saturating_sub(len) / 2;
    //     let spaces = " ".repeat(padding.saturating_sub(1));
    //     welcome_message = format!("~{spaces} {welcome_message}");
    //     welcome_message.truncate(width);
    //     Terminal::print(&welcome_message)?;
    //     Ok(())
    // }
    //
    // fn draw_empty_row() -> Result<(), Error> {
    //     Terminal::print("~")?;
    //     Ok(())
    // }
    //
    // pub fn render_welcome_screen() -> Result<(), Error> {
    //     let Size{ height, .. } = Terminal::size()?;
    //     for current_row in 0..height {
    //         Terminal::clear_line()?;
    //         #[allow(clippy::integer_division)]
    //         if current_row == height / 3 {
    //             Self::draw_welcome_message()?;
    //         } else {
    //             Self::draw_empty_row()?;
    //         }
    //         if current_row.saturating_add(1) < height {
    //             
    //             // Terminal::print("\r\n")?;
    //         }
    //     }
    //     Ok(())
    // }
    // pub fn render_buffer(&self) -> Result<(), Error> {
    //     let Size{ height, .. } = Terminal::size()?;
    //     for current_row in 0..height {
    //         Terminal::clear_line()?;
    //         if let Some(line) = self.buffer.lines.get(current_row) {
    //             let _ = Terminal::print(line);
    //             let _ = Terminal::print("\r\n");
    //             continue;
    //         } else {
    //             Self::draw_empty_row()?;
    //         }
    //         if current_row.saturating_add(1) < height {
    //             // Terminal::print("\r\n")?;
    //         }
    //     }
    //     Ok(())
    // }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
