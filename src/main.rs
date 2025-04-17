use std::net::{
//    TcpListener,
    TcpStream
};

use std::io::prelude::*;
use std::io::Result;

mod HttpRequest;

const IP_ADDR: &str = "bleps.ch:80";

fn start_stream() -> Result<()> {
    let mut stream = TcpStream::connect(IP_ADDR)?;   
    println!("connected to {IP_ADDR}");

    println!("Building GET request");
    let req = HttpRequest::HttpRequest::new(
        HttpRequest::MethodKind::GET, "bleps.ch".to_string(), None);

    println!("Sending request to server");
    let _ = stream.write(&req.serialize());

    println!("Reading reply");
    /*

    headers = []
    while last two bytes are not "\r\n" {
        current = ''
        while last two bytes are not "\r\n" {
            push onto current
        }
        push to headers
        read out the next two bytes
    }

    */
    let mut headers = Vec::new();
    let mut tmp = [0; 1];
    let mut last: u8 = 0;
    
    stream.read(&mut tmp).expect("someting wrong");
    last = tmp[0];
    stream.read(&mut tmp).expect("somethign wrong");

    // "\r\n" = 0x0d 0x0a
    while last != 13 && tmp[0] != 10 {
        let mut current = String::new();

        while last != 13 && tmp[0] != 10 {
            current.push(last as char);
            last = tmp[0];
            stream.read(&mut tmp).expect("something wrong");
        }

        headers.push(current);

        // 0x0d 0x0a 0x.. 0x.. 0x.. 
        // last tmp 
        stream.read(&mut tmp).expect("something wrong");
        last = tmp[0];
        stream.read(&mut tmp).expect("somethg wrong");
    }
    
    println!("done");
    // dbg!(&headers);

    let http_resp = headers[0].clone();
    headers.remove(0);

    let header_map: std::collections::HashMap<&str, &str> = headers.iter()
        .map(|header| {
            // println!("mapping header: {}", header);
            let mut split = header.split(": ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    // dbg!(&header_map);
    let msg_length = header_map["Content-Length"].parse::<usize>().unwrap();
    let mut buf = vec![0; msg_length];
    stream.read(&mut buf).expect("somethign wrong (last)");

    let to_str = std::str::from_utf8(&buf).unwrap();
    println!("{to_str}");

    Ok(())
}

fn main() {
    println!("connecting to {IP_ADDR}");

    let _ = start_stream();
}
