use tungstenite::{connect, Message};
use url::Url;
use serde_json;

fn main() {
    // connect to ws server locally
    let (mut socket, _response) = 
    connect(Url::parse("ws://localhost:8765").unwrap()).expect("Can't connect");
    // write a message containing "Hello, Test!" to the server
    socket.write_message(Message::Text("Hello, Test!".into())).unwrap();
    // loop forever, handling parsing each message
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        println!("{:?}", parsed["result"]);
    }
}
