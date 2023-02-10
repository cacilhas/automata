use std::ops::{Mul, Add, Sub};
use libm::atan2;
use crate::utils::f64fix::FromF64;


#[macro_export]
macro_rules! vec2 {
    ($tp:ty) => {{
        $crate::boid::vec2::Vec2 {x: 0 as $tp, y: 0 as $tp}
    }};

    (x: $x:expr, y: $y:expr) => {{
        $crate::boid::vec2::Vec2 {x: ($x), y: ($y)}
    }};

    ($tp:ty; x: $x:expr, y: $y:expr) => {{
        $crate::boid::vec2::Vec2 {x: ($x) as $tp, y: ($y) as $tp}
    }};
}

pub use vec2;


pub trait Number: Add<Output=Self>+Sub<Output=Self>+Mul<Output=Self>+Into<f64>+FromF64 {}

impl<T> Number for T
where T: Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Into<f64>+FromF64 {}


#[derive(Clone)]
pub struct Vec2<T> where T: Number {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> where T: Number {

    pub fn square_distance_to(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn angle_to(&self, other: &Self) -> T {
        let res = atan2((other.y - self.y).into(), (other.x - self.x).into());
        <T as FromF64>::from_f64(res)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_should_calculate_the_square_distance() {
        assert_eq!(*(&vec2![f32; x: 1, y: 1].square_distance_to(&vec2![f32; x: 4, y: 5])), 25_f32);
    }

    #[test]
    fn it_should_calculate_the_angle_to() {
        assert_eq!(*(&vec2![f32; x: 0, y: 0].angle_to(&vec2![f32; x: 4, y: 0])), 0.0);         //   0.00°
        assert_eq!(*(&vec2![f32; x: 0, y: 0].angle_to(&vec2![f32; x: 4, y: 3])), 0.6435011);   //  36.87°
        assert_eq!(*(&vec2![f32; x: 0, y: 0].angle_to(&vec2![f32; x: 0, y: 3])), 1.570796327); //  90.00°
        assert_eq!(*(&vec2![f32; x: 0, y: 0].angle_to(&vec2![f32; x: -4, y: 0])), 3.14159265); // 180.00°
    }
}
