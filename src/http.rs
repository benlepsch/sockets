use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;
use std::str::from_utf8;

pub enum MethodKind {
    GET,
    POST,
    PUT,
}

pub struct HttpRequest {
    method: MethodKind,
    url: String,
    protocol: String,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn new(method: MethodKind, url: String, protocol: Option<String>) -> HttpRequest {
        HttpRequest {
            method: method,
            url: url,
            protocol: match protocol {
                Some(p) => p,
                None => "HTTP/1.1".to_string(),
            },
            headers: HashMap::new(),
        }
    }

    // pub fn from(stream: TcpStream) -> HttpRequest {
    //     // todo: this
    // }

    pub fn header(&mut self, key: String, val: String) -> &HttpRequest {
        self.headers.insert(key, val);

        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        // to bytes
        /*
            GET / HTTP/1.1\r\n
            Host: bleps.ch\r\n
            <Headers>
        */
        
        let meth = match self.method {
            MethodKind::GET => "GET",
            MethodKind::POST => "POST",
            MethodKind::PUT => "PUT",
        };
        
        let mut out_str = format!("{} / {}\r\n", meth, &self.protocol);
        out_str = format!("{}Host: {}\r\n", out_str, self.url);

        for (k, v) in &self.headers {
            out_str = format!("{}{}: {}\r\n", out_str, k, v);
        }
        
        out_str = format!("{}\r\n", out_str);
        out_str.into_bytes()
    }
}


pub struct HttpResponse {
    protocol: String,
    status_code: u32,
    status_msg: String,
    headers: HashMap<&str, &str>,
    body: Option<String>,
}

impl HttpResponse {
    pub fn new(status_code: u32, status_msg: String, protocol: Option<String>) -> HttpResponse {
        HttpResponse {
            protocol: match protocol {
                Some(p) => p,
                None => { "HTTP/1.1".to_string() },
            },
            status_code: status_code,
            status_msg: status_msg,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn from(stream: TcpStream) -> HttpResponse {
        let mut headers_vec = Vec::new();
        let mut tmp = [0; 1];
        let mut last: u8;

        stream.read(&mut tmp).expect("Failed to read from stream 1");
        last = tmp[0];
        stream.read(&mut tmp).expect("Failed to read from stream 2");

        // read until \r\n
        while last != b'\r' && tmp[0] != b'\n' {
            let mut current = String::new();

            while last != b'\r' && tmp[0] != b'\n' {
                current.push(last as char);
                last = tmp[0];
                stream.read(&mut tmp).expect("Failed to read from string 3");
            }

            headers_vec.push(current);

            stream.read(&mut tmp).expect("Failed to read from stream 4");
            last = tmp[0];
            stream.read(&mut tmp).expect("Failed to read from stream 5");
        }

        // get response code & message
        // HTTP/1.1 200 OK
        let http_resp = headers_vec[0].clone().split(" ");
        let protocol = http_resp.next().unwrap();
        let status_code = http_resp.next().unwrap();
        let status_msg = http_resp.fold("", |acc, word| format!("{} {}", acc, word));
        

        headers_vec.remove(0);

        let header_map: HashMap<&str, &str> = headers_vec.iter()
            .map(|header| {
                let mut split = header.split(": ");
                (split.next().unwrap(), split.next().unwrap())
            })
            .collect();
        
        let msg_length = header_map["Content-Length"].parse::<usize>().unwrap();
        let mut buf = vec![0; msg_length];
        stream.read(&mut buf).expect("Failed to read body from stream");

        HttpResponse {
            protocol: protocol.to_string(),
            status_code: status_code.parse::<u32>().unwrap(),
            status_msg: status_msg.to_string(),
            headers: header_map,
            body: Some(from_utf8(&buf).unwrap().to_string()),
        }
    }

    pub fn header(&mut self, key: String, val: String) -> &HttpResponse {
        self.headers.insert(key, val);

        self
    }

    pub fn body(&mut self, body: String) -> &HttpResponse {
        self.body = Some(body);
        
        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        // set content-length header
        self.headers.insert("Content-Length", match self.body {
            Some(b) => b.len().to_string(),
            None => "0".to_string(),
        });

        // todo: convert the headers & body to bytes

        let mut out_str = format!("{} {} {}", &self.protocol, self.status_code, &self.status_msg);
        out_str.into_bytes()
    }
}