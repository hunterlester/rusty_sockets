
extern crate mio;
use mio::*;

struct WebSocketServer;

impl Handler for WebSocketServer {
    // the handler
    // interface requires us to
    // provide only two things: concrete types for
    // timeouts and messages.
    type Timeout = usize;
    type Message = ();
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = WebSocketServer;
    event_loop.run(&mut handler).unwrap();
}
