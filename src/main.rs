mod component;
mod game_input;
mod layout;
mod snake_game;

use std::io;

use component::snake::Direction;
use snake_game::SnakeGame;
use snake_in_terminal::terminus::screen::{Screen, SharedScreen};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (game_tx, mut game_rx) = mpsc::unbounded_channel();
    let rendering_handle = tokio::spawn(async move {
        let mut screen = Screen::new(io::stdout);
        screen.clear_screen()?;
        screen.cursor_mut().hide()?;
        let shared_screen = SharedScreen::new(screen);
        let mut snake_game = SnakeGame::new(
            layout::create_application_timer(SharedScreen::clone(&shared_screen))?,
            layout::create_application_game_board(SharedScreen::clone(&shared_screen))?,
            layout::create_application_game_over_message(SharedScreen::clone(&shared_screen))?,
            layout::create_application_border(SharedScreen::clone(&shared_screen))?,
        );
        snake_game.start_game_loop().await;

        while let Some(input) = game_rx.recv().await {
            if let Some(direction) = match input {
                game_input::GameInput::Up => Some(Direction::Up),
                game_input::GameInput::Down => Some(Direction::Down),
                game_input::GameInput::Left => Some(Direction::Left),
                game_input::GameInput::Right => Some(Direction::Right),
                _ => None,
            } {
                snake_game
                    .game_board_mut()
                    .lock()
                    .await
                    .snake_component_mut()
                    .snake_mut()
                    .change_direction(direction);
            }
            match input {
                game_input::GameInput::Quit => break,
                _ => {}
            }
        }
        let mut screen = shared_screen.lock()?;
        screen.erase_screen()?;
        screen.cursor_mut().show()?;
        component::Result::Ok(())
    });

    if let Err(error) = game_input::read_inputs(game_tx) {
        println!("{:?}", error.to_string());
    }

    if let Err(error) = rendering_handle.await {
        println!("{error:?}");
    }
}
