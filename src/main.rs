use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);
    loop {
        println!("Starting loop");
        let mut buffer = [0; 4096];
        let nbytes = stream.read(&mut buffer)?;

        if nbytes == 0 {
            return Ok(());
        }

        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("The client sent: {}", s);

        stream.write("Hello client!".as_bytes())?;
        stream.flush()?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3306").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3306");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    let f = handle_client(stream);
                    println!("{:?}", f);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    println!("Dropping");
    drop(listener);
}
