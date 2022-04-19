use std::io::Stdout;

use snake_in_terminal::terminus::{
    screen::SharedScreen,
    style::{Style, StyleProperty},
};

use super::Position;

pub struct ScoreComponent {
    screen: SharedScreen<Stdout>,
    score: u32,
    position: Position,
    style: Style,
}

impl ScoreComponent {
    pub fn try_new(screen: SharedScreen<Stdout>, position: Position) -> super::Result<Self> {
        let score = Self {
            screen,
            position,
            score: 0,
            style: Style::from([StyleProperty::Dim]),
        };
        score.render()?;
        Ok(score)
    }

    pub fn add(&mut self, add: u32) -> super::Result<()> {
        self.score += add;
        self.render()
    }

    fn render(&self) -> super::Result<()> {
        let Position { x, y } = self.position;
        let mut screen = self.screen.lock()?;
        screen.cursor_mut().move_to(x, y)?;
        screen.write_str(self.style.ansi_sequence().as_str())?;
        screen.write_str(&format!("Score: {}", self.score))?;
        screen.write_str(Style::RESET)?;
        Ok(())
    }
}
