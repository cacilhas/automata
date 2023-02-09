pub mod vec2;
pub mod sprite;

use raylib::prelude::Color;
use rand::{Rng, prelude::ThreadRng};

use super::utils::{config, interact::n2};
use vec2::{Vec2, vec2};
use sprite::Sprite;


macro_rules! new_boid {
    ($id:expr) => {{
        let mut rng = ThreadRng::default();
        Boid {
            id: $id,
            loc: vec2![f32;
                x: rng.gen_range(0_f32..config::window::WIDTH),
                y: rng.gen_range(0_f32..config::window::HEIGHT)
            ],
            force: vec2![f32; x: 0, y: 0],
            w: 0,
            color: Color::WHITE,
        }
    }};
}


#[derive(Clone)]
pub struct Boid {
    id: usize,
    pub loc: Vec2<f32>,
    pub w: i32,
    force: Vec2<f32>,
    color: Color,
}

impl Sprite for Boid {

    fn sprite_move(&mut self) {
        let mut rng = ThreadRng::default();
        if self.force.x.abs() < 1.0 {
            self.force.x = rng.gen_range(2_f32..4_f32);
        }
        if self.force.y.abs() < 1.0 {
            self.force.y = rng.gen_range(2_f32..4_f32);
        }
        self.loc.x += self.force.x;
        self.loc.y += self.force.y;
    }

    #[inline]
    fn location(&self) -> &Vec2<f32> {
        &self.loc
    }

    #[inline]
    fn color(&self) -> &Color {
        &self.color
    }
}

impl Boid {
    #[inline]
    pub fn reset_forces(&mut self) {
        self.force = vec2![f32; x: 0, y: 0];
    }

    pub fn recalculate_color(&mut self, largest: i32) {
        let w = (self.w * 256 / largest).min(255) as u8;
        self.color = Color {r: 255_u8 - w, g: 0_u8, b: w, a: 255_u8}
    }

    pub fn recalculate_forces(&mut self, other: &Boid) {
        let sqr_dist = self.loc.square_distance_to(&other.loc);
        if sqr_dist < config::SQRVIEW {
            let diff = (other.w - self.w) as f32;
            let angle = self.loc.angle_to(&other.loc);
            let mut force = -0.5;
            if diff != 0.0 {
                force = (config::FORCE * diff * diff.abs() / sqr_dist).min(10.0).max(-10.0);
            }
            self.force.x += force * angle.cos();
            self.force.y += force * angle.sin();
        }
    }
}


#[inline]
pub fn create_boids(count: usize) -> Vec<Boid> {
    (0..count).map(|id| new_boid!(id)).collect::<Vec<Boid>>()
}

pub fn update_boids(boids: &mut Vec<Boid>) {
    n2::foreach! {
        <Boid> in boids;
        setup(boid) {
            boid.w = 0;
        }
        step(a, b) {
            if a.loc.square_distance_to(&b.loc) < config::SQRFRND {
                a.w += 1;
            }
        }
    };
    n2::foreach! {
        <Boid> in boids;
        setup(boid) {
            boid.reset_forces();
        }
        step(a, b) {
            if a.id != b.id {
                a.recalculate_forces(b);
            }
        }
    };
    for boid in &mut *boids {
        boid.sprite_move();

        if boid.loc.x >= config::window::WIDTH {
            boid.loc.x -= config::window::WIDTH;
        } else if boid.loc.x < 0.0 {
            boid.loc.x += config::window::WIDTH;
        }

        if boid.loc.y >= config::window::HEIGHT {
            boid.loc.y -= config::window::HEIGHT;
        } else if boid.loc.y < 0.0 {
            boid.loc.y += config::window::HEIGHT;
        }
    }
}
