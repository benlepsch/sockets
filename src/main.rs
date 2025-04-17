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

    //let buf: &[u8] = to_send.as_bytes();
    // let buf = vec![0x00, 0x1a, 0x10, 0x55, 0x01, 0x00, 0x00, 
    //     0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x62, 0x6c,
    //     0x65, 0x70, 0x73, 0x02, 0x63, 0x68, 0x00, 0x00, 0x01, 0x00, 0x01];

    println!("Building GET request");
    let req = HttpRequest::new(HttpRequest::MethodKind::GET, "bleps.ch", None);

    println!("Sending request to server");
    let _ = stream.write(&req.serialize());

    println!("Reading reply");
    let mut buf = [0; 256];
    stream.read(&mut buf).expect("something has gone wrong in the read function");
    
    dbg!(&buf);


    Ok(())
}

fn main() {
    println!("connecting to {IP_ADDR}");

    let _ = start_stream();
}
