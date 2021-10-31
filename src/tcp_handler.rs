use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::utils;
use crate::paste;


const LISTEN_ADDRESS: &str = "127.0.0.1";
const LISTEN_PORT: u16 = 7777;

const BUFFER_SIZE: usize = 20480; /* in bytes */
const READ_TIMEOUT: u64 = 1000; /* in milliseconds */
const BYTE_LIMIT: i64 = 52_428_800; /* 50MB by default */

pub enum CommMessage {
    NewConnection,
    CloseConnection,
    ExitWithError,
}

pub fn listen(tx_pipe: mpsc::Sender<CommMessage>) {

    // bind the TcpListener and start listening for new connections.
    let listener = match TcpListener::bind(format!("{}:{}", LISTEN_ADDRESS, LISTEN_PORT)) {
        Ok(listener) => {
            println!("Listening for TCP connections on {}:{}", LISTEN_ADDRESS, LISTEN_PORT);
            listener
        }
        Err(e) => {
            eprintln!("Failed to bind on {}:{}. Error : {}", LISTEN_ADDRESS, LISTEN_PORT, e);
            tx_pipe.send(CommMessage::ExitWithError).unwrap();
            return;
        }
    };

    // spawn a new thread for each incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Received a connection! - {}:{}", stream.peer_addr().unwrap().ip(), stream.peer_addr().unwrap().port());
                let txp = tx_pipe.clone();
                thread::spawn(move || {
                    connect_handler(stream, txp);
                });
                tx_pipe.send(CommMessage::NewConnection).unwrap();
            }
            Err(e) => println!("Error! - {}", e)
        }
    }
    drop(listener);
}

fn connect_handler(stream: TcpStream, tx_pipe: mpsc::Sender<CommMessage>) {
    let stream = stream.try_clone().ok();

    if stream.is_none() {
        println!("Failed to connect to stream. Aborting connection...");
        tx_pipe.send(CommMessage::CloseConnection).unwrap();
        return;
    }

    let mut stream = stream.unwrap();
    let mut payload: Vec<u8> = vec![];
    let mut bytes_received: i64 = 0;


    stream.set_read_timeout(Some(Duration::from_millis(READ_TIMEOUT))).unwrap();


    loop {
        let mut buf: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 { break; }

                bytes_received += n as i64;
                if bytes_received > BYTE_LIMIT {
                    stream.write("File limit reached.\n".as_bytes()).ok();
                    break;
                }
                payload.extend(&buf[..n]);
            }
            Err(_) => {
                break;
            }
        }
    }

    let mut errored_out: bool = false;


    if stream
        .write(format!("{} received.\n",
                       utils::format_bytes(bytes_received)).as_bytes())
        .ok()
        .is_none() { errored_out = true; };

    if !errored_out {
        match paste::make_new_paste(payload) {
            Ok(url) => {
                stream
                    .write(format!("Link : {}\r\n", url).as_bytes())
                    .ok();
            }
            Err(e) => {
                stream
                    .write(format!("Failed to create paste. Error : {}", e).as_bytes())
                    .ok();
            }
        };
    }
    stream.shutdown(Shutdown::Both).ok();


    tx_pipe.send(CommMessage::CloseConnection).unwrap();

    if !errored_out {
        println!("{} received.\nLink : yourlink.com", utils::format_bytes(bytes_received));
    } else {
        println!("Connection errored out.")
    }

    println!("Client dropped");
}

