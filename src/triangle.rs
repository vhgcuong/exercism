pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&item| item == 0) ||
            sides[0] + sides[1] < sides[2] ||
            sides[0] + sides[2] < sides[1] ||
            sides[1] + sides[2] < sides[0]
        {
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
