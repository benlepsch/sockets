use std::net::{
//    TcpListener,
    TcpStream
};

use std::io::prelude::*;
use std::io::Result;

mod HttpRequest;

const IP_ADDR: &str = "bleps.ch:80";//"71.242.0.12:53";

fn start_stream() -> Result<()> {
    let mut stream = TcpStream::connect(IP_ADDR)?;   
    println!("connected to {IP_ADDR}");

    println!("Building GET request");
    let req = HttpRequest::HttpRequest::new(
        HttpRequest::MethodKind::GET, "bleps.ch".to_string(), None);

    println!("Sending request to server");
    let _ = stream.write(&req.serialize());

    println!("Reading reply");
    let mut buf = Vec::new(); //[0; 256];
    let mut tmp = [0; 1];
    let mut last: u8;
    
    stream.read(&mut tmp).expect("something wrong");
    last = tmp[0];
    stream.read(&mut tmp).expect("somsething wrong");
    
    while tmp[0] != 0 && last != 0 {
        println!("pushing byte {}", last as char); 
        buf.push(last);
        last = tmp[0];
        stream.read(&mut tmp).expect("something wrong");
    }
    
    println!("done");
    // dbg!(&buf);
    let s = match std::str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid utf8 sequence: {}", e),
    };

    println!("{}", &s);


    Ok(())
}

fn main() {
    println!("connecting to {IP_ADDR}");

    let _ = start_stream();
}
