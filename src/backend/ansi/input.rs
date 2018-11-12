use std::{
    io::{self, Read},
    sync::mpsc,
    thread,
};

use crate::key::Key;

const ESC: u8 = 27;

pub fn start_input_thread(sender: mpsc::Sender<u8>) {
    thread::spawn(move || {
        let mut input = io::stdin().bytes();

        loop {
            let byte = match input.next().unwrap() {
                Ok(v) => v,
                Err(_) => break,
            };

            match sender.send(byte) {
                Ok(_) => {},
                Err(_) => break,
            }
        }
    });
}

pub fn read_key(receiver: &mpsc::Receiver<u8>) -> Key {
    let byte = receiver.recv().unwrap();

    println!("Reading a key?");

    if byte == ESC {
        // special key??
        Key::Escape
    } else {
        Key::Char(byte.into())
    }
}