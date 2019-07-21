extern crate bytes;


use std::io::{Write, Read};
use std::io;
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use bytes::{BytesMut, BufMut};

fn encrypt(msg : String) -> BytesMut {
    let mut result = BytesMut::with_capacity(msg.len() + 4);
    result.put_u32_be(msg.len() as u32);
    let mut key :u32 = 171;

    for character in msg.chars() { 
        let a :u32 = key ^ (character as u32);
        key = a;
        result.put(a as u8);
    }

    return result;
}

fn main() -> io::Result<()> {
    let _plug_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 178, 97));

    // let msg = String::from("{\"system\":{\"set_relay_state\":{\"state\":0}}}");
    let msg = String::from("{\"system\":{\"get_sysinfo\":{}}}");

    let result = encrypt(msg);


	let mut stream = TcpStream::connect("192.168.178.97:9999")?;



    stream.write_all(&result)?;
    let mut response = vec![];    
	stream.read_to_end(&mut response)?;
	
    println!("{:#?}", response);

	return Ok(());
}
