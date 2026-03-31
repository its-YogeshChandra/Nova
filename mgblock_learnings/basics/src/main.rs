//add the things that you learned in mblock
use std::sync::{RwLock, RwLockWriteGuard};

//first about the rwlock
fn learn_rwlock() {
    let value = "value";
    let lock = RwLock::new(value);
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();

    //print the values of r1 and r2
    println!("the r1 val is : {}", r1);
    println!("the r2 val is : {}", r2);
}

fn write_rwlock() {
    println!("function called");
    let value = "value";
    let lock = RwLock::new(value);
    let mut w1 = lock.write().unwrap();
    *w1 = "new value";

    // let mut w2 = lock.write().unwrap();
    // *w2 = "fake value";
    //

    //let read_val = lock.read().unwrap();

    println!("the w1 val is : {}", w1);
    //println!("the w2 val is : {}", w2);
    //    println!("the read val is dropped {}", read_val)
}

//info macro

//what cfg does

#[cfg(target_os = "windows")]
fn os() {
    println!("the os is : macos ")
}

#[cfg(target_os = "macos")]
fn os() {
    println!("the os is : macos ")
}

//what debug does

fn main() {
    println!("Hello, world!");
    learn_rwlock();
    write_rwlock();
    os();
}
