use std::io::Stdout;

use accessors_rs::Accessors;

use snake_in_terminal::terminus::screen::SharedScreen;

use super::{
    apple::AppleComponent, score::ScoreComponent, snake::SnakeComponent, Boundary, Position,
};

#[derive(Accessors)]
#[accessors(get, get_mut)]
pub struct GameBoardComponent {
    screen: SharedScreen<Stdout>,
    #[accessors(get_copy)]
    boundary: Boundary,
    apple: AppleComponent,
    snake_component: SnakeComponent,
    score: ScoreComponent,
}

impl GameBoardComponent {
    pub fn new(
        screen: SharedScreen<Stdout>,
        boundary: Boundary,
    ) -> super::Result<GameBoardComponent> {
        let apple = AppleComponent::new(
            SharedScreen::clone(&screen),
            boundary.position() + boundary.dimension().get_random_position_inside(),
        )?;
        Ok(Self {
            screen: SharedScreen::clone(&screen),
            apple,
            snake_component: SnakeComponent::try_new(
                SharedScreen::clone(&screen),
                boundary.position(),
            )?,
            score: ScoreComponent::try_new(SharedScreen::clone(&screen), Position::new(1, 1))?,
            boundary,
        })
    }

    pub fn generate_new_apple(&mut self) -> super::Result<()> {
        let apple = AppleComponent::new(
            SharedScreen::clone(&self.screen),
            self.boundary.position() + self.boundary.dimension().get_random_position_inside(),
        )?;
        self.apple = apple;
        Ok(())
    }
}
