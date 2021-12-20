use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

use simple_logger::SimpleLogger;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    log::info!("Connection from {}", stream.peer_addr()?);
    loop {
        let mut buffer = [0; 4096];
        let nbytes = stream.read(&mut buffer)?;

        if nbytes == 0 {
            return Ok(());
        }

        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        log::trace!("The client sent: {}", s);

        stream.write("Hello client!".as_bytes())?;
        stream.flush()?;
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let listener = TcpListener::bind("0.0.0.0:3306").unwrap();
    // accept connections and process them, spawning a new thread for each one
    log::info!("Server listening on port 3306");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    let result = handle_client(stream);
                    match result {
                        Ok(()) => {
                            log::info!("Connection closed.")
                        }
                        Err(err) => {
                            log::error!("Error: {}", err)
                        }
                    }
                });
            }
            Err(e) => {
                log::error!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
