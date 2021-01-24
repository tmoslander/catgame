#![no_std]
#![no_main]

use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::DecodedKey;
use pluggable_interrupt_template::LetterMover;
use pluggable_interrupt_os::HandlerTable;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .start()
}

lazy_static! {
    static ref LETTERS: Mutex<LetterMover> = Mutex::new(LetterMover::new());
}

fn tick() {
    LETTERS.lock().tick();
}

fn key(key: DecodedKey) {
    LETTERS.lock().key(key);
}