mod onesdk_bindings;
use crate::onesdk_bindings::{onesdk_customservicetracer_create_p, onesdk_string};

use std::ffi::{c_void, CStr, CString};
use std::thread::sleep;
use std::time::Duration;

pub fn onesdk_initialize() -> i32 {
    unsafe { onesdk_bindings::onesdk_initialize() }
}

pub fn onesdk_shutdown() -> i32 {
    unsafe { onesdk_bindings::onesdk_shutdown() }
}

pub fn onesdk_customservicetracer_create(service_method: &str, service_name: &str) -> u64 {
    let mut service_name = CString::new(service_name).expect("Failed to create the string");
    let mut service: *const onesdk_string = &onesdk_string {
        data: service_name.as_ptr() as *mut c_void,
        byte_length: service_name.as_bytes().len() as u64,
        ccsid: 1209,
    };

    let mut service_method = CString::new(service_method).expect("Failed to create the string");
    let mut method: *const onesdk_string = &onesdk_string {
        data: service_method.as_ptr() as *mut c_void,
        byte_length: service_method.as_bytes().len() as u64,
        ccsid: 1209,
    };

    unsafe { onesdk_customservicetracer_create_p(method, service) }
}

pub fn onesdk_tracer_start(tracer: u64) {
    unsafe {
        onesdk_bindings::onesdk_tracer_start(tracer);
    }
}
pub fn onesdk_tracer_end(tracer: u64) {
    unsafe {
        onesdk_bindings::onesdk_tracer_end(tracer);
    }
}

fn main() {
    let init_code = onesdk_initialize();
    println!("oneagent_initialize: {}", init_code);

    for i in 0..10 {
        println!("Starting tracer! ");
        let tracer = onesdk_customservicetracer_create("Method", "Service");
        onesdk_tracer_start(tracer);

        sleep(Duration::from_millis(2000));

        println!("Ending tracer! ");
        onesdk_tracer_end(tracer);
    }

    sleep(Duration::from_millis(5000));
    onesdk_shutdown();
}
