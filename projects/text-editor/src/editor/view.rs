use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Terminal, Size, Position},
};
mod buffer;
use buffer::{Buffer};
mod location;
use location::Location;
mod line;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;
        let top = self.scroll_offset.y;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));


                let truncated_line = if line.len() <= self.scroll_offset.x || line.len() == 0 {
                    ""
                } else if line.len() >= self.scroll_offset.x && line.len() - self.scroll_offset.x >= width {
                    &line[self.scroll_offset.x..width+self.scroll_offset.x]
                } else {
                    &line[self.scroll_offset.x..]
                };
                Self::render_line(current_row, truncated_line);

            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));

            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;

        let _ = Terminal::move_caret_to(Position{
            col: self.location.x,
            row: self.location.y,
        });
        //Ok(())
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

    pub fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Location { x: mut scroll_x, y: mut scroll_y } = self.scroll_offset;
        let Size { height, width } = Terminal::size().unwrap_or_default();

        match key_code {
            KeyCode::Up => {
                if self.location.y == 0 && self.scroll_offset.y != 0 {
                    scroll_y = scroll_y.saturating_sub(1);
                }
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                // You move down the page when you are at the edge of the screen
                if self.location.y == height.saturating_sub(1) && self.scroll_offset.y != self.buffer.lines.len() {
                    scroll_y = scroll_y.saturating_add(1);
                }
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                if self.location.x == 0 && self.scroll_offset.y != 0 {
                    scroll_x = scroll_x.saturating_sub(1);
                }
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                if self.location.x == width.saturating_sub(1) && self.scroll_offset.y != self.buffer.longest() {
                    scroll_x = scroll_x.saturating_add(1);
                }
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
                scroll_y = 0;
            }
            KeyCode::PageDown => {
                y = self.buffer.lines.len();
                if y >= height.saturating_sub(1) {
                    y = height.saturating_sub(1); 
                    scroll_y = self.buffer.lines.len() - height;
                }
            }
            KeyCode::Home => {
                x = 0;
                scroll_x = 0;
            }
            KeyCode::End => {
                if let Some(line) = self.buffer.lines.get(y + scroll_y) {
                    x = line.len();
                    if x >= width.saturating_sub(1) {
                        scroll_x = x - width;
                    }
                }
            }
            _ => (),
        }
        self.location = Location { x, y };
        self.scroll_offset = Location { x: scroll_x, y: scroll_y };
        self.needs_redraw = true;
    }

    fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}
