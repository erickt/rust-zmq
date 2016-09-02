#![crate_name = "fileio3"]

//  File Transfer model #3
//
//  In which the client requests each chunk individually, using
//  command pipelining to give us a credit-based flow control.

extern crate zmq;
extern crate tempfile;
extern crate rand;

use zmq::SNDMORE;
use std::thread;
use std::io::{Seek, SeekFrom, Write, Read, Error};
use rand::Rng;
use tempfile::tempfile;
use std::fs::File;

pub static CHUNK_SIZE: usize = 250000;
pub static CHUNK_SIZE_STR: &'static str = "250000";
pub static PIPELINE: usize = 10;
pub static PIPELINE_HWM: usize = 20;

fn random_string(length: usize) -> String {
  let mut string = String::new();
  let mut rng = rand::thread_rng();
  for char in rng.gen_ascii_chars().take(length) {
    string.push(char);
  }
  string
}

fn client_thread(expected_total: usize) {
  let mut context = zmq::Context::new();
  let mut dealer = context.socket(zmq::DEALER).unwrap();
  let identity: String = (0..10).map(|_| rand::random::<u8>() as char).collect();
  dealer.set_identity(identity.as_bytes()).unwrap();

  assert!(dealer.connect("tcp://localhost:6000").is_ok());

  // Up to this many chunks in transit
  let mut credit = PIPELINE;

  let mut total = 0;       //  Total bytes received
  let mut chunks = 0;      //  Total chunks received
  let mut offset = 0;      //  Offset of next chunk request

  let mut clean_break = false;
  loop {
    while (credit > 0) && !clean_break {
      // Ask for next chunk
      dealer.send_str("fetch", SNDMORE).unwrap();
      dealer.send_str(&offset.to_string(), SNDMORE).unwrap();
      dealer.send_str(CHUNK_SIZE_STR, 0).unwrap();
      offset += CHUNK_SIZE;
      credit -= 1;
    }
    let mut chunk = zmq::Message::new().unwrap();
    dealer.recv(&mut chunk, 0).unwrap();
    let chunk_str = chunk.as_str().unwrap();

    if chunk_str.is_empty() {
      clean_break = true; //  Shutting down, quit
    }
    chunks += 1;
    credit += 1;
    let size = chunk_str.len();
    total += size;
    if size < CHUNK_SIZE {
      clean_break = true;              //  Last chunk received; exit
    }
    if clean_break && (credit == PIPELINE) {
      break; // All requests have completed, we can cleanly break.
    }
  }
  println!("{:?} chunks received, {} bytes\n", chunks, total);
  assert!(expected_total == total);
}

//  The rest of the code is exactly the same as in model 2, except
//  that we set the HWM on the server's ROUTER socket to PIPELINE
//  to act as a sanity check.
//  .skip

//  The server thread waits for a chunk request from a client,
//  reads that chunk and sends it back to the client:

fn server_thread(file: &mut File) -> Result<(), Error> {

  let mut context = zmq::Context::new();
  let mut router = context.socket(zmq::ROUTER).unwrap();
  // We have two parts per message so HWM is PIPELINE * 2
  router.set_sndhwm(PIPELINE_HWM as i32).unwrap();
  assert!(router.bind("tcp://*:6000").is_ok());

  loop {
    // First frame in each message is the sender identity
    let mut identity = zmq::Message::new().unwrap();
    router.recv(&mut identity, 0).unwrap();
    if identity.as_str().unwrap().is_empty() {
      break;              //  Shutting down, quit
    }

    // Second frame is "fetch" command
    let mut command_msg = zmq::Message::new().unwrap();
    router.recv(&mut command_msg, 0).unwrap();
    let command = command_msg.as_str().unwrap();
    assert!(command == "fetch");

    // Third frame is chunk offset in file
    let mut offset_msg = zmq::Message::new().unwrap();
    router.recv(&mut offset_msg, 0).unwrap();
    let offset = offset_msg.as_str().unwrap().parse::<u64>().unwrap();;

    // Fourth frame is maximum chunk size
    let mut chunk_size_msg = zmq::Message::new().unwrap();
    router.recv(&mut chunk_size_msg, 0).unwrap();
    let chunk_size = chunk_size_msg.as_str().unwrap().parse::<usize>().unwrap();;

    // Seek to offset
    file.seek(SeekFrom::Start(offset)).unwrap();
    // Read chunk of data from file
    let mut data = vec![0; chunk_size];
    let size = try!(file.read(&mut data));
    data.truncate(size);
    // Send resulting chunk to client
    router.send_msg(identity, SNDMORE).unwrap();
    router.send(&data, 0).unwrap();
  }
  Ok(())
}

//  The main task starts the client and server threads; it's easier
//  to test this as a single process with threads, than as multiple
//  processes:
fn main() {
  // Start child threads
  thread::spawn(|| {

    // Generate test data in a temp directory
    println!("Generating temporary data...");
    let mut file = tempfile().unwrap();
    // Prepare some random test data of appropriate size
    file.write(random_string(10 * CHUNK_SIZE).as_bytes()).unwrap();

    // Start server thread
    println!("Emitting file content of {:?} bytes", 10 * CHUNK_SIZE);
    server_thread(&mut file).unwrap();
  });
  let client_child = thread::spawn(|| {
    // Start client thread
    client_thread(10 * CHUNK_SIZE);
  });
  // Loop until client tells us it's done
  client_child.join().unwrap();
}
