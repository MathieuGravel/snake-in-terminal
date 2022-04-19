use std::io::Stdout;

use snake_in_terminal::terminus::{screen::SharedScreen, style::Style};

use super::{Boundary, Dimension, Position};

const TOP_LEFT_CORNER: char = '╭';
const TOP_RIGHT_CORNER: char = '╮';
const BOTTOM_RIGHT_CORNER: char = '╯';
const BOTTOM_LEFT_CORNER: char = '╰';
const LINE: char = '─';
const COLUMN: char = '│';

pub struct BorderComponent {
    screen: SharedScreen<Stdout>,
    style: Style,
    boundary: Boundary,
}

impl BorderComponent {
    pub fn new(screen: SharedScreen<Stdout>, boundary: Boundary, style: Style) -> BorderComponent {
        Self {
            screen,
            style,
            boundary,
        }
    }

    pub fn render(&self) -> super::Result<()> {
        self.render_border_with(
            TOP_LEFT_CORNER,
            TOP_RIGHT_CORNER,
            BOTTOM_RIGHT_CORNER,
            BOTTOM_LEFT_CORNER,
            LINE,
            COLUMN,
            LINE,
            COLUMN,
        )
    }

    pub fn erase(&self) -> super::Result<()> {
        self.render_border_with(' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ')
    }

    fn render_border_with(
        &self,
        top_left_corner: char,
        top_right_corner: char,
        bottom_right_corner: char,
        bottom_left_corner: char,
        top_line: char,
        right_line: char,
        bottom_line: char,
        left_line: char,
    ) -> super::Result<()> {
        let Position { x, y } = self.boundary.position();
        let Dimension { width, height } = self.boundary.dimension();
        let mut screen = self.screen.lock()?;

        screen.write_str(&self.style.ansi_sequence())?;

        // Top left corner.
        screen.cursor_mut().move_to(x, y)?;
        screen.write_char(top_left_corner)?;
        // Top line.
        screen.write_str(&top_line.to_string().repeat((width - 2) as usize))?;
        // Top right corner.
        screen.write_char(top_right_corner)?;
        // Right column.
        for _ in 1..height - 1 {
            if x + width - 1 < width {
                screen.cursor_mut().move_left(1)?;
            }
            screen.cursor_mut().move_down(1)?;
            screen.write_char(right_line)?;
        }
        // Left column.
        screen.cursor_mut().move_to(x, y)?;
        for _ in 1..height - 1 {
            screen.cursor_mut().move_down(1)?;
            screen.write_char(left_line)?;
            screen.cursor_mut().move_left(1)?;
        }
        // Bottom left corner.
        screen.cursor_mut().move_down(1)?;
        screen.write_char(bottom_left_corner)?;
        // Bottom Line.
        screen.write_str(&bottom_line.to_string().repeat((width - 2) as usize))?;
        // Bottom right corner.
        screen.write_char(bottom_right_corner)?;

        screen.write_str(Style::RESET)?;
        Ok(())
    }
}

impl Drop for BorderComponent {
    fn drop(&mut self) {
        let _ = self.erase();
    }
}
