mod engine;
mod logic;

use crate::engine::Engine;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

fn handle_client(mut stream: &mut TcpStream, mut engine: &mut Engine) {
    let mut response = String::new();
    let mut conn = BufReader::new(&mut stream);
    conn.read_line(&mut response).expect("unable to read");

    let server_response = &response[4..];
    println!("Request : {:?}", server_response.to_string());
    let engine_response = engine.process_command(server_response.to_string());
    let server_response = match engine_response {
        Ok(r) => {
            println!("{}", format!("Sending back {r}"));
            format!("{r}\nok")
        }
        Err(e) => {
            println!("{}", format!("Sending back err {e}"));
            format!("err {e}\nok")
        }
    };

    stream
        .write(server_response.as_bytes())
        .expect("Failed to write response to client");
}

fn main() {
    let mut engine = Engine::new();

    let listener = TcpListener::bind("127.0.0.1:8181").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8181");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => loop {
                let sleep_duration = time::Duration::from_millis(200);
                thread::sleep(sleep_duration);
                handle_client(&mut stream, &mut engine);
            },
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
