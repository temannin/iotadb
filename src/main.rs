use std::fs::File;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

use argparse::{ArgumentParser, Store, StoreTrue};

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    info!("Connection from {}", stream.peer_addr()?);
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

        trace!("The client sent: {}", s);

        stream.write("Hello client!".as_bytes())?;
        stream.flush()?;
    }
}

fn set_logger(log_path: String) {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(".log").unwrap(),
        ),
    ])
    .unwrap();

    let mut ap = ArgumentParser::new();
    ap.set_description(
        "iotadb. A completely open-source database that prioritizes safety over anything else. Learn more at: https://github.com/temannin/iotadb",
    );
    ap.parse_args_or_exit();
}

fn start_network() {
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

fn main() {
    set_logger();
    start_network();
}
