use pc_keyboard::{
    layouts::Us104Key, DecodedKey, HandleControl::Ignore, KeyCode, KeyEvent, KeyState, Keyboard,
    ScancodeSet1,
};
use spin::{Lazy, Mutex};
use x86_64::instructions::interrupts::without_interrupts;

use crate::{library::interrupt::STDIN_BUFFER, print};

use super::TEXTWRITER;

pub fn execute(scancode: u8) {
    static KEYBOARD: Lazy<Mutex<Keyboard<Us104Key, ScancodeSet1>>> = Lazy::new(|| {
        let keyboard = pc_keyboard::Keyboard::new(Us104Key, ScancodeSet1, Ignore);
        Mutex::new(keyboard)
    });

    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        // println!("Event tirgger: {:?}", event);
        match event {
            KeyEvent {
                code: KeyCode::Delete,
                state: KeyState::Down,
            } => {}
            KeyEvent {
                code: KeyCode::Backspace,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXTWRITER.get() {
                    without_interrupts(|| writer.lock().cursor_left())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowLeft,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXTWRITER.get() {
                    without_interrupts(|| writer.lock().cursor_left())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowRight,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXTWRITER.get() {
                    without_interrupts(|| writer.lock().cursor_right())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowUp,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXTWRITER.get() {
                    without_interrupts(|| writer.lock().cursor_up())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowDown,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXTWRITER.get() {
                    without_interrupts(|| writer.lock().cursor_down())
                }
            }
            _ => {
                if let Some(key) = keyboard.process_keyevent(event) {
                    // println!("Key: {:?}", key);
                    match key {
                        DecodedKey::Unicode(charactor) => {
                            if charactor.is_ascii() {
                                // print!("{:}", charactor);
                                serial_print!("{:}", charactor);
                                STDIN_BUFFER.lock().push_back(charactor as u8);
                            } else {
                                print!("{:?}", key);
                                serial_print!("{:?}", key);
                            }
                        }
                        DecodedKey::RawKey(key) => {
                            print!("{:?}", key);
                            serial_print!("{:?}", key);
                        }
                    }
                }
            }
        }
    }
}
