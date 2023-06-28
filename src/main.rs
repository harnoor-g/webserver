use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},    
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // todo: handle connecting to port 80 requires administrator privileges (nonadministrators can listen only on ports higher than 1023), so if we tried to connect to port 80 without being an administrator, binding wouldn’t work
        // todo: handle if we ran two instances of our program and so had two programs listening to the same port

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // todo: handle if data isn't valid UTF-8
    // todo: handle if problem reading from stream

    println!("Request: {:#?}", http_request);
}
