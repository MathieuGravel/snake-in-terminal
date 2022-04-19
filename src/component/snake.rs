use std::{
    collections::{HashSet, LinkedList},
    io::Stdout,
};

use accessors_rs::Accessors;

use snake_in_terminal::terminus::{
    screen::SharedScreen,
    style::{Color, Style, StyleProperty},
};

use super::Position;

#[derive(Accessors)]
pub struct SnakeComponent {
    screen: SharedScreen<Stdout>,
    head_style: Style,
    body_style: Style,
    #[accessors(get, get_mut)]
    snake: Snake,
}

impl SnakeComponent {
    pub fn try_new(screen: SharedScreen<Stdout>, position: Position) -> super::Result<Self> {
        let snake = Self {
            screen,
            snake: Snake::new(SnakeNode::new(position, Direction::Right), 10),
            head_style: Style::from([
                StyleProperty::Color(Color::RGB(83, 134, 66)),
                StyleProperty::Bold,
            ]),
            body_style: Style::from([StyleProperty::Color(Color::RGB(184, 195, 52))]),
        };
        snake.render()?;
        Ok(snake)
    }

    fn render(&self) -> super::Result<()> {
        let mut screen = self.screen.lock()?;

        let last_idx = self.snake.nodes.len() - 1;

        let mut previous_body_direction = self.snake.head().direction;

        for (
            i,
            SnakeNode {
                position,
                direction,
            },
        ) in self.snake.nodes.iter().enumerate()
        {
            screen.cursor_mut().move_to(position.x, position.y)?;
            if i == 0 {
                screen.write_str(self.head_style.ansi_sequence().as_str())?;
            } else if i == 1 {
                screen.write_str(self.body_style.ansi_sequence().as_str())?;
            }
            if i == last_idx {
                screen.write_str(match previous_body_direction {
                    Direction::Up => "╿",
                    Direction::Down => "╽",
                    Direction::Left => "╾",
                    Direction::Right => "╼",
                })?;
                screen.write_str(Style::RESET)?;
            } else {
                screen.write_str(match (direction, previous_body_direction) {
                    (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => "║",
                    (Direction::Left | Direction::Right, Direction::Left | Direction::Right) => "═",
                    (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => "╗",
                    (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => "╔",
                    (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => "╝",
                    (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => "╚",
                })?;
            }
            previous_body_direction = direction.clone();
        }
        Ok(())
    }

    fn erase(&self) -> super::Result<()> {
        let mut screen = self.screen.lock()?;
        for SnakeNode {
            position: Position { x, y },
            ..
        } in &self.snake.nodes
        {
            screen.cursor_mut().move_to(*x, *y)?;
            screen.write_str(" ")?;
        }
        Ok(())
    }

    pub fn move_forward(&mut self) -> super::Result<()> {
        if let Some(old_tail) = self.snake.move_forward() {
            self.erase_snake_node(old_tail)?;
            self.render()?;
        }
        Ok(())
    }

    fn erase_snake_node(&self, node: SnakeNode) -> super::Result<()> {
        let mut screen = self.screen.lock()?;
        let Position { x, y } = node.position;
        screen.cursor_mut().move_to(x, y)?;
        screen.write_str(" ")?;
        Ok(())
    }
}

pub struct Snake {
    direction: Direction,
    nodes: LinkedList<SnakeNode>,
}

impl Snake {
    fn new(head: SnakeNode, len: u16) -> Snake {
        let nodes = LinkedList::from([head]);
        let direction = Direction::Right;
        let mut snake = Self { direction, nodes };
        for _ in 0..len {
            snake.add_node_at_the_front()
        }
        snake
    }

    pub fn head(&self) -> &SnakeNode {
        self.nodes.front().expect("Snake need at least one node.")
    }

    pub fn tail(&self) -> &SnakeNode {
        self.nodes.back().expect("Snake need at least one node.")
    }

    pub fn eat(&mut self) {
        self.add_node_at_the_back()
    }

    pub fn change_direction(&mut self, direction: Direction) -> bool {
        let head_direction = self.head().direction;
        let has_change = match direction {
            Direction::Up => head_direction != Direction::Down,
            Direction::Down => head_direction != Direction::Up,
            Direction::Left => head_direction != Direction::Right,
            Direction::Right => head_direction != Direction::Left,
        };
        if has_change {
            self.direction = direction;
        }
        has_change
    }

    fn move_forward(&mut self) -> Option<SnakeNode> {
        self.add_node_at_the_front();
        self.nodes.pop_back()
    }

    pub fn is_biting_itself(&self) -> bool {
        let mut p_map = HashSet::new();
        for SnakeNode { position, .. } in &self.nodes {
            if !p_map.insert(position) {
                return true;
            }
        }
        false
    }

    pub fn get_next_position(&self) -> Position {
        let mut position = self.head().position;
        match self.direction {
            Direction::Up => position.y -= 1,
            Direction::Down => position.y += 1,
            Direction::Left => position.x -= 1,
            Direction::Right => position.x += 1,
        }
        position
    }

    fn add_node_at_the_front(&mut self) {
        self.nodes
            .push_front(SnakeNode::new(self.get_next_position(), self.direction));
    }

    fn add_node_at_the_back(&mut self) {
        let mut position = self.tail().position;
        let direction = self.tail().direction;
        match direction {
            Direction::Up => position.y += 1,
            Direction::Down => position.y -= 1,
            Direction::Left => position.x += 1,
            Direction::Right => position.x -= 1,
        }
        self.nodes.push_back(SnakeNode::new(position, direction))
    }
}

#[derive(Accessors)]
#[accessors(get_copy)]
pub struct SnakeNode {
    position: Position,
    direction: Direction,
}

impl SnakeNode {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Drop for SnakeComponent {
    fn drop(&mut self) {
        let _ = self.erase();
    }
}
