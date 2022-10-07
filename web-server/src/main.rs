use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, sleep},
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, file_name) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 ok", "hello.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 ok", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
