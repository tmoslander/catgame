#![no_std]
#![no_main]

use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::DecodedKey;
use pluggable_interrupt_template::LetterMover;
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::clear_screen;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .startup(clear_screen)
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
