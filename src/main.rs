#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::io;
use std::io::Read;
use std::fs::File;

fn read_input_event(f: &mut File) -> io::Result<(input_event)> {
    const sz: usize = std::mem::size_of::<input_event>();
    let mut buf = [0; sz];
    f.read_exact(&mut buf)?;
    let ie: input_event = unsafe { std::mem::transmute(buf) };
    Ok(ie)
}

fn dump_input_event(fname: &str) {
    let mut f = File::open(fname).expect("open failed.");
    loop {
        match read_input_event(&mut f).expect("read error.") {
            input_event { type_, code, value, .. } if type_ as u32 == EV_KEY => {
                match code as u32 {
                    KEY_DOWN => {
                        if value != 0 {
                            println!("down")
                        }
                    }
                    KEY_UP => {
                        if value != 0 {
                            println!("up")
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} /dev/input/event4", &args[0]);
        return;
    }
    dump_input_event(&args[1]);
}
