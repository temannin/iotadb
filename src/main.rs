use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut buf: &[u8] = &[74, 0, 0, 1];
    stream.write(buf)?;
    return Ok(());
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
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
