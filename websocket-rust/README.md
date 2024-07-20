## **What Is This?**

This is a simple Rust workspace containing `websocket_client` and `websocket_server` packages.

## **Build** :hammer:

```bash
cargo build --release
```

## **Run**

1. Start `websocker_server`:

```bash
./target/release/websocket_server
```

2. Start `websocker_client`:

```bash
./target/release/websocket_client
```


## **Verify** :white_check_mark:

By running the commands in **Run** section, you should see something similar to below:

> Terminal running `websocket_server`:

```bash
A client connected with id #0
Received a message from client #0: Text("Hello WebSocket")
Client #0 disconnected.
```

> Terminal running `websocket_server`:

```bash
Connected to the server
Response HTTP code: 101 Switching Protocols
Response contains the following headers:
* connection
* upgrade
* sec-websocket-accept
Received: Hello WebSocket
```

## **References**

-[tungstenite](https://docs.rs/tungstenite/0.23.0/tungstenite/)
-[serde_json](https://docs.rs/serde_json/latest/serde_json/)
-[simple-websockets](https://docs.rs/simple-websockets/latest/simple_websockets/)