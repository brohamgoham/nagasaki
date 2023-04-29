use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::database;
use std::thread;

/// connector function takes a TcpStream as an argument
/// and reads the stream into a buffer
/// then writes a response to the stream
pub fn connector(mut stm: TcpStream) {
    database::add_rand().expect("Error trying to add a new planet to the DB");

    // declare mut buffer of size 1024
    let mut buffer = [0; 1024];
    // reads data from TCP Stream and places it in buffer
    stm.read(&mut buffer)
        .expect("Error trying to read the stream");

    /*
     * `HTTP/1.1 200 OK\r\n\r\n` is used to represent the 
     * end of an HTTP response header 
     * \r\n\r\n is used to separate the header from the body
     * the first \r\n is used to represent a carriage return and line feed chars that
     * indicate new line. the second \r\n signified end of header
     */
    let res = "HTTP/1.1 200 OK\r\n\r\n";

    // write to HTTP response msg to TCP stream
    stm.write(res.as_bytes())
        .expect("Error trying to write to the stream");

    // flush any remaining data in the stream 
    stm.flush().expect("Error trying to flush the stream");
}

pub fn listener() {
    let listener = TcpListener::bind("127.0.0.1:9999").expect("Error trying to bind the listener");

    // spawn a new thread 
    // closure takes a stream as an argument
    // and calls connector function
    thread::spawn(move || {
        // start loop to listen for incoming connections
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            connector(stream);
        }
    });
}
