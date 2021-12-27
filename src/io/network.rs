use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Instant;

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn start_network() {
    let listener = TcpListener::bind("0.0.0.0:3306").unwrap();
    // accept connections and process them, spawning a new thread for each one
    info!("Server listening on port 3306");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    let result = handle_client(stream);
                    match result {
                        Ok(()) => {
                            info!("Connection closed.")
                        }
                        Err(err) => {
                            error!("Error: {}", err)
                        }
                    }
                });
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    info!("Connection from {}", stream.peer_addr()?);

    // am optimization so I don't have to keep instantiating a string
    // everytime we go through the loop. Probably too early in the
    // to care about that. But hey why not?
    let mut statement: String;

    // instantiate the dialect.
    let dialect = GenericDialect {};

    loop {
        let mut buffer = [0; 4096];
        let nbytes = stream.read(&mut buffer)?;
        let now = Instant::now();

        if nbytes == 0 {
            return Ok(());
        }

        statement = match String::from_utf8(buffer[..nbytes].to_vec()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let ast = Parser::parse_sql(&dialect, &statement).unwrap();
        info!("AST: {:?}", ast);

        trace!("Query executed in {}ms", now.elapsed().as_millis());
        stream.write("Hello client!".as_bytes())?;
        stream.flush()?;
    }
}
