// `std` is Rust's standard library: these tools come with Rust.
// Read and Write let us receive bytes from a connection and send bytes back.
use std::io::{Read, Write};
// A listener waits for connections. A stream represents one connected client.
use std::net::{TcpListener, TcpStream};

fn main() {
    // 127.0.0.1 means "this computer"; 8080 is the port we listen on.
    // bind returns a Result: either Ok(listener) or Err(error).
    // expect stops the program with a useful message if startup fails.
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Could not start the server. Is another program using port 8080?");
    println!("Server running at http://127.0.0.1:8080");
    println!("Press Ctrl+C to stop.");

    // Wait for clients and handle them one at a time.
    // `match` chooses what to do for each possible Result value.
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => handle_connection(stream),
            Err(error) => eprintln!("Connection failed: {error}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // `mut` means a value may change. This array has room for 1024 bytes.
    let mut buffer = [0; 1024];
    // &mut temporarily lends the buffer to read so it can fill it with bytes.
    // read returns the number of bytes received. Step 2 will read in a loop
    // because a request can arrive in more than one piece.
    let bytes_read = stream.read(&mut buffer).expect("Failed to read request");
    if bytes_read == 0 {
        return; // The client disconnected without sending a request.
    }
    // Examine only the filled part of the array. lossy replaces invalid text
    // bytes instead of crashing while printing the request.
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request:\n{request}");
    let body = "<h1>Hello from Rust!</h1>";

    // HTTP is the message format browsers understand. A response contains a
    // status line (200 means success), headers, a blank line, and the body.
    // \r\n ends an HTTP line. Two in a row separate headers from the body.
    // format! creates a String and inserts values into {} slots.
    // len counts bytes, which is what Content-Length needs.
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    // write_all sends every byte or reports an error. One write might only
    // send some bytes. as_bytes gives us the bytes that make up the text.
    stream
        .write_all(response.as_bytes())
        .expect("Failed to send response");
    // Rust closes the connection when stream goes out of scope here.
}
