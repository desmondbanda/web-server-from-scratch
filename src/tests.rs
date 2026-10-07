// `super` means the parent module: main.rs in this project.
use super::route_request;

// #[test] tells cargo test to run this function as an automated check.
#[test]
fn home_page_is_available() {
    let (status, body) = route_request("GET / HTTP/1.1");
    // assert_eq! fails the test if these two values are not equal.
    assert_eq!(status, "200 OK");
    assert!(body.contains("Hello from Rust!"));
}

#[test]
fn about_page_ignores_query_string() {
    let (status, body) = route_request("GET /about?from=home HTTP/1.1");
    assert_eq!(status, "200 OK");
    assert!(body.contains("About this server"));
}

#[test]
fn unknown_paths_do_not_expose_files() {
    for path in ["/missing", "/../Cargo.toml", "/pages/index.html"] {
        let request_line = format!("GET {path} HTTP/1.1");
        let (status, body) = route_request(&request_line);
        assert_eq!(status, "404 Not Found");
        assert!(body.contains("Page not found"));
    }
}

#[test]
fn unsupported_methods_are_rejected() {
    for method in ["POST", "PUT", "DELETE", "HEAD"] {
        let (status, _) = route_request(&format!("{method} / HTTP/1.1"));
        // _ means we do not need the second value (the body) in this test.
        assert_eq!(status, "405 Method Not Allowed");
    }
}

#[test]
fn malformed_request_lines_are_rejected() {
    for line in [
        "",
        "GET",
        "GET /",
        "GET / HTTP/1.1 extra",
        "GET nowhere HTTP/1.1",
        "GET / HTTP/9",
    ] {
        let (status, _) = route_request(line);
        assert_eq!(status, "400 Bad Request");
    }
}

#[test]
fn http_10_can_request_a_page() {
    let (status, _) = route_request("GET / HTTP/1.0");
    assert_eq!(status, "200 OK");
}
