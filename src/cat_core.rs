use pluggable_interrupt_os::vga_buffer::(BUFFER_HEIGHT, BUFFER_WIDTH)


#![derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position<const WIDTH: usize, const HEIGHT: usize>{
    col: i16, row: i16
}

impl <const WIDTH: usize, const HEIGHT: usize> Position<WIDTH, HEIGHT>{
    pub fn is_legal(&self) -> bool{
        0<= self.col && self.col < WIDTH as i16 && 0<= self.row && self.row < HEIGHT as i16
    }
    pub fn row_col -> (usize, usize){
        (self.row as usize, self.col as usize)
    }
}


#![derive(Debug, Copy, Clone, Eq, PartialEq)]
struct cat<const WIDTH:usize, const HEIGHT: usize>{
    pos: Position<WIDTH, HEIGHT>, open:bool
}

impl <const WIDTH:usize, const HEIGHT: usize>{
    fn new(pos: Position<WIDTH, HEIGHT>, icon:char) -> Self{
        cat{pos, open:true}
    }

    fn tick(&mut self){
        self.open = !self.open;
    }
    fn icon(&self) -> char{
            self.icon = "C"
    }
}


#![derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct dog<const WIDTH: usize, const HEIGHT: usize>{
    pos: Position<WIDTH,HEIGHT>, active:bool
}
// need to make it so that the dogs move around in the game

pub fn icon(&self) -> char {
    if self.active{'D'}
    else {'d'}
}

fn squash(&mut self){
    self.active =false;
}

fn revive(&mut self){
    self.active = true;
}

#![derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status{
    Normal,
    Over
}



impl<const WIDTH:usize, const HEIGHT: usize> catgame<WIDTH, HEIGHT>{
    pub fn new() -> Self{
        let mut game = catgame{
            Cat : cat::new(Position {col: 0, row: 0}),
            dogs : [dog{pos: Position {col:0, row: 0}, active: true}; 2],
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
