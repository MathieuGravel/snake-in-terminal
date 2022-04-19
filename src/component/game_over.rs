use std::io::Stdout;

use accessors_rs::Accessors;
use snake_in_terminal::terminus::screen::SharedScreen;

use super::Position;

pub const GAME_OVER_WIDTH: u16 = 74;
pub const GAME_OVER_HEIGHT: u16 = 8;
const GAME_OVER: [&'static str; GAME_OVER_HEIGHT as usize] = [
    " ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗███████╗██████╗ ",
    "██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║██╔════╝██╔══██╗",
    "██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║█████╗  ██████╔╝",
    "██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗",
    "╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║",
    " ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝",
    "                                                                          ",
    "                          Press q to quit game!                           ",
];

#[derive(Accessors)]
pub struct GameOverComponent {
    screen: SharedScreen<Stdout>,
    #[accessors(get_copy)]
    position: Position,
}

impl GameOverComponent {
    pub fn new(screen: SharedScreen<Stdout>, position: Position) -> Self {
        Self { screen, position }
    }

    pub fn render(&self) -> super::Result<()> {
        let mut screen = self.screen.lock()?;
        let Position { x, y } = self.position;
        for (i, y) in (y..y + GAME_OVER_HEIGHT).enumerate() {
            screen.cursor_mut().move_to(x, y)?;
            screen.write_str(GAME_OVER[i])?;
        }
        Ok(())
    }

    pub fn erase(&self) -> super::Result<()> {
        let mut screen = self.screen.lock()?;
        let Position { x, y } = self.position;
        for y in y..y + GAME_OVER_HEIGHT {
            screen.cursor_mut().move_to(x, y)?;
            screen.write_str(&" ".repeat(GAME_OVER_WIDTH.into()))?;
        }
        Ok(())
    }
}

impl Drop for GameOverComponent {
    fn drop(&mut self) {
        let _ = self.erase();
    }
}
