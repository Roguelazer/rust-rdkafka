//! Utility module to create structures to be used in testing.

extern crate libc;

use bindings as r;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub fn create_message(payload: Option<Vec<u8>>, key: Option<Vec<u8>>, err: r::rd_kafka_resp_err_t, partition: i32, offset: i64) -> *mut r::rd_kafka_message_t {
    unsafe {
        let mut message = libc::malloc(mem::size_of::<r::rd_kafka_message_t>()) as *mut r::rd_kafka_message_t;
        match payload {
            Some(payload) => {
                let boxed_payload = payload.into_boxed_slice();
                (*message).len = boxed_payload.len();
                (*message).payload = Box::into_raw(boxed_payload) as *mut c_void;
            }
            None => {
                (*message).len = 0;
                (*message).payload = ptr::null::<u8>() as *mut c_void;
            }
        }
        match key {
            Some(key) => {
                let boxed_key = key.into_boxed_slice();
                (*message).key_len = boxed_key.len();
                (*message).key = Box::into_raw(boxed_key) as *mut c_void;
            }
            None => {
                (*message).key_len = 0;
                (*message).key = ptr::null::<u8>() as *mut c_void;
            }
        }
        (*message).err = err;
        (*message).partition = partition;
        (*message).offset = offset;
        (*message)._private = ptr::null::<u8>() as *mut c_void;
        (*message).rkt = ptr::null::<u8>() as *mut r::rd_kafka_topic_t;
        message
    }
}

#[cfg(test)]
mod tests {
    use bindings as r;
    use mock;

    #[test]
    fn allocate_and_free_message() {
        let payload = vec![1, 2, 3, 4];
        let key = vec![1, 2, 3];
        let message = mock::create_message(Some(payload), Some(key), r::rd_kafka_resp_err_t::RD_KAFKA_RESP_ERR_NO_ERROR, 12, 345);

        unsafe { r::rd_kafka_message_destroy(message) };
    }
}
