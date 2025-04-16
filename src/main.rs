use std::net::{
    TcpListener,
    TcpStream
};

use std::io::prelude::*;
use std::io::Result;

const IP_ADDR: &str = "71.242.0.12:53";

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("{}", format!("new connection at {}", stream.local_addr().unwrap()));
    
    loop {
        let mut reading: [u8; 32] = [0; 32];

        let _ = stream.read(&mut reading)?;
        if reading == [0; 32] { break; }
    
        let to_str = std::str::from_utf8(&reading).unwrap();
        //dbg!(&reading);
        println!("{to_str}"); 

        let writing: &[u8] = "message received".as_bytes();
        let _ = stream.write(&writing);
    }

    println!("Connection closed");
    Ok(())
}

fn start_server() -> Result<()> {
    let listener = TcpListener::bind(IP_ADDR)?;
    
    println!("listening on {IP_ADDR}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { let _ = handle_client(stream)?; }
            Err (_e) => { println!("Connection failed :("); }
        }
    }

    Ok(())
}

fn start_stream() -> Result<()> {
    let mut stream = TcpStream::connect(IP_ADDR)?;
   
    println!("connected to {IP_ADDR}");
/*
    println!("Enter a message to send, or `quit` to quit");
    
    //loop {
        let mut to_send = String::new();
        std::io::stdin().read_line(&mut to_send)
            .expect("Failed to read input");

        while to_send.len() > 32 {
            println!("message is too long, enter another message under 32 characters in length:");
            std::io::stdin().read_line(&mut to_send)
                .expect("Failed to read input");
        }
*/
      //  if to_send == "quit\n".to_string() { break; }

        //let buf: &[u8] = to_send.as_bytes();
        let buf = vec![0x00, 0x1a, 0x10, 0x55, 0x01, 0x00, 0x00, 
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x62, 0x6c,
            0x65, 0x70, 0x73, 0x02, 0x63, 0x68, 0x00, 0x00, 0x01, 0x00, 0x01];
        let _ = stream.write(&buf);
    
        let mut len: [u8; 2] = [0; 2];
        let _ = stream.read(&mut len);
        let msg_length = (len[0] << 4) + len[1];
        println!("response length: {msg_length}");

        let mut buf = [0; 256];// vec![0, msg_length];
        let _ = stream.read(&mut buf[..]);
        dbg!(buf);
        //let to_str = std::str::from_utf8(&buf).unwrap();
        //println!("{to_str}");
    //}

    Ok(())
}

fn main() {
    println!("type 'server' to start server or 'stream' to start stream");

    let mut inp_str = String::new();
    std::io::stdin().read_line(&mut inp_str)
        .expect("Failed to read input");
    
    if inp_str == "server\n".to_string() {
        println!("starting server");
        let _ = start_server();
    } else {
        println!("starting stream");
        let _ = start_stream();
    }

}
