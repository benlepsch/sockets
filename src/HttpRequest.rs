pub struct HttpRequest {
    method: String,
    url: String,
    protocol: String,
    headers: Vec<String>,
}

impl HttpRequest {
    pub fn new(&self, method: String, url: String, protocol: Option<String>) {
        self.method = method;
        self.url = url;

        match protocol {
            Some(p) => { self.protocol = p; },
            None => { self.protocol = "HTTP/1.1".to_string(); },
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        // to bytes
        /*
            GET / HTTP/1.1\r\n
            Host: bleps.ch\r\n
            <Headers>
        */
        let mut out_str = &self.method + " / " + &self.protocol + "\r\n";
        out_str += "Host: " + &self.url + "\r\n";

        // TODO: add headers

        out_str.into_bytes()
    }
}