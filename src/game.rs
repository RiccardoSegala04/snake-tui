use crate::util::Vec2;

use rand::Rng;

pub const FIELD_LINES: usize = 16;
pub const FIELD_COLS: usize = 25;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Snake(u16),
    Apple,
    Free,
}

type Field = [[Tile; FIELD_COLS]; FIELD_LINES];

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Starting,
    Started,
    Ended,
}

pub struct Game {
    head: Vec2,
    length: u16,
    direction: Direction,
    field: Field,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            head: Vec2::new(0, 0),
            length: 1,
            direction: Direction::Right,
            field: [[Tile::Free; FIELD_COLS]; FIELD_LINES],
            state: GameState::Starting,
        }
    }

    pub fn points(&self) -> u16 {
        self.length - 1
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn move_to(&mut self, d: Direction) {
        if self.state != GameState::Started {
            self.length = 1;
            self.create_apple();
            self.create_snake();
            self.state = GameState::Started;
            self.direction = d;
        } else if self.direction.opposite() != d {
            self.direction = d;
        }
    }

    pub fn step(&mut self) {
        for row in self.field.iter_mut() {
            for elem in row.iter_mut() {
                *elem = update_element(*elem);
            }
        }

        if let GameState::Started = self.state {
            self.move_head();

            let ended = self.handle_collisions();

            if !ended {
                self.set_head(Tile::Snake(self.length));
            }
        }
    }

    pub fn head(&self) -> Tile {
        self.field[self.head.y as usize][self.head.x as usize]
    }

    pub fn set_head(&mut self, e: Tile) {
        self.field[self.head.y as usize][self.head.x as usize] = e;
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    fn handle_collisions(&mut self) -> bool {
        match self.head() {
            Tile::Snake(_) => {
                self.clear();
                self.state = GameState::Ended;
            }
            Tile::Apple => {
                self.length += 1;
                self.create_apple();
            }
            _ => {}
        };

        self.state == GameState::Ended
    }

    fn move_head(&mut self) {
        self.head.x = (self.head.x as i32
            + match self.direction {
                Direction::Left => -1i32,
                Direction::Right => 1i32,
                _ => 0,
            })
        .rem_euclid(self.field[0].len() as i32) as u16;

        self.head.y = (self.head.y as i32
            + match self.direction {
                Direction::Up => -1i32,
                Direction::Down => 1i32,
                _ => 0,
            })
        .rem_euclid(self.field.len() as i32) as u16;
    }

    fn generate_position(&self) -> (usize, usize) {
        let mut rng = rand::rng();
        loop {
            let line: usize = rng.random_range(0..FIELD_LINES);
            let col: usize = rng.random_range(0..FIELD_COLS);

            if let Tile::Free = self.field[line][col] {
                return (line as usize, col as usize);
            }
        }
    }

    fn create_snake(&mut self) {
        let (line, col) = self.generate_position();
        self.field[line][col] = Tile::Snake(1);
    }

    fn create_apple(&mut self) {
        let (line, col) = self.generate_position();
        self.field[line][col] = Tile::Apple;
    }

    fn clear(&mut self) {
        for row in self.field.iter_mut() {
            for elem in row.iter_mut() {
                *elem = Tile::Free;
            }
        }
    }
}

fn update_element(e: Tile) -> Tile {
    match e {
        Tile::Snake(index) => {
            if index == 1 {
                Tile::Free
            } else {
                Tile::Snake(index - 1)
            }
        }
        _ => e,
    }
}
