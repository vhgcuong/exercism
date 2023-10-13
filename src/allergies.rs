pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

use self::Allergen::*;
const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            allergens: Self::calculate_allergens(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.iter().any(|allergy| allergy == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }

    fn calculate_allergens(score: u32) -> Vec<Allergen> {
        let mut allergens = Vec::<Allergen>::new();

        for flag in 0..=7 {
            if score & 1 << flag != 0 {
                allergens.push(ALLERGENS[flag as usize]);
            }
        }
        allergens
    }
}
