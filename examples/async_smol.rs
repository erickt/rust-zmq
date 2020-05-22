use std::io;
use std::ops::Deref;

use smol::{Async, Task};
use zmq::{Context, PollEvents, Socket, SocketType, DONTWAIT, POLLIN, POLLOUT};

async fn with_event<T>(
    sock: &Async<Socket>,
    events: PollEvents,
    mut f: impl FnMut(&Socket) -> zmq::Result<T>,
) -> io::Result<T> {
    // watch the read readiness of fd
    sock.read_with(|inner| {
        // check the actual events
        if inner.get_events()?.contains(events) {
            Ok(f(inner)?)
        } else {
            Err(io::ErrorKind::WouldBlock.into())
        }
    })
    .await
}

async fn with_read<T>(
    sock: &Async<Socket>,
    f: impl FnMut(&Socket) -> zmq::Result<T>,
) -> io::Result<T> {
    with_event(sock, POLLIN, f).await
}

async fn with_write<T>(
    sock: &Async<Socket>,
    f: impl FnMut(&Socket) -> zmq::Result<T>,
) -> io::Result<T> {
    with_event(sock, POLLOUT, f).await
}

async fn run_client(client: Async<Socket>, endpoint: &str) -> io::Result<()> {
    client.get_ref().connect(endpoint)?;
    let mut i = 0_u64;
    loop {
        with_write(&client, |sock| {
            sock.send(format!("hello world {}", i).as_bytes(), DONTWAIT)
        })
        .await?;
        let msg = with_read(&client, |sock| sock.recv_msg(DONTWAIT)).await?;
        println!("{}", msg.as_str().unwrap());
        i += 1;
    }
}

async fn run_server(server: Async<Socket>, endpoint: &str) -> io::Result<()> {
    server.get_ref().bind(endpoint)?;
    loop {
        let msg = with_read(&server, |sock| sock.recv_msg(DONTWAIT)).await?;
        with_write(&server, |sock| sock.send(msg.deref(), DONTWAIT)).await?;
    }
}

async fn a_main() -> io::Result<()> {
    let ctx = Context::new();
    let client = Async::new(ctx.socket(SocketType::REQ).unwrap()).unwrap();
    let server = Async::new(ctx.socket(SocketType::REP).unwrap()).unwrap();
    let endpoint = "ipc:///tmp/test_zmq";
    let client_t = Task::local(run_client(client, endpoint));
    let server_t = Task::local(run_server(server, endpoint));
    client_t.await?;
    server_t.cancel().await.unwrap_or(Ok(()))
}

fn main() {
    smol::run(a_main()).unwrap();
}
