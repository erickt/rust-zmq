#![allow(dead_code)]

pub extern crate timebomb;

use std::sync::{Once, ONCE_INIT};

static LOGGER_INIT: Once = ONCE_INIT;

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

pub fn ensure_env_logger_initialized() {
    LOGGER_INIT.call_once(env_logger::init);
}
