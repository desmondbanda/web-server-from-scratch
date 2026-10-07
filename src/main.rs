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
    // HTTP headers are bytes. Reject invalid UTF-8 rather than guessing text.
    let request_text = match std::str::from_utf8(&request) {
        Ok(text) => text,
        Err(_) => {
            return send_response(
                &mut stream,
                "400 Bad Request",
                "<h1>Invalid request text</h1>",
            );
        }
    };
    // The first line looks like: GET /about HTTP/1.1
    // unwrap_or supplies an empty line if there is no first line.
    let first_line = request_text.lines().next().unwrap_or("");
    // A tuple groups the status and page body into one returned value.
    let (status, body) = route_request(first_line);
    send_response(&mut stream, status, body)
}

// This function only chooses a page: it does not need a network connection.
// 'static means these returned strings are available for the whole program.
fn route_request(first_line: &str) -> (&'static str, &'static str) {
    // Collect the words so we can check their count before indexing the list.
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() != 3 {
        return ("400 Bad Request", "<h1>Expected: GET /path HTTP/1.1</h1>");
    }
    let method = parts[0];
    let target = parts[1];
    let version = parts[2];
    if !target.starts_with('/') || (version != "HTTP/1.1" && version != "HTTP/1.0") {
        return ("400 Bad Request", "<h1>Invalid request line</h1>");
    }
    // This learning server implements GET only. We do not read request bodies.
    if method != "GET" {
        return (
            "405 Method Not Allowed",
            "<h1>Use GET to request a page</h1>",
        );
    }
    // A query such as /about?from=home should still open the about page.
    let path = target.split('?').next().unwrap_or(target);
    // include_str! embeds a file as text when Rust compiles the program.
    // Paths here are relative to this Rust file, not the terminal directory.
    // Re-run cargo run after editing HTML so the embedded pages are rebuilt.
    // Only these named files are served; URL paths never become disk paths.
    match path {
        "/" => ("200 OK", include_str!("../pages/index.html")),
        "/about" => ("200 OK", include_str!("../pages/about.html")),
        _ => ("404 Not Found", include_str!("../pages/404.html")),
    }
}

// &str borrows some text instead of taking ownership of a String.
// &mut TcpStream lets this helper write to the caller's connection.
fn send_response(stream: &mut TcpStream, status: &str, body: &str) -> io::Result<()> {
    // HTTP is the message format browsers understand. A response contains a
    // status line (200 means success), headers, a blank line, and the body.
    // \r\n ends an HTTP line. Two in a row separate headers from the body.
    // format! creates a String and inserts values into {} slots.
    // len counts bytes, which is what Content-Length needs.
    // 405 responses tell clients which method is supported.
    let allow_header = if status == "405 Method Not Allowed" {
        "Allow: GET\r\n"
    } else {
        ""
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n{allow_header}\r\n{}",
        body.len(),
        body
    );
    // write_all sends every byte or reports an error. One write might only
    // send some bytes. as_bytes gives us the bytes that make up the text.
    stream.write_all(response.as_bytes())
    // This last expression has no semicolon: its Result is the return value.
    // The caller owns stream and closes it when handle_connection finishes.
}
