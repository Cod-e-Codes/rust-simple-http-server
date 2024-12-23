# Rust Simple HTTP Server

A lightweight and asynchronous HTTP server written in Rust using the `tokio` runtime. This server listens on `127.0.0.1:3000` and responds with a simple HTML page.

## Features
- **Asynchronous TCP handling**: Utilizes `tokio` for efficient, non-blocking IO operations.
- **Simple request handling**: Parses incoming requests and responds with an HTML page.
- **Concurrent connections**: Handles multiple client connections simultaneously using `tokio::spawn`.

## Requirements
- [Rust](https://www.rust-lang.org/) (version 1.65.0 or later)
- [Tokio](https://tokio.rs/) (runtime for asynchronous programming in Rust)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Cod-e-Codes/rust-simple-http-server.git
   cd rust-simple-http-server
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## Usage
1. Start the server:
   ```bash
   cargo run
   ```
2. Open your browser and navigate to [http://127.0.0.1:3000](http://127.0.0.1:3000).
3. You will see the following response:

   ```html
   <html><body><h1>Welcome to Rust HTTP Server!</h1></body></html>
   ```

## How It Works
1. **Binding to a socket**: The server binds to the IP address `127.0.0.1` on port `3000`.
2. **Listening for connections**: Listens for incoming TCP connections using `tokio::net::TcpListener`.
3. **Handling requests**: Reads incoming data, logs the HTTP request, and sends a fixed HTML response.
4. **Concurrency**: Each connection is handled in a separate asynchronous task (`tokio::spawn`).

## Example Request and Response
### Request:
```
GET / HTTP/1.1
Host: 127.0.0.1:3000
```

### Response:
```
HTTP/1.1 200 OK

<html><body><h1>Welcome to Rust HTTP Server!</h1></body></html>
```

## File Structure
- **`main.rs`**: Contains the server logic, request handling, and response generation.

## Dependencies
- `tokio`: Provides asynchronous runtime and utilities for handling networking tasks.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## Future Enhancements
- Add support for parsing HTTP headers.
- Extend the server to handle different routes.
- Implement a configuration file for customizable settings (e.g., port, IP address).
- Improve error handling and logging.

