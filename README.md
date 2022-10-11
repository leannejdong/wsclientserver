# Websockets client server

The goal of this repo is to set up a simple Websocket client, stream
JSON data from a server, and parsing the data back into Rust via `Tungstenite` and `Serde`.

## Create a basic websocket server in Python
The `main.py` create a server which will read any message, wrap it in a basic JSON object and send it back to the client.

To run this Python code, first, install the websockets library, which we can do by installing globally on our system with
```
python3 -m venv venv
source venv/bin/activate
(venv) $ pip3 install websockets
```

Run the python script under the venv.

## Implement the Websocket Rust Client
* implementing the streaming and de-serialization of JSON formatted data, all in Rust

The `main.rs` sets up a WebSocket client which connects to a local WebServer, sends ``Hello, Test!" and parses the result key from the returned JSON object.

Run and check everything’s working by first running the Python WebSocket server in one terminal:

```
cd testserver
python main.py

```

From a different terminal, run
`cargo run`.