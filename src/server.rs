use chalk_rs::Chalk;
use std::fs;
use std::io::{Error, Read, Write};
use std::net::TcpListener;
use std::path::Path;
pub struct Server {
    port: i32,
}
impl Server {
    pub fn new(port: i32) -> Server {
        Server { port }
    }
    pub fn start(&self) -> Result<TcpListener, Error> {
        let host = ["127.0.0.1:".to_string(), self.port.to_string()].concat();
        let mut chalk = Chalk::new();
        match TcpListener::bind(&host) {
            Ok(v) => {
                chalk
                    .green()
                    .bold()
                    .println(&format!("Your server is running at http://{}", host));
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }
    pub fn handle_stream(listener: &TcpListener) {
        let mut chalk = Chalk::new();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 512];
            let contents = fs::read_to_string(Path::new("static/index.html")).unwrap();
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
            stream.read(&mut buffer).unwrap();
            chalk.magenta().bold().println(&format!(
                "Request: {}",
                String::from_utf8_lossy(&buffer[..])
            ));
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
