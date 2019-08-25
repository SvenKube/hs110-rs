#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bytes;

use bytes::{BufMut, BytesMut};
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use byteorder::{BigEndian, ByteOrder};
use serde_json::json;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, Debug)]
struct Sysinfo {
    system: serde_json::Value,
}

struct SmarPlug {}

impl SmarPlug {
    pub fn get_sysinfo(&self) -> Option<Sysinfo> {
        None
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

fn decrypt(cipher: &mut [u8]) -> String {
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

fn send_message<'a, T>(msg: String) -> Option<T>
where T: serde::Deserialize<'a>
{
    let mut stream = match TcpStream::connect("192.168.178.97:9999") {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("Error creating TcpStream: {}", error);
            return None
        },
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

    let decrypted = decrypt(&mut data);


    let json: T = serde_json::from_str(decrypted.as_str()).unwrap();


    Some(json)
}

fn main() {
    let _plug_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 178, 97));

    // let msg = String::from("{\"system\":{\"set_relay_state\":{\"state\":0}}}");
    let msg = json!({
        "system": {
            "get_sysinfo":{}
        }
    });



    let response = match send_message::<serde_json::Value>(msg.to_string()) {
        Some(data) => data,
        None => {
            eprintln!("Could not send message");
            return;
        },
    };

    println!("{:#?}", response);

    println!("{}", response["system"]["get_sysinfo"]["deviceId"]);

    /*


    stream.write_all(&result)?;
    let mut response = vec![0; 4];
    stream.read(&mut response)?;

    let length = u32::from_be_bytes(*pop(&response));

    println!("{:#?}", length);

    let mut res = vec![0; length as usize - 7];
    stream.read(&mut res);

    println!("{:#?}",str::from_utf8(&res).unwrap());
    */
}
