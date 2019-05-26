#[macro_use]
mod common;

use quickcheck::{quickcheck, Arbitrary, Gen};
use zmq::Message;

// A pair which contains two non-equal values
#[derive(Clone, Debug)]
struct NePair<T>(T, T);

impl<T> Arbitrary for NePair<T>
where
    T: Eq + Arbitrary,
{
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let v1 = T::arbitrary(g);
        let v2 = (0..).map(|_| T::arbitrary(g)).find(|v| *v != v1).unwrap();
        NePair(v1, v2)
    }
}

quickcheck! {
    fn msg_cmp_eq(input: Vec<u8>) -> bool {
        Message::from(&input) == Message::from(&input)
    }

    fn msg_cmp_ne(input: NePair<Vec<u8>>) -> bool {
        Message::from(&input.0) != Message::from(&input.1)
    }

    fn msg_vec_roundtrip(input: Vec<u8>) -> bool {
        let original = Message::from(&input.clone());
        Message::from(input) == original
    }
}
