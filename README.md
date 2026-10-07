# A web server from scratch in Rust

A learning project using only Rust's standard library. Start with `src/main.rs`:
its comments explain both Rust syntax and HTTP messages.

Install stable Rust (edition 2024 needs Rust 1.85 or newer), then run:

```sh
cargo run
```

Open <http://127.0.0.1:8080>. Stop with Ctrl+C in the terminal.
If startup fails, another program may already be using port 8080.

## Five learning steps

1. Explain TCP connections and a simple HTTP response.
2. Read complete request headers and handle connection errors.
3. Choose a response based on the requested page.
4. Move HTML into separate, readable page files.
5. Test the behavior and document how to explore the project.

Visit `/` or `/about`; other paths return 404. Query
strings do not change the selected page. Unsupported methods return 405 with
an `Allow: GET` header; malformed request lines return 400.

Requests can arrive in several chunks. Header storage is limited to 8 KiB,
and reads/writes have a five-second timeout per operation. Client errors are
logged instead of stopping the server.

## Editing the pages

The home, about, and missing-page HTML live in `pages/index.html`,
`pages/about.html`, and `pages/404.html`. `include_str!` copies these files into
the program at compile time. Stop the server and run `cargo run` again after
editing HTML. You do not need to learn file I/O or install a template library.

## Reading the finished code

All five steps are complete. Read the functions in this order:

1. `main` binds to a local port and waits for connections.
2. `handle_connection` reads bytes until the headers end, then selects a response.
3. `route_request` examines the request line and chooses a page and status.
4. `send_response` writes HTTP headers and the HTML body back to the browser.
5. `src/tests.rs` checks the routing rules with small examples.

A **TCP connection** carries bytes between your browser and the server.
An **HTTP request** gives those bytes meaning. For example:

```text
GET /about HTTP/1.1
Host: 127.0.0.1:8080

```

The real request uses `\r\n` line endings and ends its headers with a blank line.
The response starts with `HTTP/1.1 200 OK`, describes the page with headers,
then includes a blank line and the HTML. `Content-Length` counts bytes, so an
accented letter can count as more than one byte.

## Try it yourself

With `cargo run` running in one terminal, use another terminal:

```sh
curl -i http://127.0.0.1:8080/
curl -i 'http://127.0.0.1:8080/about?from=home'
curl -i http://127.0.0.1:8080/missing
curl -i -X POST http://127.0.0.1:8080/
```

Expect 200, 200, 404, and 405 respectively. `-i` displays response headers.
HEAD also returns 405, with headers only as required for HEAD responses.

Run the checks without starting the server:

```sh
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

Try adding a `/hello` page: create its HTML file, add a match arm in
`route_request`, add a test, then restart with `cargo run`.

## Deliberate limits

This is a learning server bound to your own computer, not a production server.
It handles one connection and one request at a time. It has no HTTPS, concurrent
workers, keep-alive, request-body processing, or complete HTTP validation.
Header values are ignored; request-line validation is intentionally small.
An incomplete request is closed on disconnect or timeout. The five-second
limit applies to each I/O operation, not the total request: a client sending
bytes slowly can still occupy the server. Header storage is capped at 8 KiB.

## Follow the five commits

Use `git log --oneline -5` to see the five learning steps, then
`git show <commit-id>` to read one step's changes and detailed commit message.
Each step builds on the previous one and can compile on its own.
