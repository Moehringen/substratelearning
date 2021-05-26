use std::thread; // thread library which can handle muliptle tasks
use std::net::{TcpListener, TcpStream, Shutdown};//A TCP socket server related library
use std::io::{Read, Write};// io related library 

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap(); // creating a TcpListener by binding it to a local address with port 3333
    println!("Server listening on port 3333");// print information
    for stream in listener.incoming() {//by iterating over the Incoming iterator returned by incoming,the listnener listens for incoming TCP connections.
        //type Item = Result<TcpStream>;
        match stream { //pattern matching via the match
            Ok(stream) => { // as the TcpListerner.incoming return a Result, here can use match pattern
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {   //spawning a new thread for each one
                    // connection succeeded
                    println!("telnet successful");
                    //stream.write(b"you have been telnet our server successfully").unwrap();
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