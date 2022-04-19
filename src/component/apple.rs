use std::io::Stdout;

use accessors_rs::Accessors;
use snake_in_terminal::terminus::{
    screen::SharedScreen,
    style::{Color, Style, StyleProperty},
};

use super::Position;

#[derive(Accessors)]
pub struct AppleComponent {
    style: Style,
    #[accessors(get_copy)]
    position: Position,
    screen: SharedScreen<Stdout>,
}

impl AppleComponent {
    pub fn new(screen: SharedScreen<Stdout>, position: Position) -> super::Result<Self> {
        let apple = Self {
            position,
            screen,
            style: Style::from([
                StyleProperty::Bold,
                StyleProperty::Color(Color::RGB(235, 35, 55)),
            ]),
        };
        apple.render()?;
        Ok(apple)
    }

    fn render(&self) -> super::Result<()> {
        let mut screen = self.screen.lock()?;
        let Position { x, y } = self.position;
        screen.cursor_mut().move_to(x, y)?;
        screen.write_str(self.style.ansi_sequence().as_str())?;
        screen.write_str("▄")?;
        screen.write_str(Style::RESET)?;
        Ok(())
    }

    fn erase(&self) -> super::Result<()> {
        if let Ok(mut screen) = self.screen.lock() {
            let Position { x, y } = self.position;
            screen.cursor_mut().move_to(x, y)?;
            screen.write_str(" ")?;
        }
        Ok(())
    }
}

impl Drop for AppleComponent {
    fn drop(&mut self) {
        let _ = self.erase();
    }
}
