#![no_std]
#![no_main]

use cat_core::Cell;
use cat_core::Position;
use cat_core::Status;
use cat_core::CatGame;
use pluggable_interrupt_os::vga_buffer::clear_row;
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_num, plot_str};
pub mod cat_core;

const GAME_HEIGHT: usize = BUFFER_HEIGHT-2;
const HEADER_SPACE: usize = BUFFER_HEIGHT - GAME_HEIGHT;
 
const DOG_COLORS : [Color; 2] = [Color::LightGray, Color::Brown];

pub type MainGame = CatGame<BUFFER_WIDTH, GAME_HEIGHT>;

pub fn tick(game: &mut MainGame){
    if game.countdown_complete(){
        game.update();
        draw(game);
    }
}

fn draw(game: &MainGame){
    draw_header(game);
    draw_board(game);
}

fn draw_header(game: &MainGame){
    match game.status(){
        Status::Normal => draw_normal_header(game),
        Status::Over => draw_game_over(game)
    }
}

fn draw_subheader(subheader: &str){
    plot_str(subheader, 0, 1, ColorCode::new(Color::Black, Color::Green));
}

fn draw_normal_header(game: &MainGame){
    let header_color = ColorCode::new(Color::White, Color::Green);
    let score_text = "Score: ";
    clear_row(0, Color::Green);
    clear_row(1, Color::Green);
    plot_str(score_text, 0,0,header_color);
    plot_num(game.score() as isize, score_text.len()+ 1, 0, header_color);
}

fn draw_game_over(game: &MainGame){
    draw_subheader("Game Over. Press S to restart.");
}

fn draw_board(game: &MainGame){
    for p in game.cell_pos_iter(){
        let (row,col) = p.row_col();
        let (c, color) = get_icon_color(game, p, &game.cell(p));
        plot(c,col, row + HEADER_SPACE, color);
    }
}

fn get_icon_color(game: &MainGame, p: Position<BUFFER_WIDTH, GAME_HEIGHT>, cell: &Cell)-> (char,ColorCode){
    let (icon, foreground) =
    if p == game.cat_at(){
        (match game.status(){
            Status::Over => '%',
            _ => 'C'
        }, Color::Yellow)
    }else{
        if let Some((d, Dog)) = game.dog_at(p){
            ('D', DOG_COLORS[d])
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