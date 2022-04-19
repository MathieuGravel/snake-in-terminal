use std::{io::Stdout, sync::Arc};

use snake_in_terminal::terminus::{
    screen::SharedScreen,
    style::{Color, Style, StyleProperty},
};
use tokio::sync::Mutex;

use crate::component::{
    self,
    border::BorderComponent,
    game_board::GameBoardComponent,
    game_over::{GameOverComponent, GAME_OVER_HEIGHT, GAME_OVER_WIDTH},
    timer::TimerComponent,
    Boundary, Dimension, Position,
};

pub fn create_application_timer(
    screen: SharedScreen<Stdout>,
) -> component::Result<Arc<Mutex<TimerComponent>>> {
    let screen_dimension: Dimension = screen.lock()?.size().into();
    let position = Position {
        x: screen_dimension.width - 12,
        y: 1,
    };
    let timer = TimerComponent::new(screen, position);
    timer.render()?;
    let timer = Arc::new(Mutex::new(timer));
    Ok(timer)
}

pub fn create_application_border(
    screen: SharedScreen<Stdout>,
) -> component::Result<BorderComponent> {
    let screen_dimension: Dimension = screen.lock()?.size().into();
    let boundary = Boundary::new(
        Position::new(1, 2),
        Dimension::new(screen_dimension.width, screen_dimension.height - 1),
    );
    let style = Style::from([StyleProperty::Color(Color::RGB(255, 255, 255))]);
    let border = BorderComponent::new(screen, boundary, style);
    border.render()?;
    Ok(border)
}

pub fn create_application_game_board(
    screen: SharedScreen<Stdout>,
) -> component::Result<Arc<Mutex<GameBoardComponent>>> {
    let screen_dimension: Dimension = screen.lock()?.size().into();
    let game_board = GameBoardComponent::new(screen, get_game_board_boundary(screen_dimension))?;
    let game_board = Arc::new(Mutex::new(game_board));
    Ok(game_board)
}

pub fn create_application_game_over_message(
    screen: SharedScreen<Stdout>,
) -> component::Result<Arc<Mutex<GameOverComponent>>> {
    let screen_dimension: Dimension = screen.lock()?.size().into();
    let position = Position::new(
        (screen_dimension.width - GAME_OVER_WIDTH) / 2,
        (screen_dimension.height - GAME_OVER_HEIGHT) / 2,
    );
    let game_over = Arc::new(Mutex::new(GameOverComponent::new(screen, position)));
    Ok(game_over)
}

fn get_game_board_boundary(screen_dimension: Dimension) -> Boundary {
    Boundary::new(
        Position::new(2, 3),
        Dimension::new(screen_dimension.width - 2, screen_dimension.height - 3),
    )
}
