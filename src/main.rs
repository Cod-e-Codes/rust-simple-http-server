use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Failed to bind to address");

    println!("Server running on http://127.0.0.1:3000");

    loop {
        let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            match socket.read(&mut buffer).await {
                Ok(_) => {
                    let response = handle_request(&buffer);
                    socket.write_all(response.as_bytes()).await.expect("Failed to write response");
                }
                Err(e) => println!("Failed to read from socket: {}", e),
            }
        });
    }
}

fn handle_request(buffer: &[u8]) -> String {
    let request = String::from_utf8_lossy(buffer);
    println!("Received request:\n{}", request);

    let status_line = "HTTP/1.1 200 OK\r\n";
    let content = "<html><body><h1>Welcome to Rust HTTP Server!</h1></body></html>";

    format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    )
}
