use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

use std::thread;
use crate::database;

pub fn connector(mut stm: TcpStream) {
    database::add_rand()
        .expect("Error trying to add a new planet to the DB");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)
        .expect("Error trying to read the stream");

    let res = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(res.as_bytes())
        .expect("Error trying to write to the stream");
    stream.flush()
        .expect("Error trying to flush the stream");
}

pub fn listener() {
    let listener = TcpListener::bind("127.0.0.1:9999")
        .expect("Error trying to bind the listener");

    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream?;
            connector(stream);
        }
    });
}