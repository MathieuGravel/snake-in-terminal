use std::{
    sync::{Arc, Weak},
    time::Duration,
};

use accessors_rs::Accessors;

use tokio::sync::Mutex;

use crate::component::{
    self,
    border::BorderComponent,
    game_board::GameBoardComponent,
    game_over::GameOverComponent,
    timer::TimerComponent,
};

#[derive(Accessors)]
pub struct SnakeGame {
    _border: BorderComponent,
    #[accessors(get)]
    timer: Arc<Mutex<TimerComponent>>,
    #[accessors(get, get_mut)]
    game_board: Arc<Mutex<GameBoardComponent>>,
    #[accessors(get, get_mut)]
    game_over: Arc<Mutex<GameOverComponent>>,
}

impl SnakeGame {
    pub fn new(
        timer: Arc<Mutex<TimerComponent>>,
        game_board: Arc<Mutex<GameBoardComponent>>,
        game_over: Arc<Mutex<GameOverComponent>>,
        border: BorderComponent,
    ) -> Self {
        Self {
            _border: border,
            timer,
            game_board,
            game_over,
        }
    }

    pub async fn start_game_loop(&self) {
        TimerComponent::start_timer(Arc::downgrade(&self.timer)).await;

        let game_board = Arc::downgrade(self.game_board());
        let game_over = Arc::downgrade(self.game_over());

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(75));
            loop {
                interval.tick().await;
                if let Some(game_board) = Weak::upgrade(&game_board) {
                    let mut game_board = game_board.lock().await;
                    // check is it's game over.
                    if game_board.snake_component().snake().is_biting_itself()
                        || !game_board
                            .boundary()
                            .is_inside(game_board.snake_component().snake().get_next_position())
                    {
                        if let Some(game_over) = game_over.upgrade() {
                            game_over.lock().await.render()?;
                            break;
                        }
                    }
                    // check if the snake eat the apple.
                    if game_board.apple().position()
                        == game_board.snake_component().snake().head().position()
                    {
                        game_board.snake_component_mut().snake_mut().eat();
                        game_board.score_mut().add(100)?;
                        game_board.generate_new_apple()?;
                    }
                    // move snake.
                    game_board.snake_component_mut().move_forward()?;
                }
            }
            component::Result::Ok(())
        });
    }
}
