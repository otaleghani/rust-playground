use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor{ should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("err:#?");
        }
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error>{
        print!("\x1b[2J");
        enable_raw_mode()?;
        print!("\x1b[2J");
        loop {
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?}, State: {state:?}\r");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    },
                    _ => ()
                }
                if self.should_quit {
                    break;
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
