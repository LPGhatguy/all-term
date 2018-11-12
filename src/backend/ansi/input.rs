use std::{
    collections::VecDeque,
    io::{self, Read},
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
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

pub fn read_key(receiver: &mpsc::Receiver<u8>, output: &mut VecDeque<Key>) {
    let byte = receiver.recv().unwrap();

    if byte == ESC {
        match receiver.recv_timeout(Duration::from_millis(20)) {
            Ok(second_byte) => {
                if second_byte == b'[' {
                    match receiver.recv_timeout(Duration::from_millis(20)) {
                        Ok(third_byte) => {
                            match third_byte {
                                b'A' => output.push_back(Key::Up),
                                b'D' => output.push_back(Key::Left),
                                b'B' => output.push_back(Key::Down),
                                b'C' => output.push_back(Key::Right),
                                b'H' => output.push_back(Key::Home),
                                b'F' => output.push_back(Key::End),
                                _ => {
                                    output.push_back(Key::Escape);
                                    output.push_back(Key::Char(second_byte.into()));
                                    output.push_back(Key::Char(third_byte.into()));
                                },
                            }
                        },
                        Err(RecvTimeoutError::Timeout) => {
                            output.push_back(Key::Escape);
                            output.push_back(Key::Char(second_byte.into()));
                        },
                        Err(RecvTimeoutError::Disconnected) => {},
                    }
                } else {
                    output.push_back(Key::Escape);
                    output.push_back(Key::Char(second_byte.into()));
                }
            },
            Err(RecvTimeoutError::Timeout) => {
                output.push_back(Key::Escape);
            },
            Err(RecvTimeoutError::Disconnected) => {},
        }
    } else {
        output.push_back(Key::Char(byte.into()));
    }
}