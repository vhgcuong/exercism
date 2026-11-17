use std::ops::{Add, Sub};
pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T>
    where T: Default + Copy + PartialEq + PartialOrd + Add<Output=T> + Sub<Output=T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|x| *x == T::default()) {
            return None;
        }

        let mut sorted_sides = sides;
        sorted_sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sorted_sides[2] - (sorted_sides[1] + sorted_sides[0]) > T::default() {
            return None;
        }

        Some(Triangle(sides[0], sides[1], sides[2]))
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2 && self.2 != self.0
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.0 == self.2
    }
}
