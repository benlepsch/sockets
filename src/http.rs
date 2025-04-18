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

    pub fn header(&self, key: String, val: String) -> HttpRequest {
        if !self.headers.contains_key(key) {
            self.headers.insert(key, val);
        } else {
            self.headers.key = val;
        }

        &self
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

        for (k, v) in self.headers {
            out_str = format!("{}{}: {}\r\n", out_str, k, v);
        }
        
        out_str = format!("{}\r\n", out_str);
        out_str.into_bytes()
    }
}
