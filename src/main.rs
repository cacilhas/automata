mod boid;
mod prelude;
mod utils;

use prelude::*;
use raylib::prelude::*;

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(config::window::WIDTH as i32, config::window::HEIGHT as i32)
        .title("Automata")
        .build();

    let mut boids = boid::create_boids(config::BOIDS);

    while !handle.window_should_close() {
        let mut draw = handle.begin_drawing(&thread);

        boid::update_boids(&mut boids);
        let largest = boids.iter().map(|boid| boid.w).fold(0, |acc, w| acc.max(w));

        draw.clear_background(Color::BLACK);
        for boid in &mut boids {
            boid.recalculate_color(largest);
            boid.draw(&mut draw);
        }

        draw.draw_text(largest.to_string().as_str(), 12, 12, 20, Color::WHITE);
    }
}
