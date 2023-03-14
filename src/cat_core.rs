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

fn empower_tick(&mut self){
    if self.empowered_ticks_left >0{
        self.empowered_ticks_left -=1;
        if self.empowered_ticks == 0{
            self.status = Status.Normal;
            for dog in self.dogs.iter_mut(){
                dog.revive();
            }
        }
    }
}

fn update_dogs(&mut self){
    for d in 0..self..dogs.len(){
        let(ahead,left,right) = self.ahead(self.dogs[d].pos);
        self.resolve_dog_col(d);
        self.dogs[d].go(ahead, self.cat.pos);
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

fn ahead(&self, p: Position<WIDTH,HEIGHT>) -> (Cell){
    let ahead = self.cell(p.neighbor(dir));
    (ahead)
}