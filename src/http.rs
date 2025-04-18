use std::collections::HashMap;

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
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpResponse {
    pub fn new(status_code: u32, status_msg: String, protocol: Option<String>) -> HttpResponse {
        HttpResponse {
            protocol: match protocol => {
                Some(p) => p,
                None => { "HTTP/1.1".to_string() },
            },
            status_code: status_code,
            status_msg: status_msg,
            headers: HashMap::new(),
            body: None,
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