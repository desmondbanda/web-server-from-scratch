// `std` is Rust's standard library: these tools come with Rust.
// Read and Write let us receive bytes from a connection and send bytes back.
use std::io::{self, Read, Write};
// A listener waits for connections. A stream represents one connected client.
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

// Constants give meaningful names to limits used throughout the server.
const MAX_HEADER_BYTES: usize = 8192;

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
            Ok(stream) => {
                // Log a client error, then continue serving the next client.
                if let Err(error) = handle_connection(stream) {
                    eprintln!("Could not handle connection: {error}");
                }
            }
            Err(error) => eprintln!("Connection failed: {error}"),
        }
    }
}

// Result<()> means success has no extra value (()), or we return an I/O error.
fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    // Some supplies a timeout value. A slow client must not block us forever.
    // `?` returns early with an error if an operation fails; otherwise we continue.
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    // Vec is a growable list. Unlike the fixed buffer, it can hold many chunks.
    let mut request = Vec::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(()); // A disconnected client needs no response.
        }
        // Examine each byte so the limit applies only to headers, not a body
        // that happens to arrive in the same chunk. We do not use request bodies.
        for byte in &buffer[..bytes_read] {
            request.push(*byte); // * copies the byte from its borrowed location.
            if request.ends_with(b"\r\n\r\n") {
                break;
            }
            if request.len() >= MAX_HEADER_BYTES {
                return send_response(
                    &mut stream,
                    "431 Request Header Fields Too Large",
                    "<h1>Request headers are too large</h1>",
                );
            }
        }
        // TCP delivers bytes in pieces, not whole HTTP messages. Keep reading
        // until the blank line that ends the headers has actually arrived.
        if request.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    let body = "<h1>Hello from Rust!</h1>";

    send_response(&mut stream, "200 OK", body)
}

// &str borrows some text instead of taking ownership of a String.
// &mut TcpStream lets this helper write to the caller's connection.
fn send_response(stream: &mut TcpStream, status: &str, body: &str) -> io::Result<()> {
    // HTTP is the message format browsers understand. A response contains a
    // status line (200 means success), headers, a blank line, and the body.
    // \r\n ends an HTTP line. Two in a row separate headers from the body.
    // format! creates a String and inserts values into {} slots.
    // len counts bytes, which is what Content-Length needs.
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    // write_all sends every byte or reports an error. One write might only
    // send some bytes. as_bytes gives us the bytes that make up the text.
    stream.write_all(response.as_bytes())
    // This last expression has no semicolon: its Result is the return value.
    // The caller owns stream and closes it when handle_connection finishes.
}
