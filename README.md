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

Step 1 is complete. Every address returns the same greeting. This version reads
one chunk and stops on read/write errors; step 2 will improve those parts.
