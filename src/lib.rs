#![no_std]
#![no_main]

use cat_core::Cell;
use cat_core::Position;
use cat_core::Status;
use cat_core::CatGame;
use pluggable_interrupt_os::vga_buffer::*;
mod cat_core;

const GameHeight: usize = BUFFER_HEIGHT-2;
const HeaderSpace: usize = BUFFER_HEIGHT - GameHeight;

const Dog_Colors : [Color; 2] = [Color::LightGray, Color::Brown];

pub type MainGame = CatGame<BUFFER_WIDTH, GameHeight>;

pub fn tick(game: &mut MainGame){
    if game.countdown_complete(){
        game.update();
        draw(game);
    }
}

fn draw(game: &mut MainGame){
    draw_header(game);
    draw_board(game);
}

fn draw_header(game: &MainGame){
    let header_color = ColorCode::new(Color::White, Color::Black);
    let score_text = "Score: ";
    clear_row(0, Color::Black);
    clear_row(1, Color::Black);
    plot_str(score_text, 0,0,header_color);
    plot_num(game.score() as isize, score_text.len()+ 1, 0, header_color);
}

fn draw_subheader(subheader: &str) {
    plot_str(subheader, 0 , 1, ColorCode::new(Color::LightBlue, Color::Black));
}

fn draw_game_over(game: &MainGame){
    draw_header(game);
    draw_subheader("Game Over. Press S to restart.");
}

fn draw_board(game: &MainGame){
    for p in game.cell_pos_iter(){
        let (row,col) = p.row_col();
        let (c, color) = get_icon_color(game, game.p, &game.cell(p));
        plot(c,col, row + HeaderSpace, color);
    }
}

fn get_icon_color(game: &MainGame, p: Position<BUFFER_WIDTH, GameHeight>, cell: &Cell)-> (char,ColorCode){
    let (icon, foreground) =
    if p == game.dog_at(){
        (match game.status(){
            Status::Over => '%',
            _ => game.cat_icon()
        }, Color::Yellow)
    }else{
        if let Some((d, Dog)) = game.dog_at(){
            (Dog.icon(), Dog_Colors[d])
        }else{
            match cell{
                Cell::Fish => ('f', Color::LightBlue),
                Cell::Empty => (' ', Color::Black),
                Cell::Wall => ('#', Color::Green),
            }
        }
    };
    (icon, ColorCode::new(foreground, Color::Black))
}