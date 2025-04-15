use std::net::{
    TcpListener,
    TcpStream
};

use std::io::prelude::*;
use std::io::Result;

const IP_ADDR: &str = "127.0.0.1:8080";

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("{}", format!("new connection at {}", stream.local_addr().unwrap()));
    
    let mut reading: [u8; 32] = [0; 32];

    stream.read(&mut reading)?;
    
    let to_str = std::str::from_utf8(&reading).unwrap();
    println!("{to_str}"); 

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
   
    println!("connected to {IP_ADDR}");

    println!("Enter a message to send");
    let mut to_send = String::new();
    std::io::stdin().read_line(&mut to_send)
        .expect("Failed to read input");

    while to_send.len() > 32 {
        println!("message is too long, enter another message under 32 characters in length:");
        std::io::stdin().read_line(&mut to_send)
            .expect("Failed to read input");
    }

    let buf: &[u8] = to_send.as_bytes();
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
