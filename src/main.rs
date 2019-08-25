#[macro_use]
extern crate serde_derive;
extern crate bytes;
extern crate serde;
extern crate serde_json;

use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, BytesMut};
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::thread;

pub mod types;

use types::*;
use std::time::Duration;
use std::borrow::Borrow;

#[derive(Clone)]
struct SmartPlug {
    ip: String,
}

impl SmartPlug {
    pub fn new(ip: String) -> SmartPlug {
        SmartPlug { ip }
    }

    pub fn get_sysinfo(&self) -> Option<PlugInfo> {
        let message = json!({
            "system": {
                "get_sysinfo":{}
            }

        });

        send_message::<PlugInfo>(message.to_string())
    }

    pub fn turn_on(&self) -> Option<PlugInfo> {
        send_message::<PlugInfo>(json!({
            "system":{
                "set_relay_state": {
                    "state": 1
                }
            }
        }).to_string())
    }

    pub fn turn_off(&self) -> Option<PlugInfo> {
        send_message::<PlugInfo>(json!({
            "system":{
                "set_relay_state": {
                    "state": 0
                }
            }
        }).to_string())
    }

    pub fn get_emeter_realtime(&self) -> Option<PlugInfo> {
        send_message::<PlugInfo>(json!({
            "emeter": {
                "get_realtime": {}
            }
        }).to_string())
    }
}

fn encrypt_message(msg: String) -> BytesMut {
    let mut result = BytesMut::with_capacity(msg.len() + 4);
    result.put_u32_be(msg.len() as u32);
    let mut key: u32 = 171;

    for character in msg.chars() {
        let a: u32 = key ^ (character as u32);
        key = a;
        result.put(a as u8);
    }

    return result;
}

fn decrypt_message(cipher: &mut [u8]) -> String {
    let len = cipher.len();

    let mut key = 0xAB;
    let mut next: u8;

    for i in 0..len {
        next = cipher[i];
        cipher[i] ^= key;
        key = next;
    }

    String::from_utf8_lossy(cipher).into_owned()
}

fn send_message<T>(msg: String) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut stream = match TcpStream::connect("192.168.178.97:9999") {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("Error creating TcpStream: {}", error);
            return None;
        }
    };

    let message = encrypt_message(msg);
    stream.write_all(&message);

    // read the length of the actual data
    let mut resp = vec![0; 4];
    stream.read(&mut resp);

    let len = BigEndian::read_u32(&resp);

    // read the actual data
    let mut data = vec![0; len as usize];
    stream.read(&mut data);

    let decrypted = decrypt_message(&mut data);

    println!("{}", decrypted);

    match serde_json::from_str(decrypted.as_str()) {
        Ok(object) => Some(object),
        Err(_) => None
    }
}

fn main() {
    let plug_ip = String::from("192.168.178.97:9999");
    let plug = SmartPlug::new(plug_ip);

    let p1 = plug.clone();
    let p2 = plug.clone();

    //let sysinfo = plug.get_sysinfo();
    //plug.turn_on();
    //plug.turn_off();

    let child = thread::spawn(move || {
        loop {
            let res = p1.get_emeter_realtime();
            let realtime_stats = res.unwrap()
                .emeter.unwrap()
                .get_realtime.unwrap();

            println!("Current realtime stats: {:#?}", realtime_stats);

            thread::sleep(Duration::from_secs(1));
        }
    });

    let child2 = thread::spawn(move || {
        loop {
            p2.turn_on();
            thread::sleep(Duration::from_secs(20));

            p2.turn_off();
            thread::sleep(Duration::from_secs(20));
        }
    });

    child.join();
    child2.join();
}
