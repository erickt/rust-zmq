Rust Zeromq bindings.

To build, just run `rustc zmq.rc`. rust-zmq's AI is a pretty straight forward
port of the C API into Rust:

```
use zmq;
import zmq::{context, socket, error};

fn main() {
    let ctx = alt zmq::init(1) {
      ok(ctx) { ctx }
      err(e) { fail e.to_str() }
    };

    let socket = alt.ctx.socket(zmq::REQ) {
      ok(socket) { socket }
      err(e) { fail e.to_str() }
    };

    alt socket.connect("tcp://127.0.0.1:1234") {
      ok(()) { }
      err(e) { fail e.to_str() }
    }

    alt socket.send_str("hello world!", 0) {
      ok(()) { }
      err(e) { fail e.to_str() }
    }

    socket.close();
    ctx.close();
}
```
