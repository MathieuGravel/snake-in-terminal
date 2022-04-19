use std::{io::Stdout, sync::Weak, time::Duration};

use accessors_rs::Accessors;
use tokio::{sync::Mutex, task::JoinHandle};

use snake_in_terminal::terminus::{
    screen::SharedScreen,
    style::{Style, StyleProperty},
};

use super::Position;

pub const LABEL: &'static str = "Timer: ";

#[derive(Accessors)]
pub struct TimerComponent {
    screen: SharedScreen<Stdout>,
    style: Style,
    timer_handle: Option<JoinHandle<super::Result<()>>>,
    position: Position,
    seconds: u32,
}

impl TimerComponent {
    pub fn new(screen: SharedScreen<Stdout>, position: Position) -> Self {
        Self {
            screen,
            position,
            seconds: 0,
            timer_handle: None,
            style: Style::from([StyleProperty::Dim]),
        }
    }

    fn text(&self) -> String {
        let min = self.seconds / 60;
        let sec = self.seconds - (min * 60);
        format!("{LABEL}{:02}:{:02}", min, sec)
    }

    pub fn render(&self) -> super::Result<()> {
        let Position { x, y } = self.position;
        let mut screen = self.screen.lock()?;
        screen.cursor_mut().move_to(x, y)?;
        screen.write_str(&self.style.prettify(&self.text()))?;
        Ok(())
    }

    fn erase(&self) -> super::Result<()> {
        let Position { x, y } = self.position;
        let mut screen = self.screen.lock()?;
        screen.cursor_mut().move_to(x, y)?;
        screen.write_str(&" ".repeat(self.text().len()))?;
        Ok(())
    }

    fn abort_handle(&mut self) {
        if let Some(handle) = self.timer_handle.take() {
            handle.abort();
        }
    }

    pub async fn start_timer(timer: Weak<Mutex<Self>>) {
        let weak_timer = Weak::clone(&timer);

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                if let Some(timer) = timer.upgrade() {
                    let mut timer = timer.lock().await;
                    timer.seconds += 1;
                    timer.render()?;
                } else {
                    break;
                }
            }
            Ok(())
        });

        if let Some(timer) = weak_timer.upgrade() {
            let mut timer = timer.lock().await;
            timer.abort_handle();
            timer.timer_handle.replace(handle);
        }
    }
}

impl Drop for TimerComponent {
    fn drop(&mut self) {
        let _ = self.erase();
        self.abort_handle();
    }
}
