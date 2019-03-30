#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;


use std::sync::atomic::AtomicUsize;

struct HitCount {
    count: AtomicUsize
}

fn main() {
    println!("hello world")
}