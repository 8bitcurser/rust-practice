// this provides functionality that allows one to read and write from a stream
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use web_server::ThreadPool;

fn main() {
    // bind works like new it returns a TcpListener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // incoming returns a seq of TcpStreams
    // a stream is an open connection between client and server
    // a connection is the full req / res process
    for stream in listener.incoming() {
        // this might fail as what we are iterating over are connection
        // attempts and not connections fully established
        let stream = stream.unwrap();
        println!("Connection established");
        // when stream goes out of scope the connection is closed

        // we assign one thread per connection, the execute method takes a 
        // clousure
        pool.execute(|| {handle_connection(stream)});
    }
}

// the parameter is mutable given that TcpStream keeps track the data given
// this might change as the stream might ask for more data than we asked and 
// keep it for the next time we read.
fn handle_connection(mut stream: TcpStream) {
    // 512 byte buffer on the stack to hold the data
    let mut buffer = [0; 512];
    let contents = fs::read_to_string("hello.html").unwrap();

    // read the bytes from the stream and place them into the buffer
    stream.read(&mut buffer).unwrap();
    // we are reading bytes then we need to create a byte string
    let get = b"GET / HTTP/1.1\r\n";
    // if its a get request to something else than localhost:7878
    // display the HTML if not return 404
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    // plus the content of the response
    let response = format!("{} {}", status_line, contents);
    // turn response into bytes, panic if it fails the write takes a &[u8]
    // thats why it gets turned into bytes
    stream.write(response.as_bytes()).unwrap();
    // flush waits and prevent the program from moving forward until all the 
    // bytes have been written to the connection.
    // turn the buffer into string, if it finds an invalid  utf8 it will
    stream.flush().unwrap();
}
