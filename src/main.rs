use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn parse_request(request: String) -> (String, String) {
    todo!()
}

fn handle_request(cmd: String, body: String) {
    todo!()
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut request = String::new();
    if buf_reader.read_line(&mut request).is_err() {
        return;
    }

    let (cmd, body) = parse_request(request);
    let result = handle_request(cmd, body);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
