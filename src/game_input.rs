use crossterm::event::{self, Event, KeyCode};
use tokio::sync::mpsc::{error::SendError, UnboundedSender};

pub enum GameInput {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

pub fn read_inputs(tx: UnboundedSender<GameInput>) -> Result<(), SendError<GameInput>> {
    while let Ok(event) = event::read() {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Up => tx.send(GameInput::Up)?,
                KeyCode::Down => tx.send(GameInput::Down)?,
                KeyCode::Left => tx.send(GameInput::Left)?,
                KeyCode::Right => tx.send(GameInput::Right)?,
                KeyCode::Char('q') => {
                    tx.send(GameInput::Quit)?;
                    break;
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(())
}
