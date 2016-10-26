extern crate zmq;

#[macro_use]
extern crate quickcheck;

use zmq::{z85_encode,z85_decode};
use quickcheck::{Gen,Arbitrary};

#[test]
fn test_z85() {
    let test_str = "/AB8cGJ*-$lEbr2=TW$Q?i7:)<?G/4zr-hjppA3d";
    let decoded = z85_decode(test_str).unwrap();
    let encoded = z85_encode(&decoded).unwrap();
    assert_eq!(test_str, encoded);
}

// Valid input for z85 encoding (i.e. a slice of bytes with its length
// being a multiple of 4)
#[derive(Clone,Debug)]
struct Input(Vec<u8>);

impl Arbitrary for Input {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let len = g.gen_range(0, 256) * 4;
        Input(g.gen_iter::<u8>().take(len).collect())
    }
}

quickcheck! {
    fn z85_roundtrip(input: Input) -> bool {
        let encoded = z85_encode(&input.0).unwrap();
        let decoded = z85_decode(&encoded).unwrap();
        return input.0 == decoded;
    }
}
