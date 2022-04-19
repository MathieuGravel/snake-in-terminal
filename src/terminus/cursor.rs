use std::io;

use crate::terminus::ansi_escape_sequences::CSI;

pub struct Cursor<T: io::Write> {
    out: T,
}

impl<T> Cursor<T>
where
    T: io::Write,
{
    pub fn from(op: impl Fn() -> T) -> Self {
        Self { out: op() }
    }

    pub fn hide(&mut self) -> std::io::Result<()> {
        self.out.write(CSI::HideCursor.to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn show(&mut self) -> std::io::Result<()> {
        self.out.write(CSI::ShowCursor.to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn save_position(&mut self) -> std::io::Result<()> {
        self.out
            .write(CSI::SaveCursorPosition.to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn restore_position(&mut self) -> std::io::Result<()> {
        self.out
            .write(CSI::RestoreCursorPosition.to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_to(&mut self, x: u16, y: u16) -> std::io::Result<()> {
        self.out
            .write(CSI::CursorPosition(y, x).to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_up(&mut self, n: u16) -> std::io::Result<()> {
        self.out.write(CSI::CursorUp(n).to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_down(&mut self, n: u16) -> std::io::Result<()> {
        self.out.write(CSI::CursorDown(n).to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_left(&mut self, n: u16) -> std::io::Result<()> {
        self.out.write(CSI::CursorBack(n).to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_right(&mut self, n: u16) -> std::io::Result<()> {
        self.out
            .write(CSI::CursorForward(n).to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_horizontally(&mut self, x: u16) -> std::io::Result<()> {
        self.out
            .write(CSI::CursorHorizontalAbsolute(x).to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_to_n_previous_line(&mut self, n: u16) -> std::io::Result<()> {
        self.out
            .write(CSI::CursorPreviousLine(n).to_string().as_bytes())?;
        self.out.flush()
    }

    pub fn move_to_n_next_line(&mut self, n: u16) -> std::io::Result<()> {
        self.out
            .write(CSI::CursorNextLine(n).to_string().as_bytes())?;
        self.out.flush()
    }
}
