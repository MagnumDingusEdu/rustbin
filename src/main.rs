use std::process::exit;
use std::sync::mpsc;
use std::thread;

mod utils;
mod paste;
mod tcp_handler;

fn main() {
    // Initialize the communication channel for the various threads.
    let (channel_tx, channel_rx) = mpsc::channel();

    let tx_pipe = channel_tx.clone();

    // spawn a single thread for the listener
    thread::spawn(move || { tcp_handler::listen(tx_pipe) });

    let mut connections = 0;
    loop {
        println!("Connection count : {}", connections);
        match channel_rx.recv() {
            Ok(signal) => {
                match signal {
                    tcp_handler::CommMessage::NewConnection => connections += 1,
                    tcp_handler::CommMessage::CloseConnection => {
                        println!("Connection dropped. Waiting for new connections...");
                        connections -= 1;
                    }
                    tcp_handler::CommMessage::ExitWithError => exit(1),
                }
            }
            Err(e) => {
                println!("Pipe broken - {}", e);
            }
        }
    }
}