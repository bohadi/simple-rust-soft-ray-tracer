extern crate smr;
extern crate time;
use smr::test_render_scene;
use time::precise_time_ns;

fn main() {
    let t0: u64 = precise_time_ns();
    test_render_scene();
    let t1: u64 = precise_time_ns();
    println!("{}ms", ((t1-t0)as f64)/1e6);
}