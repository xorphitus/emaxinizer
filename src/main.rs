extern crate libc;

use self::libc::{c_int, c_char, c_void, useconds_t, wchar_t};
use std::{ptr};

type Window = c_int;
type Xdo = *const c_void;

const CURRENT_WINDOW: c_int = 0;

#[repr(C)]
struct CharcodemapT {
    key: wchar_t, // the letter for this key, like 'a'
    code: KeyCode, // the keycode that this key is on
    symbol: KeySym, // the symbol representing this key
    group: c_int, // the keyboard group that has this key in it
    modmask: c_int, // the modifiers to apply when sending this key
    // if this key need to be bound at runtime because it does not
    // exist in the current keymap, this will be set to 1.
    needs_binding: c_int,
}

#[link(name="xdo")]
extern "C" {
    fn xdo_new(display: *const c_char) -> Xdo;
    fn xdo_free(xdo: Xdo);

    fn xdo_send_keysequence_window_list_do(
        xdo: Xdo,
        window: Window,
        keys: *const CharcodemapT,
        nkeys: c_int,
        pressed: c_int,
        modifier: *const c_int,
        delay: useconds_t,
    ) -> c_int;
}

fn main() {
    unsafe {
        let xdo = xdo_new(ptr::null());
        loop {
            let charcodemap = CharcodemapT{};
            let a = xdo_send_keysequence_window_list_do(xdo, CURRENT_WINDOW, &charcodemap, 1, 1, &1, 1);
            println!("... {}", charcodemap.key);
        }
        xdo_free(xdo);
    }
    println!("hello");
}
