//use quickcheck::{quickcheck, Arbitrary, Gen};
//use rand::Rng;
//use std::iter;

use zmq::{z85_decode, z85_encode, DecodeError, EncodeError};

#[test]
fn test_z85() {
    let test_str = "/AB8cGJ*-$lEbr2=TW$Q?i7:)<?G/4zr-hjppA3d";
    let decoded = z85_decode(test_str).unwrap();
    let encoded = z85_encode(&decoded).unwrap();
    assert_eq!(test_str, encoded);
}

#[test]
fn test_decode_errors() {
    let bad_str = "/AB8";
    match z85_decode(bad_str) {
        Err(DecodeError::BadLength) => (),
        _ => panic!("expected bad length error"),
    }

    let bad_str = "/AB\x008";
    match z85_decode(bad_str) {
        Err(DecodeError::NulError(_)) => (),
        _ => panic!("expected nul error"),
    }

    let bad_bytes = b"\x01\x01\x01\x01\x01";
    match z85_encode(bad_bytes) {
        Err(EncodeError::BadLength) => (),
        _ => panic!("expected bad length error"),
    }
}

/*
// Disabled because quickcheck doesn't expose gen_range and gen anymore

// Valid input for z85 encoding (i.e. a slice of bytes with its length
// being a multiple of 4)
#[derive(Clone, Debug)]
struct Input(Vec<u8>);

impl Arbitrary for Input {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..256) * 4;
        Input(iter::repeat(()).map(|_| g.gen()).take(len).collect())
    }
}

quickcheck! {
    fn z85_roundtrip(input: Input) -> bool {
        let encoded = z85_encode(&input.0).unwrap();
        let decoded = z85_decode(&encoded).unwrap();
        input.0 == decoded
    }
}
*/
