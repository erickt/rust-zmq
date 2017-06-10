#![crate_name = "lbbroker"]

//! load balancing broker
//! clients and workers here are shown in process

extern crate zmq;
extern crate rand;

use zmq::SNDMORE;
//use std::time::;
use std::thread;

//  Basic request-reply client using REQ socket
//  Because s_send and s_recv can't handle 0MQ binary identities, we
//  set a printable text identity to allow routing.
fn client_task(client_nbr: i32) {
    //create context and client socket
    let context = zmq::Context::new();
    let client = context.socket(zmq::REQ).unwrap();

    //set random indentity string and connect
    let identity = format!("Client{}", client_nbr.to_string());
    client.set_identity(identity.as_bytes()).unwrap();
    client
        .connect("ipc://frontend.ipc")
        .expect("failed connecting client");

    //send request, get reply
    client
        .send("HELLO", 0)
        .expect("client failed sending request");
    let reply = client
        .recv_string(0)
        .expect("client failed receiving reply")
        .unwrap();
    println!("Client: {}", reply);
}

fn worker_task(worker_nbr: i32) {
    let context = zmq::Context::new();
    let worker = context.socket(zmq::REQ).unwrap();
    let identity = format!("Worker{}", worker_nbr.to_string());
    worker.set_identity(identity.as_bytes()).unwrap();
    assert!(worker.connect("ipc://backend.ipc").is_ok());

    // Tell the broker we're ready for work
    worker.send("READY", 0).unwrap();

    loop {
        //Read and save all frames until we get an empty frame
        //In this example there is only 1 but there could be more
        let identity = worker
            .recv_string(0)
            .expect("worker failed receiving id")
            .unwrap();
        let empty = worker
            .recv_string(0)
            .expect("worker failed receving empty")
            .unwrap();
        assert!(empty.len() == 0);
        // Get workload from broker, until finished
        let request = worker.recv_string(0).unwrap().unwrap();
        println!("Worker: {}", request);
        worker
            .send(&identity, SNDMORE)
            .expect("worker failed sending identity");
        worker
            .send("", SNDMORE)
            .expect("worker failed sending empty frame");
        worker.send("OK", 0).expect("worker failed sending OK");


    }
}


fn main() {
    let worker_pool_size = 3;
    let client_pool_size = 10;
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::ROUTER).unwrap();
    frontend
        .bind("ipc://frontend.ipc")
        .expect("failed binding frontend");
    backend
        .bind("ipc://backend.ipc")
        .expect("failed binding backend");
    // While this example runs in a single process, that is just to make
    // it easier to start and stop the example. Each thread has its own
    // context and conceptually acts as a separate process.
    let mut client_thread_pool = Vec::new();
    for client_nbr in 0..client_pool_size {
        let child = thread::spawn(move || { client_task(client_nbr); });
        client_thread_pool.push(child);
    }

    let mut worker_thread_pool = Vec::new();
    for worker_nbr in 0..worker_pool_size {
        let child = thread::spawn(move || { worker_task(worker_nbr); });
        worker_thread_pool.push(child);
    }
    //  Here is the main loop for the least-recently-used queue. It has two
    //  sockets; a frontend for clients and a backend for workers. It polls
    //  the backend in all cases, and polls the frontend only when there are
    //  one or more workers ready. This is a neat way to use 0MQ's own queues
    //  to hold messages we're not ready to process yet. When we get a client
    //  reply, we pop the next available worker and send the request to it,
    //  including the originating client identity. When a worker replies,
    //  we requeue that worker and forward the reply to the original
    //  client using the reply envelope.
    let mut client_nbr = client_pool_size;
    let mut worker_queue = Vec::new();
    loop {
        let mut items = [
            backend.as_poll_item(zmq::POLLIN),
            frontend.as_poll_item(zmq::POLLIN),
        ];
        let rc = zmq::poll(
            &mut items[0..if worker_queue.is_empty() { 1 } else { 2 }],
            -1,
        ).unwrap();

        if rc == -1 {
            break;
        }

        if items[0].is_readable() {
            let worker_id = backend
                .recv_string(0)
                .expect("backend failed receiving worker id")
                .unwrap();
            assert!(
                backend
                    .recv_string(0)
                    .expect("backend failed receiving empty")
                    .unwrap() == ""
            );
            assert!(worker_queue.len() < (worker_pool_size as usize));
            worker_queue.push(worker_id);
            let client_id = backend
                .recv_string(0)
                .expect("backend failed receiving client id")
                .unwrap();

            //if client reply send rest to front end
            if client_id != "READY" {
                assert!(
                    backend
                        .recv_string(0)
                        .expect("backend failed receiving second empty")
                        .unwrap() == ""
                );
                let reply = backend
                    .recv_string(0)
                    .expect("backend failed receiving client reply")
                    .unwrap();
                frontend
                    .send(&client_id, SNDMORE)
                    .expect("frontend failed sending client id");
                frontend
                    .send("", SNDMORE)
                    .expect("frontend failed sending empty");
                frontend
                    .send(&reply, 0)
                    .expect("frontend failed sending reply");
                client_nbr -= 1;
                if client_nbr == 0 {
                    break;
                }
            }
        }

        if items[1].is_readable() {
            //  Now get next client request, route to last-used worker
            //  Client request is [identity][empty][request]
            let client_id = frontend
                .recv_string(0)
                .expect("frontend failed receiving client id")
                .unwrap();
            assert!(
                frontend
                    .recv_string(0)
                    .expect("frontend failed receiving empty")
                    .unwrap() == ""
            );
            let request = frontend
                .recv_string(0)
                .expect("frontend failed receiving request")
                .unwrap();

            let worker = worker_queue.pop().unwrap();
            backend
                .send(&worker, SNDMORE)
                .expect("backend failed sending worker_id");
            backend
                .send("", SNDMORE)
                .expect("backend failed sending empty");
            backend
                .send(client_id.as_bytes(), SNDMORE)
                .expect("backend failed sending client_id");
            backend
                .send("", SNDMORE)
                .expect("backend faield sending second empty");
            backend
                .send(&request, 0)
                .expect("backend failed sending request");
        }
    }
}
