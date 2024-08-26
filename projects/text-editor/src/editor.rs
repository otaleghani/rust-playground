use std::io::stdout;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, size};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::cursor::MoveTo;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor{ should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        Self::draw_rows();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), std::io::Error>{
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }

    fn draw_rows() {
        let screen_size = size();

        match screen_size {
            Ok((x, y)) => {
                for i in 0..y {
                    execute!(stdout(), MoveTo(0, i));
                    print!("~");
                }
            },
            Err(err) => ()
        }
    }
}
