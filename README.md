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

Steps 1–4 are complete. Visit `/` or `/about`; other paths return 404. Query
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
