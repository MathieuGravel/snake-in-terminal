use derive_deref_rs::Deref;
use std::{
    io::{self, Write},
    sync::{Arc, Mutex, MutexGuard},
};

use error_chain::error_chain;
use terminal_size::{Height, Width};

use crate::terminus::{
    ansi_escape_sequences::{EraseOption, CSI},
    cursor::Cursor,
};

error_chain! {
    errors {
        CannotLockSharedScreen
        CannotOutputInNonTTYOutput {
            description("The Snake game can only be output in terminal.")
        }
    }

    foreign_links {
        Io(std::io::Error);
    }
}

pub struct Screen<T: Write + Send> {
    out: T,
    cursor: Cursor<T>,
}

impl<T: Write + Send> Screen<T> {
    pub fn new(op: impl Fn() -> T) -> Screen<T> {
        Self {
            out: op(),
            cursor: Cursor::from(op),
        }
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor<T> {
        &mut self.cursor
    }

    pub fn try_size(&self) -> Result<(u16, u16)> {
        if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
            Ok((w, h))
        } else {
            Err(Error::from_kind(ErrorKind::CannotOutputInNonTTYOutput))
        }
    }

    pub fn size(&self) -> (u16, u16) {
        self.try_size().unwrap_or((0, 0).into())
    }

    pub fn clear_screen(&mut self) -> std::io::Result<()> {
        self.out
            .write(CSI::EraseInDisplay(EraseOption::All).to_string().as_bytes())?;
        self.cursor.move_to(1, 1)?;
        self.out.flush()
    }

    pub fn erase_screen(&mut self) -> std::io::Result<()> {
        self.cursor.move_to(1, 1)?;
        self.out.write(
            CSI::EraseInDisplay(EraseOption::CursorToEnd)
                .to_string()
                .as_bytes(),
        )?;
        self.out.flush()
    }

    pub fn scroll_to_bottom(&mut self) -> Result<()> {
        let height = self.try_size()?.0;
        self.out
            .write(CSI::ScrollDown(height).to_string().as_bytes())?;
        self.cursor.move_to(1, height)?;
        self.out.flush()?;
        Ok(())
    }

    /// Scroll Content down without bringing history in the screen.
    pub fn scroll_down(&mut self, n: u16) -> Result<()> {
        self.out.write(CSI::ScrollDown(n).to_string().as_bytes())?;
        self.out.flush()?;
        Ok(())
    }

    /// Scroll Content up but doest not write if higher than the screen.
    pub fn scroll_up(&mut self, n: u16) -> Result<()> {
        self.out.write(CSI::ScrollUp(n).to_string().as_bytes())?;
        self.out.flush()?;
        Ok(())
    }

    pub fn writeln(&mut self) -> Result<()> {
        self.out.write(&['\n' as u8])?;
        self.out.flush()?;
        Ok(())
    }

    pub fn write_str(&mut self, str: &str) -> Result<()> {
        self.out.write(str.as_bytes())?;
        self.out.flush()?;
        Ok(())
    }

    pub fn writeln_str(&mut self, str: &str) -> Result<()> {
        self.out.write(str.as_bytes())?;
        self.out.write(&['\n' as u8])?;
        self.out.flush()?;
        Ok(())
    }

    pub fn write_char(&mut self, c: char) -> Result<()> {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        c.encode_utf8(&mut buf);
        self.out.write(&buf)?;
        self.out.flush()?;
        Ok(())
    }
}

pub struct SharedScreen<T: Write + Send> {
    screen: Arc<Mutex<Screen<T>>>,
}

impl<T: Write + Send> SharedScreen<T> {
    pub fn new(screen: Screen<T>) -> SharedScreen<T> {
        Self {
            screen: Arc::new(Mutex::new(screen)),
        }
    }

    pub fn lock(&self) -> Result<SharedScreenGuard<'_, T>> {
        let screen = self
            .screen
            .lock()
            .map_err(|_| Error::from_kind(ErrorKind::CannotLockSharedScreen))?;
        let screen_guard = SharedScreenGuard::new(screen)?;
        Ok(screen_guard)
    }
}

impl<T: Write + Send> Clone for SharedScreen<T> {
    fn clone(&self) -> Self {
        Self {
            screen: Arc::clone(&self.screen),
        }
    }
}

#[derive(Deref)]
pub struct SharedScreenGuard<'a, T: Write + Send> {
    screen_guard: MutexGuard<'a, Screen<T>>,
}

impl<'a, T: Write + Send> SharedScreenGuard<'a, T> {
    pub fn new(mutex: MutexGuard<'a, Screen<T>>) -> io::Result<Self> {
        let mut ssg = Self {
            screen_guard: mutex,
        };
        ssg.cursor_mut().save_position()?;
        Ok(ssg)
    }
}

impl<'a, T: Write + Send> Drop for SharedScreenGuard<'a, T> {
    fn drop(&mut self) {
        let _ = self.cursor_mut().restore_position();
    }
}
