use std::net::{
    TcpListener,
    TcpStream
};

use std::io::prelude::*;
use std::io::Result;

const IP_ADDR: &str = "127.0.0.1:8080";

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut reading: [u8; 32] = [0; 32];

    stream.read(&mut reading)?;
    dbg!(reading);

    Ok(())
}

fn start_server() -> Result<()> {
    let listener = TcpListener::bind(IP_ADDR)?;
    
    println!("listening on {IP_ADDR}");

    for stream in listener.incoming() {
        let _ = handle_client(stream?);
    }

    Ok(())
}

fn start_stream() -> Result<()> {
    let mut stream = TcpStream::connect(IP_ADDR)?;
   
    println!("connected, sending data");

    let buf: &[u8] = "hello there".as_bytes();
    let _ = stream.write(&buf);

    Ok(())
}

fn main() {
    println!("type 'server' to start server or 'stream' to start stream");

    let mut inp_str = String::new();
    std::io::stdin().read_line(&mut inp_str)
        .expect("Failed to read input");
    
    if inp_str == "server\n".to_string() {
        println!("starting server");
        dbg!(start_server());
    } else {
        println!("starting stream");
        let _ = start_stream();
    }

}
