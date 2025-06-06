//! Ensure we catch UB due to writing through a shared reference.
#![allow(invalid_reference_casting)]

use std::cell::UnsafeCell;
use std::mem;

const WRITE_AFTER_CAST: () = unsafe {
    let mut x = 0;
    let ptr = &x as *const i32 as *mut i32;
    *ptr = 0; //~ERROR: immutable
};

const WRITE_AFTER_TRANSMUTE: () = unsafe {
    let mut x = 0;
    let ptr: *mut i32 = mem::transmute(&x);
    *ptr = 0; //~ERROR: immutable
};

// it's okay when there is interior mutability;
const WRITE_INTERIOR_MUT: () = unsafe {
    let x = UnsafeCell::new(0);
    let ptr = &x as *const _ as *mut i32;
    *ptr = 0;
};

fn main() {}
