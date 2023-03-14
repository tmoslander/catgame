use pluggable_interrupt_os::vga_buffer::{BUFFER_HEIGHT, BUFFER_WIDTH};
use core::ops::{Add,Sub};
use pc_keyboard::{DecodedKey, KeyCode};

const UPDATE_FREQUENCY: usize = 3;
//#[allow(incomplete_features)]
//use core::prelude::rust_2024::derive;

pub struct CatGame<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Cell; WIDTH]; HEIGHT],
    cat: Cat<WIDTH,HEIGHT>,
    dogs: [Dog<WIDTH,HEIGHT>; 2],
    status: Status,
    fish_eaten: u32,
    countdown: usize,
    last_key: Option<Dir>
}

enum Dir{
    N,S,E,W
}

impl Dir{

    fn icon(&self) -> char {
        match self {
            Dir::N | Dir::S | Dir::E | Dir::W => 'C'
        }
    }

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

impl From<char> for Dir {
    fn from(icon: char) -> Self {
        match icon {
            'C' => Dir::S,
            'C' => Dir::N,
            'C' => Dir::W,
            'C' => Dir::E,
            _ => panic!("Illegal icon: '{}'", icon)
        }
    }
}

pub enum Cell {
    Fish,
    Empty,
    Wall
}

//#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position<const WIDTH: usize, const HEIGHT: usize>{
    col: i16, row: i16
}

impl <const WIDTH: usize, const HEIGHT: usize> Position<WIDTH, HEIGHT>{
    pub fn is_legal(&self) -> bool{
        0<= self.col && self.col < WIDTH as i16 && 0<= self.row && self.row < HEIGHT as i16
    }
    pub fn row_col(&self) -> (usize, usize){
        (self.row as usize, self.col as usize)
    }
}


//#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cat<const WIDTH: usize, const HEIGHT: usize>{
    pos: Position<WIDTH, HEIGHT>, dir: Dir, open: bool
}

impl <const WIDTH:usize, const HEIGHT: usize> Cat<WIDTH,HEIGHT> {
    fn new(pos: Position<WIDTH, HEIGHT>, icon:char) -> Self{
        Cat {pos, dir: Dir::from(icon), open: true}
    }

    fn tick(&mut self) {
        self.open = !self.open;
    }
    fn icon(&self) -> char{
        match self.dir {
            Dir::N | Dir::S | Dir::E | Dir::W => 'C'
        }
    }
}


//#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Dog<const WIDTH: usize, const HEIGHT: usize>{
    pos: Position<WIDTH,HEIGHT>, dir: Dir, active: bool
}
// need to make it so that the dogs move around in the game

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

    pub fn icon(&self) -> char {
        'D'
    }

}

//#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status{
    Normal,
    Over
}



impl<const WIDTH:usize, const HEIGHT: usize> catgame<WIDTH, HEIGHT>{
    pub fn new() -> Self{
        let mut game = CatGame{
            cat : Cat::new(Position {col: 0, row: 0}),
            dogs : [Dog{pos: Position {col:0, row: 0}, active: true}; 2],
            fish_eaten: 0,
            countdown: UPDATE_FREQUENCY, last_key: None, status: Status::Normal
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
        self.status = Status::Normal;
        self.fish_eaten = 0;
        self.last_key = None;
    }

    pub fn score(&self) -> u32{
        self.fish_eaten
    }

    fn translate_icon(&mut self, dog : &mut usize, row: usize, col: usize, icon:char){
        match icon{
              //match # to wall, f to fish, D to dog and c to cat, _ to panic
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

pub fn dog_at(&self) -> Postion<WIDTH,HEIGHT>{
    self.dogs.iter().enumerate().find(|(_,dog)| dog.pos == p)
}

pub fn cat_icon(&self) -> char{
    self.cat.icon()
}


pub fn update(&mut self){
    self.last_key = None;
    self.cat.tick();
    self.empower_tick();
    self.update_dogs();
}



fn update_dogs(&mut self){
    for d in 0..self..dogs.len(){
        // make it so that the dogs randomly wander (Ferrer question)
        self.resolve_dog_col(d);
        }
}

fn resolve_dog_col(&mut self, d:usize){
    if self.dogs[d].pos == self.cat.pos && self.dogs[d].active{
        match self.status{
            Status::Normal => self.status = Status.Over,
            Status::Empowered => {
                self.fish_eaten += 100;
                self.dogs[d].squash();
            }
            Status::Over => {}
        }
    }
}

}

// need to match keys to directions being moved, maybe need dir after all without the icon matching.
