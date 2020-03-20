extern crate env_logger;
extern crate stomp;
use stomp::frame::Frame;
use stomp::header::{Header, SuppressedHeader, ContentType};
use stomp::subscription::AckOrNack::Ack;
use stomp::subscription::AckMode;
use stomp::connection::{HeartBeat, Credentials};
use stomp::session::ReceiptHandler;

fn main() {
  env_logger::init();

  let destination = "/topic/messages";
  let mut message_count: u64 = 0;

  let mut session = match stomp::session("127.0.0.1", 61613)
    .with(Header::new("custom-client-id", "hmspna4"))
    .with(SuppressedHeader("content-length"))
    .with(HeartBeat(5000, 2000))
    .with(Credentials("sullivan", "m1k4d0"))
    .start() {
      Ok(session) => session,
      Err(error)  => panic!("Could not connect to the server: {}", error)
   };

  session.on_error(|frame: &Frame| {
    println!("Something went horribly wrong: {}", frame);
  });

  let _ = session.subscription(destination, |frame: &Frame| {
    message_count += 1;
    println!("Received message #{}:\n{}", message_count, frame);
    Ack
  })
  .with(AckMode::Client)
  .with(Header::new("custom-subscription-header", "lozenge"))
  .with(ReceiptHandler::new(|_: &Frame| println!("Subscribed successfully.")))
  .start();

  let _ = session.message(destination, "Animal").send();
  let _ = session.message(destination, "Vegetable").send();
  let _ = session.message(destination, "Mineral").send();

  let _ = session.message(destination, "Hypoteneuse".as_bytes())
    .with(ContentType("text/plain"))
    .with(Header::new("persistent", "true"))
    .with(ReceiptHandler::new(|_: &Frame| println!("Got a RECEIPT for 'Hypoteneuse'")))
    .send();

  let _ = session.listen(); // Loops infinitely, awaiting messages

  let _ = session.disconnect();
}
