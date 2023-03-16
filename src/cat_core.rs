use core::ops::{Add,Sub};
use pc_keyboard::{DecodedKey, KeyCode};

const UPDATE_FREQUENCY: usize = 3;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CatGame<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Cell; WIDTH]; HEIGHT],
    cat: Cat<WIDTH,HEIGHT>,
    dogs: [Dog<WIDTH,HEIGHT>; 2],
    status: Status,
    fish_eaten: u32,
    countdown: usize,
    last_key: Option<Dir>
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Dir{
    N,S,E,W
}

impl Dir{

    fn reverse(&self) -> Dir{
        match self{
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E
        }
    }

    fn left(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N
        }
    }
}

#[derive(PartialEq, Clone, Copy, Eq)]
pub enum Cell {
    Fish,
    Empty,
    Wall
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position<const WIDTH: usize, const HEIGHT: usize>{
    col: i16, row: i16
}

impl <const WIDTH: usize, const HEIGHT: usize> Add for Position<WIDTH,HEIGHT> {
    type Output = Position<WIDTH,HEIGHT>;

    fn add(self, rhs: Self) -> Self::Output {
        Position {col: self.col + rhs.col, row: self.row + rhs.row}
    }
}

impl <const WIDTH: usize, const HEIGHT: usize> Sub for Position<WIDTH,HEIGHT> {
    type Output = Position<WIDTH,HEIGHT>;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {col: self.col - rhs.col, row: self.row - rhs.row}
    }
}


impl <const WIDTH: usize, const HEIGHT: usize> Position<WIDTH, HEIGHT>{
    pub fn is_legal(&self) -> bool{
        (0 <= self.col && self.col < WIDTH as i16) && 0 <= self.row && self.row < HEIGHT as i16
    }
    pub fn row_col(&self) -> (usize, usize){
        (self.row as usize, self.col as usize)
    }
    pub fn neighbor(&self, d: Dir) -> Position<WIDTH,HEIGHT> {
        match d {
            Dir::N => Position {row: self.row - 1, col: self.col},
            Dir::S => Position {row: self.row + 1, col: self.col},
            Dir::E => Position {row: self.row,     col: self.col + 1},
            Dir::W => Position {row: self.row,     col: self.col - 1}
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cat<const WIDTH: usize, const HEIGHT: usize>{
    pos: Position<WIDTH, HEIGHT>, dir: Dir
}

impl <const WIDTH:usize, const HEIGHT: usize> Cat<WIDTH,HEIGHT> {
    fn new(pos: Position<WIDTH, HEIGHT>) -> Self{
        Cat {pos, dir: Dir::N}
    }

    fn tick(&mut self) {
    }
}


#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Dog<const WIDTH: usize, const HEIGHT: usize>{
    pos: Position<WIDTH,HEIGHT>, dir: Dir, active: bool
}
impl <const WIDTH: usize, const HEIGHT: usize> Dog<WIDTH,HEIGHT> {
    fn on_my_left(&self, other: Position<WIDTH,HEIGHT>) -> bool {
        let offset = self.pos - other;
        match self.dir {
            Dir::N => offset.col > 0,
            Dir::S => offset.col < 0,
            Dir::E => offset.row > 0,
            Dir::W => offset.row < 0
        }
    }

    fn ahead_or_behind(&self, other: Position<WIDTH,HEIGHT>) -> bool {
        let offset = self.pos - other;
        match self.dir {
            Dir::N | Dir::S => offset.col == 0,
            Dir::E | Dir::W => offset.row == 0
        }
    }

    fn on_my_right(&self, other: Position<WIDTH,HEIGHT>) -> bool {
        !self.on_my_left(other) && !self.ahead_or_behind(other)
    }

    fn go(&mut self, ahead: Cell, left: Cell, right: Cell, cat_pos: Position<WIDTH,HEIGHT>) {
        if self.active {
            if left == Cell::Wall && ahead == Cell::Wall && right == Cell::Wall {
                self.dir = self.dir.reverse();
            } else if left != Cell::Wall && (self.on_my_left(cat_pos) || ahead == Cell::Wall) {
                self.dir = self.dir.left();
            } else if right != Cell::Wall && (self.on_my_right(cat_pos) || ahead == Cell::Wall) {
                self.dir = self.dir.right();
            }
            self.pos = self.pos.neighbor(self.dir);
        }
    }

}

#[derive (Copy, Clone, Eq, PartialEq)]
pub enum Status{
    Normal,
    Over
}



impl<const WIDTH:usize, const HEIGHT: usize> CatGame<WIDTH, HEIGHT>{
    pub fn new() -> Self{
        let mut game = CatGame{
            cells: [[Cell::Empty; WIDTH]; HEIGHT],
            cat : Cat::new(Position{col: 0, row: 0}),
            dogs : [Dog{pos: Position {col:0, row: 0}, active: true, dir: Dir::E}; 2],
            fish_eaten: 0,
            countdown: UPDATE_FREQUENCY, last_key: None, 
            status: Status::Normal
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        let mut dog = 0;
        for (row, row_chars) in START.split('\n').enumerate(){
            for (col, icon) in row_chars.trim().chars().enumerate(){
                self.translate_icon(&mut dog, row, col, icon);
            }
        }
        assert_eq!(dog, 2);
        self.status = Status::Normal; //fails horribly when changed to Status::Over here for some reason, the correct screen isn't drawn at all
        self.fish_eaten = 0;
        self.last_key = None;
    }

    pub fn score(&self) -> u32{
        self.fish_eaten
    }

    fn translate_icon(&mut self, dog : &mut usize, row: usize, col: usize, icon:char){
        match icon{
                '#' => self.cells[row][col] = Cell::Wall,
                'f' => self.cells[row][col] = Cell::Fish,
                ' ' => self.cells[row][col] = Cell::Empty,
                'D' => {
                    let dir = DOG_START_DIR[*dog];
                    self.dogs[*dog] = Dog{pos: Position{row:row as i16, col:col as i16}, dir, active:true};
                    *dog += 1
                },
                'C' => {
                    self.cat = Cat::new(Position{row: row as i16, col: col as i16});
                },
                _ => panic!()
                }                
    }

    pub fn cell(&self, p: Position<WIDTH, HEIGHT>)-> Cell{
        self.cells[p.row as usize][p.col as usize]
    }
    pub fn cell_pos_iter(&self) -> RowColIter<WIDTH, HEIGHT>{
        RowColIter{row:0, col:0}
    }
    pub fn cat_at(&self) -> Position<WIDTH, HEIGHT>{
    self.cat.pos
    }
    pub fn dog_at(&self, p: Position<WIDTH,HEIGHT>) -> Option<(usize,&Dog<WIDTH,HEIGHT>)>{
        self.dogs.iter().enumerate().find(|(_, dog)|dog.pos == p)
    }
    pub fn update(&mut self){
        self.resolve_move();
        self.last_key = None;
        self.cat.tick();
        self.update_dogs();
    }
    fn update_dogs(&mut self){
        for d in 0..self.dogs.len() {
            let (ahead, left, right) = self.ahead_left_right(self.dogs[d].pos, self.dogs[d].dir);
            self.resolve_dog_collision(d);
            self.dogs[d].go(ahead, left, right, self.cat_at());
            self.resolve_dog_collision(d);
            }
        }
    fn resolve_dog_collision(&mut self, d:usize){
        if self.dogs[d].pos == self.cat.pos && self.dogs[d].active{
            match self.status(){
                Status::Normal => {self.status = Status::Over},
                Status::Over => {}
            }
        }
    }
    fn ahead_left_right(&self, p: Position<WIDTH,HEIGHT>, dir: Dir) -> (Cell,Cell,Cell) {
        let ahead = self.cell(p.neighbor(dir));
        let left = self.cell(p.neighbor(dir.left()));
        let right = self.cell(p.neighbor(dir.right()));
        (ahead, left, right)
    }

    pub fn countdown_complete(&mut self) -> bool {
        if self.countdown == 0 {
            self.countdown = UPDATE_FREQUENCY;
            true
        } else {
            self.countdown -= 1;
            false
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match self.status {
            Status::Over => {
                match key {
                    DecodedKey::RawKey(KeyCode::S) | DecodedKey::Unicode('s') => self.reset(),
                    _ => {}
                }
            }
            _ => {
                let key = key2dir(key);
                if key.is_some() {
                    self.last_key = key;
                }
            }
        }
    }

    fn resolve_move(&mut self) {
        if let Some(dir) = self.last_key {
            let neighbor = self.cat.pos.neighbor(dir);
            if neighbor.is_legal() {
                let (row, col) = neighbor.row_col();
                if self.cells[row][col] != Cell::Wall {
                    self.move_to(neighbor, dir);
                }
            }
        }
    }

    fn move_to(&mut self, neighbor: Position<WIDTH,HEIGHT>, dir: Dir) {
        self.cat.pos = neighbor;
        self.cat.dir = dir;
        let (row, col) = neighbor.row_col();
        match self.cells[row][col] {
            Cell::Fish => {
                self.fish_eaten += 1;
                self.cells[row][col] = Cell::Empty;
            },
            _ => {}
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }
}
//end of super long catgame impl

fn key2dir(key: DecodedKey) -> Option<Dir> {
    match key {
        DecodedKey::RawKey(k) => match k {
            KeyCode::ArrowUp => Some(Dir::N),
            KeyCode::ArrowDown => Some(Dir::S),
            KeyCode::ArrowLeft => Some(Dir::W),
            KeyCode::ArrowRight => Some(Dir::E),
            _ => None
        }
        DecodedKey::Unicode(c) => match c {
            'w' => Some(Dir::N),
            'a' => Some(Dir::W),
            's' => Some(Dir::S),
            'd' => Some(Dir::E),
            _ => None
        }
    }
}

pub struct RowColIter<const WIDTH: usize, const HEIGHT: usize> {
    row: usize, col: usize
}

impl <const WIDTH: usize, const HEIGHT: usize> Iterator for RowColIter<WIDTH,HEIGHT> {
    type Item = Position<WIDTH,HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < HEIGHT {
            let result = Some(Position {row: self.row as i16, col: self.col as i16});
            self.col += 1;
            if self.col == WIDTH {
                self.col = 0;
                self.row += 1;
            }
            result
        } else {
            None
        }
    }
}

const DOG_START_DIR: [Dir; 2] = [Dir::E, Dir::W];

const START: &'static str =
    "################################################################################
     #                                                                              #
     #         D                                                                    #
     #                                                                              #
     #                                                            f                 #
     #        f                                                                     #
     #                                                                              #
     #                                                                              #
     #                                                                              #
     #                                                                              #
     #                                      C                                       #
     #                        f                                                     #
     #                                                                              #
     #                                                                              #
     #                                                                              #
     #                                                                              #
     #                                                                              #
     #                                                                    f         #
     #                                                                              #
     #                                                                              #
     #        f                                                                     #
     #                                                                      D       #
     ################################################################################";

// need to match keys to directions being moved, maybe need dir after all without the icon matching.
