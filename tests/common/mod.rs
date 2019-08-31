#![allow(dead_code)]

pub extern crate timebomb;

use std::sync::Once;

static LOGGER_INIT: Once = Once::new();

#[macro_export]
macro_rules! test {
    ($name:ident, $block:block) => {
        #[test]
        fn $name() {
            $crate::common::ensure_env_logger_initialized();
            $crate::common::timebomb::timeout_ms(|| $block, 10000);
        }
    };
}

#[macro_export]
macro_rules! test_capability {
    ($name:ident, $capability:literal, $block:block) => {
        #[test]
        fn $name() {
            if zmq::has($capability).unwrap() {
                $crate::common::ensure_env_logger_initialized();
                $crate::common::timebomb::timeout_ms(|| $block, 10000);
            }
        }
    };
}

pub fn ensure_env_logger_initialized() {
    LOGGER_INIT.call_once(env_logger::init);
}
