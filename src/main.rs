use std::net::{
    TcpListener,
    TcpStream
};

use std::io::prelude::*;
use std::io::Result;

const IP_ADDR: &str = "bleps.ch:80";//"71.242.0.12:53";

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
    
        //let mut len: [u8; 2] = [0; 2];
        //let _ = stream.read(&mut len);

        let mut buf = [0; 256];
        let _ = stream.read(&mut buf);
        // dbg!(buf);

        /*
            response format
            2 bytes - message length
            2 bytes - Transaction ID
            2 bytes - flags
                1st bit 1 if message is response
                last 4 bits 0 if no errors
            2 bytes - Questions
            2 bytes - Answer RRs
            2 bytes - Authority RRs
            2 bytes - Additional RRs
            Queries
                Name - URL
                Type - 2 bytes, 0x0001 = host address
                Class - 2 bytes, 0x0001 = IN
            Answers (x Answer RRs)
                2 bytes - c0 0c
                2 bytes - Type A
                2 bytes - Class IN
                4 bytes - TTL
                2 bytes - data length
                Address
        */
        
        let msg_len = (buf[0] << 4) + buf[1];
        println!("Message Length: {msg_len}");
        
        let trans_id = (buf[2] << 4) + buf[3];
        println!("Transaction ID: {:#06x}", trans_id);

        let flags = (buf[4] << 4) + buf[5];
        println!("Flags: {:#b}", flags);

        let questions = (buf[6] << 4) + buf[7];
        println!("Questions: {questions}");

        let answers = (buf[8] << 4) + buf[9];
        println!("Answers: {answers}");

        /*
        for i in range(questions):
            while buf[i] != '0b00':
                url += buf[i]
            // read two-byte type
            // read two-byte class
        */
        
        // ignore authority & additional RRs
        let mut buf = buf[14..].to_vec();
        
        for i in 0..questions {
            let mut j = 0;
            let mut name = String::new();
            
            while buf[j] != 0 {
                let mut c: char;
                
                if buf[j] == 2 { 
                    c = '.'; 
                } else {
                    c = std::char::from_u32(buf[j] as u32).unwrap();
                }
                
                name.push(c);
                j += 1;
            }
            
            // while loop finishes on 0 char
            let q_type = (buf[j+1] << 4) + buf[j+2];
            let q_class = (buf[j+3] << 4) + buf[j+4];

            println!("Query:");
            println!("\turl:\t{name}");
            println!("\ttype:\t{:#04x}", q_type);
            println!("\tclass:\t{:#04x}", q_class);
        }

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
