use std::time::Duration;
use std::thread;
use gdnative::prelude::godot_gdnative_init;

#[no_mangle]
pub extern fn init_from_rust() {
    godot_gdnative_init!();  // Comment or uncomment this line for debug change.
}

#[no_mangle]
pub extern fn count_from_rust() {
    for step in 0..1000 {
        println!("{}", step);  // Breakpoint in IDE here.
        thread::sleep(Duration::from_millis(1000))
    }
}
