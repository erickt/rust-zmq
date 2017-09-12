extern crate zmq;
#[macro_use]
extern crate quickcheck;

#[macro_use]
mod common;

use zmq::Message;
use quickcheck::{Gen, Arbitrary};

// A pair which contains two non-equal values
#[derive(Clone, Debug)]
struct NePair<T>(T, T);

impl<T> Arbitrary for NePair<T> where T: Eq + Arbitrary {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let v1 = T::arbitrary(g);
        let v2 = (0..).map(|_| T::arbitrary(g)).filter(|v| *v != v1).next().unwrap();
        NePair(v1, v2)
    }
}

quickcheck! {
    fn msg_cmp_eq(input: Vec<u8>) -> bool {
        Message::from_slice(&input) == Message::from_slice(&input)
    }

    fn msg_cmp_ne(input: NePair<Vec<u8>>) -> bool {
        Message::from_slice(&input.0) != Message::from_slice(&input.1)
    }

    fn msg_vec_roundtrip(input: Vec<u8>) -> bool {
        let original = Message::from_slice(&input.clone());
        Message::from(input) == original
    }
}
