stomp-rs
=====
`stomp-rs` aspires to provide a full STOMP 1.2 client implementation for the Rust
programming language. It is in an alpha state and should not be used.


### Example stomp-rs code
```rust
extern crate stomp;
use stomp::frame::Frame;

let session = match stomp::connect("127.0.0.1", 61613) {
  Ok(session)  => session,
  Err(error) => fail!("Could not connect to the server! {}", error)
};

fn on_message(frame: Frame){
  println!("Received a message:\n{}", frame.to_str());
}

session.subscribe("/topic/messages", on_message);
session.listen(); // Loops infinitely, awaiting messages
```

### Example Cargo.toml
```toml
[package]

name = "stomp_test"
version = "0.0.1"

[[bin]]

name = "stomp_test"

git = "https://github.com/zslayton/stomp-rs.git"
```
