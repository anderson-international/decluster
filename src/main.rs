#![feature(drain_filter)]
extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

mod canvas;
mod constants;
mod decluster;
mod point;

use canvas::Canvas;

fn main() {
    let point_count = 500;
    let min_distance = 65.0;

    Canvas::new("Decluster".to_string(), point_count, min_distance).show();
}
