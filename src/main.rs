#![no_std]
#![no_main]

mod cat_core;
use catgame::MainGame;
use cat_core::CatGame;
use lazy_static::lazy_static;
use spin::Mutex;
use pluggable_interrupt_os::HandlerTable;
use pc_keyboard::DecodedKey;


lazy_static!{
    static ref GAME: Mutex<MainGame> = Mutex::new(CatGame::new());
}

fn tick(){
    Cat::tick(&mut GAME.lock());
}

fn key(key: DecodedKey){
    GAME.lock().key(key);
}

#[no_mangle]
pub extern "C" fn _start() -> !{
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .start()
}