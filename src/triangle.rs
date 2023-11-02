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
        todo!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        todo!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        todo!("Determine if the Triangle is isosceles.");
    }
}
